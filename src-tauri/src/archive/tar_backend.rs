use anyhow::{Context, Result, bail};
use std::fs::{self, File};
use std::io::{self, BufReader, Read};
use std::path::{Path, PathBuf};

use super::entry::{ArchiveEntry, format_datetime};
use super::ArchiveFormat;
use crate::progress::ProgressSender;

pub struct TarBackend {
    path: PathBuf,
    format: ArchiveFormat,
}

impl TarBackend {
    pub fn open(path: &Path, format: ArchiveFormat) -> Result<Self> {
        if !path.exists() {
            anyhow::bail!("File not found: {}", path.display());
        }
        Ok(Self {
            path: path.to_path_buf(),
            format,
        })
    }

    fn open_reader(&self) -> Result<Box<dyn Read + Send>> {
        let file = BufReader::new(
            File::open(&self.path)
                .with_context(|| format!("Cannot open {}", self.path.display()))?,
        );

        let reader: Box<dyn Read + Send> = match self.format {
            ArchiveFormat::TarGz => Box::new(flate2::read::GzDecoder::new(file)),
            ArchiveFormat::TarBz2 => Box::new(bzip2::read::BzDecoder::new(file)),
            ArchiveFormat::TarZst => Box::new(zstd::stream::read::Decoder::new(file)?),
            ArchiveFormat::Tar => Box::new(file),
            _ => bail!("Unsupported tar variant"),
        };

        Ok(reader)
    }

    pub fn create(
        path: &Path,
        files: &[PathBuf],
        base_dir: &Path,
        format: ArchiveFormat,
        progress: ProgressSender,
    ) -> Result<()> {
        let file =
            File::create(path).with_context(|| format!("Cannot create {}", path.display()))?;

        let writer: Box<dyn io::Write + Send> = match format {
            ArchiveFormat::TarGz => Box::new(flate2::write::GzEncoder::new(
                file,
                flate2::Compression::default(),
            )),
            ArchiveFormat::TarBz2 => Box::new(bzip2::write::BzEncoder::new(
                file,
                bzip2::Compression::default(),
            )),
            ArchiveFormat::TarZst => {
                Box::new(zstd::stream::write::Encoder::new(file, 3)?.auto_finish())
            }
            ArchiveFormat::Tar => Box::new(file),
            _ => bail!("Unsupported tar variant for creation"),
        };

        let mut tar = tar::Builder::new(writer);
        let total = files.len() as u64;

        for (i, file_path) in files.iter().enumerate() {
            let relative = file_path.strip_prefix(base_dir).unwrap_or(file_path);
            let archive_path = relative.to_string_lossy().replace('\\', "/");

            progress.send_progress(i as u64, total, format!("Adding: {archive_path}"));

            if file_path.is_dir() {
                tar.append_dir_all(&archive_path, file_path)?;
            } else {
                let mut f = File::open(file_path)?;
                tar.append_file(&archive_path, &mut f)?;
            }
        }

        tar.finish()?;
        progress.send_progress(total, total, "Archive created".to_string());
        Ok(())
    }
}

impl super::ArchiveBackend for TarBackend {
    fn list_entries(&self) -> Result<Vec<ArchiveEntry>> {
        let reader = self.open_reader()?;
        let mut archive = tar::Archive::new(reader);
        let mut entries = Vec::new();

        for entry in archive.entries()? {
            let entry = entry?;
            let header = entry.header();
            let path = entry.path()?.to_string_lossy().to_string();
            let is_directory = header.entry_type().is_dir();

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

            let modified = header.mtime().ok().and_then(|ts| {
                chrono::DateTime::from_timestamp(ts as i64, 0).map(|dt| dt.naive_utc())
            });

            let size = header.size().unwrap_or(0);

            entries.push(ArchiveEntry {
                path: path.trim_end_matches('/').to_string(),
                name,
                is_directory,
                compressed_size: size,
                uncompressed_size: size,
                modified: format_datetime(modified),
                compression_method: match self.format {
                    ArchiveFormat::TarGz => "Gzip",
                    ArchiveFormat::TarBz2 => "BZip2",
                    ArchiveFormat::TarZst => "Zstd",
                    _ => "Store",
                }
                .to_string(),
                crc32: None,
                encrypted: false,
            });
        }

        Ok(entries)
    }

    fn extract_all(&self, dest: &Path, progress: ProgressSender) -> Result<()> {
        let reader = self.open_reader()?;
        let mut archive = tar::Archive::new(reader);
        let entries: Vec<_> = archive.entries()?.collect::<Result<_, _>>()?;
        let total = entries.len() as u64;

        let reader = self.open_reader()?;
        let mut archive = tar::Archive::new(reader);

        progress.send_progress(0, total, "Extracting...".to_string());
        archive.unpack(dest)?;
        progress.send_progress(total, total, "Extraction complete".to_string());

        Ok(())
    }

    fn extract_entries(
        &self,
        target_entries: &[String],
        dest: &Path,
        progress: ProgressSender,
    ) -> Result<()> {
        let reader = self.open_reader()?;
        let mut archive = tar::Archive::new(reader);
        let total = target_entries.len() as u64;
        let mut extracted = 0u64;

        for entry in archive.entries()? {
            let mut entry = entry?;
            let path = entry.path()?.to_string_lossy().to_string();
            let clean_path = path.trim_end_matches('/');

            let should_extract = target_entries
                .iter()
                .any(|t| clean_path == t || clean_path.starts_with(&format!("{t}/")));

            if should_extract {
                extracted += 1;
                progress.send_progress(extracted, total, format!("Extracting: {clean_path}"));
                let out_path = dest.join(&path);
                if entry.header().entry_type().is_dir() {
                    fs::create_dir_all(&out_path)?;
                } else {
                    if let Some(parent) = out_path.parent() {
                        fs::create_dir_all(parent)?;
                    }
                    let mut out_file = File::create(&out_path)?;
                    io::copy(&mut entry, &mut out_file)?;
                }
            }
        }

        progress.send_progress(total, total, "Extraction complete".to_string());
        Ok(())
    }

    fn add_files(
        &mut self,
        _files: &[PathBuf],
        _archive_path_prefix: &str,
        _progress: ProgressSender,
    ) -> Result<()> {
        bail!("Adding files to TAR archives is not supported. Create a new archive instead.")
    }

    fn delete_entries(&mut self, _entries: &[String], _progress: ProgressSender) -> Result<()> {
        bail!("Deleting from TAR archives is not supported. Create a new archive instead.")
    }

    fn format(&self) -> ArchiveFormat {
        self.format
    }
}
