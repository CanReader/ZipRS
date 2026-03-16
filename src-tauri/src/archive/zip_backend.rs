use anyhow::{Context, Result, bail};
use std::fs::{self, File};
use std::io;
use std::path::{Path, PathBuf};
use zip::write::SimpleFileOptions;
use zip::{CompressionMethod, ZipArchive, ZipWriter};

use super::entry::{ArchiveEntry, format_datetime};
use super::ArchiveFormat;
use crate::progress::Progress;

pub struct ZipBackend {
    path: PathBuf,
}

impl ZipBackend {
    pub fn open(path: &Path) -> Result<Self> {
        let file = File::open(path).with_context(|| format!("Cannot open {}", path.display()))?;
        let _ = ZipArchive::new(file).with_context(|| "Invalid ZIP archive")?;
        Ok(Self {
            path: path.to_path_buf(),
        })
    }

    fn open_archive(&self) -> Result<ZipArchive<File>> {
        let file = File::open(&self.path)?;
        Ok(ZipArchive::new(file)?)
    }

    pub fn create(
        path: &Path,
        files: &[PathBuf],
        base_dir: &Path,
        progress: &dyn Progress,
    ) -> Result<()> {
        let file =
            File::create(path).with_context(|| format!("Cannot create {}", path.display()))?;
        let mut zip = ZipWriter::new(file);
        let options = SimpleFileOptions::default()
            .compression_method(CompressionMethod::Deflated)
            .compression_level(Some(9));

        let total = files.len() as u64;
        for (i, file_path) in files.iter().enumerate() {
            let relative = file_path.strip_prefix(base_dir).unwrap_or(file_path);
            let archive_path = relative.to_string_lossy().replace('\\', "/");

            progress.send_progress(i as u64, total, format!("Adding: {archive_path}"));

            if file_path.is_dir() {
                zip.add_directory(&archive_path, options)?;
            } else {
                zip.start_file(&archive_path, options)?;
                let mut f = File::open(file_path)?;
                io::copy(&mut f, &mut zip)?;
            }
        }

        zip.finish()?;
        progress.send_progress(total, total, "Archive created".to_string());
        Ok(())
    }

    fn compression_name(method: CompressionMethod) -> String {
        match method {
            CompressionMethod::Stored => "Store",
            CompressionMethod::Deflated => "Deflate",
            CompressionMethod::Bzip2 => "BZip2",
            CompressionMethod::Zstd => "Zstd",
            _ => "Unknown",
        }
        .to_string()
    }

    fn add_directory_recursive(
        writer: &mut ZipWriter<File>,
        dir: &Path,
        archive_prefix: &str,
        options: SimpleFileOptions,
    ) -> Result<()> {
        writer.add_directory(format!("{archive_prefix}/"), options)?;

        for entry in fs::read_dir(dir)? {
            let entry = entry?;
            let path = entry.path();
            let name = entry.file_name().to_string_lossy().to_string();
            let archive_path = format!("{archive_prefix}/{name}");

            if path.is_dir() {
                Self::add_directory_recursive(writer, &path, &archive_path, options)?;
            } else {
                writer.start_file(&archive_path, options)?;
                let mut f = File::open(&path)?;
                io::copy(&mut f, writer)?;
            }
        }

        Ok(())
    }
}

impl super::ArchiveBackend for ZipBackend {
    fn list_entries(&self) -> Result<Vec<ArchiveEntry>> {
        let mut archive = self.open_archive()?;
        let mut entries = Vec::with_capacity(archive.len());

        for i in 0..archive.len() {
            let file = archive.by_index(i)?;
            let path = file.name().to_string();
            let is_directory = file.is_dir();

            let name = if is_directory {
                let trimmed = path.trim_end_matches('/');
                trimmed
                    .rsplit('/')
                    .next()
                    .unwrap_or(trimmed)
                    .to_string()
            } else {
                path.rsplit('/').next().unwrap_or(&path).to_string()
            };

            let modified = file.last_modified().and_then(|dt| {
                chrono::NaiveDate::from_ymd_opt(
                    dt.year() as i32,
                    dt.month() as u32,
                    dt.day() as u32,
                )
                .and_then(|d| {
                    d.and_hms_opt(dt.hour() as u32, dt.minute() as u32, dt.second() as u32)
                })
            });

            entries.push(ArchiveEntry {
                path: path.trim_end_matches('/').to_string(),
                name,
                is_directory,
                compressed_size: file.compressed_size(),
                uncompressed_size: file.size(),
                modified: format_datetime(modified),
                compression_method: Self::compression_name(file.compression()),
                crc32: Some(file.crc32()),
                encrypted: file.encrypted(),
            });
        }

        Ok(entries)
    }

    fn extract_all(&self, dest: &Path, progress: &dyn Progress) -> Result<()> {
        let mut archive = self.open_archive()?;
        let total = archive.len() as u64;

        for i in 0..archive.len() {
            let mut file = archive.by_index(i)?;
            let out_path = dest.join(file.name());

            progress.send_progress(i as u64, total, format!("Extracting: {}", file.name()));

            if file.is_dir() {
                fs::create_dir_all(&out_path)?;
            } else {
                if let Some(parent) = out_path.parent() {
                    fs::create_dir_all(parent)?;
                }
                let mut out_file = File::create(&out_path)?;
                io::copy(&mut file, &mut out_file)?;
            }
        }

        progress.send_progress(total, total, "Extraction complete".to_string());
        Ok(())
    }

    fn extract_entries(
        &self,
        entries: &[String],
        dest: &Path,
        progress: &dyn Progress,
    ) -> Result<()> {
        let mut archive = self.open_archive()?;
        let total = entries.len() as u64;

        for (i, entry_name) in entries.iter().enumerate() {
            progress.send_progress(i as u64, total, format!("Extracting: {entry_name}"));

            let name_with_slash = format!("{entry_name}/");
            let lookup_name = if archive.by_name(entry_name).is_ok() {
                entry_name.as_str()
            } else {
                name_with_slash.as_str()
            };
            let mut file = archive
                .by_name(lookup_name)
                .with_context(|| format!("Entry not found: {entry_name}"))?;

            let out_path = dest.join(file.name());
            if file.is_dir() {
                fs::create_dir_all(&out_path)?;
            } else {
                if let Some(parent) = out_path.parent() {
                    fs::create_dir_all(parent)?;
                }
                let mut out_file = File::create(&out_path)?;
                io::copy(&mut file, &mut out_file)?;
            }
        }

        progress.send_progress(total, total, "Extraction complete".to_string());
        Ok(())
    }

    fn add_files(
        &mut self,
        files: &[PathBuf],
        archive_path_prefix: &str,
        progress: &dyn Progress,
    ) -> Result<()> {
        let src_file = File::open(&self.path)?;
        let mut archive = ZipArchive::new(src_file)?;

        let tmp_path = self.path.with_extension("zip.tmp");
        let tmp_file = File::create(&tmp_path)?;
        let mut writer = ZipWriter::new(tmp_file);

        for i in 0..archive.len() {
            let file = archive.by_index_raw(i)?;
            writer.raw_copy_file(file)?;
        }

        let options = SimpleFileOptions::default()
            .compression_method(CompressionMethod::Deflated)
            .compression_level(Some(9));

        let total = files.len() as u64;
        for (i, file_path) in files.iter().enumerate() {
            let archive_name = if archive_path_prefix.is_empty() {
                file_path
                    .file_name()
                    .unwrap_or_default()
                    .to_string_lossy()
                    .to_string()
            } else {
                format!(
                    "{}/{}",
                    archive_path_prefix.trim_end_matches('/'),
                    file_path.file_name().unwrap_or_default().to_string_lossy()
                )
            };

            progress.send_progress(i as u64, total, format!("Adding: {archive_name}"));

            if file_path.is_dir() {
                Self::add_directory_recursive(&mut writer, file_path, &archive_name, options)?;
            } else {
                writer.start_file(&archive_name, options)?;
                let mut f = File::open(file_path)?;
                io::copy(&mut f, &mut writer)?;
            }
        }

        writer.finish()?;
        fs::rename(&tmp_path, &self.path)?;

        progress.send_progress(total, total, "Files added".to_string());
        Ok(())
    }

    fn delete_entries(&mut self, entries: &[String], progress: &dyn Progress) -> Result<()> {
        let src_file = File::open(&self.path)?;
        let mut archive = ZipArchive::new(src_file)?;

        let tmp_path = self.path.with_extension("zip.tmp");
        let tmp_file = File::create(&tmp_path)?;
        let mut writer = ZipWriter::new(tmp_file);

        let total = archive.len() as u64;
        let mut deleted = 0u64;

        for i in 0..archive.len() {
            let file = archive.by_index_raw(i)?;
            let name = file.name().trim_end_matches('/').to_string();

            let should_delete = entries
                .iter()
                .any(|e| name == *e || name.starts_with(&format!("{e}/")));

            progress.send_progress(i as u64, total, format!("Processing: {name}"));

            if should_delete {
                deleted += 1;
            } else {
                writer.raw_copy_file(file)?;
            }
        }

        if deleted == 0 {
            drop(writer);
            fs::remove_file(&tmp_path)?;
            bail!("No matching entries found to delete");
        }

        writer.finish()?;
        fs::rename(&tmp_path, &self.path)?;

        progress.send_progress(total, total, format!("{deleted} entries deleted"));
        Ok(())
    }

    fn format(&self) -> ArchiveFormat {
        ArchiveFormat::Zip
    }
}
