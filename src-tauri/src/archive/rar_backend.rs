use anyhow::{Result, bail};
use std::fs;
use std::path::{Path, PathBuf};

use super::entry::ArchiveEntry;
use super::ArchiveFormat;
use crate::progress::Progress;

pub struct RarBackend {
    path: PathBuf,
}

impl RarBackend {
    pub fn open(path: &Path) -> Result<Self> {
        if !path.exists() {
            bail!("File not found: {}", path.display());
        }
        // Verify it's a valid RAR archive
        let archive = unrar::Archive::new(path)
            .open_for_listing()
            .map_err(|e| anyhow::anyhow!("Invalid RAR archive: {}", e))?;
        drop(archive);
        Ok(Self {
            path: path.to_path_buf(),
        })
    }

    fn format_file_time(ft: u32) -> Option<String> {
        if ft == 0 {
            return None;
        }
        let second = (ft & 0x1F) * 2;
        let minute = (ft >> 5) & 0x3F;
        let hour = (ft >> 11) & 0x1F;
        let day = (ft >> 16) & 0x1F;
        let month = (ft >> 21) & 0x0F;
        let year = ((ft >> 25) & 0x7F) + 1980;
        Some(format!(
            "{:04}-{:02}-{:02} {:02}:{:02}:{:02}",
            year, month, day, hour, minute, second
        ))
    }
}

impl super::ArchiveBackend for RarBackend {
    fn list_entries(&self) -> Result<Vec<ArchiveEntry>> {
        let archive = unrar::Archive::new(&self.path)
            .open_for_listing()
            .map_err(|e| anyhow::anyhow!("Cannot open RAR: {}", e))?;

        let mut entries = Vec::new();

        for result in archive {
            let header = result.map_err(|e| anyhow::anyhow!("Error reading RAR entry: {}", e))?;

            let path = header.filename.to_string_lossy().replace('\\', "/");
            let is_directory = header.is_directory();

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

            let modified = Self::format_file_time(header.file_time);

            entries.push(ArchiveEntry {
                path: path.trim_end_matches('/').to_string(),
                name,
                is_directory,
                compressed_size: header.unpacked_size, // RAR listing doesn't expose packed size
                uncompressed_size: header.unpacked_size,
                modified,
                compression_method: format!("RAR (m{})", header.method),
                crc32: Some(header.file_crc),
                encrypted: header.is_encrypted(),
            });
        }

        Ok(entries)
    }

    fn extract_all(&self, dest: &Path, progress: &dyn Progress) -> Result<()> {
        fs::create_dir_all(dest)?;

        // Count entries first
        let entries = self.list_entries()?;
        let total = entries.len() as u64;

        let archive = unrar::Archive::new(&self.path)
            .open_for_processing()
            .map_err(|e| anyhow::anyhow!("Cannot open RAR for extraction: {}", e))?;

        let mut current = 0u64;
        let mut archive_state = archive;

        loop {
            let header_result = archive_state
                .read_header()
                .map_err(|e| anyhow::anyhow!("Error reading header: {}", e))?;

            match header_result {
                None => break,
                Some(cursor) => {
                    let entry_name = cursor.entry().filename.to_string_lossy().replace('\\', "/");
                    progress.send_progress(current, total, format!("Extracting: {}", entry_name));

                    archive_state = cursor
                        .extract_with_base(dest)
                        .map_err(|e| anyhow::anyhow!("Failed to extract {}: {}", entry_name, e))?;

                    current += 1;
                }
            }
        }

        progress.send_progress(total, total, "Extraction complete".to_string());
        Ok(())
    }

    fn extract_entries(
        &self,
        target_entries: &[String],
        dest: &Path,
        progress: &dyn Progress,
    ) -> Result<()> {
        fs::create_dir_all(dest)?;

        let total = target_entries.len() as u64;
        let mut extracted = 0u64;

        let archive = unrar::Archive::new(&self.path)
            .open_for_processing()
            .map_err(|e| anyhow::anyhow!("Cannot open RAR for extraction: {}", e))?;

        let mut archive_state = archive;

        loop {
            let header_result = archive_state
                .read_header()
                .map_err(|e| anyhow::anyhow!("Error reading header: {}", e))?;

            match header_result {
                None => break,
                Some(cursor) => {
                    let entry_path = cursor.entry().filename.to_string_lossy().replace('\\', "/");
                    let clean_path = entry_path.trim_end_matches('/');

                    let should_extract = target_entries
                        .iter()
                        .any(|t| clean_path == t || clean_path.starts_with(&format!("{t}/")));

                    if should_extract {
                        extracted += 1;
                        progress.send_progress(
                            extracted,
                            total,
                            format!("Extracting: {}", clean_path),
                        );

                        archive_state = cursor
                            .extract_with_base(dest)
                            .map_err(|e| {
                                anyhow::anyhow!("Failed to extract {}: {}", clean_path, e)
                            })?;
                    } else {
                        archive_state = cursor
                            .skip()
                            .map_err(|e| {
                                anyhow::anyhow!("Failed to skip {}: {}", clean_path, e)
                            })?;
                    }
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
        _progress: &dyn Progress,
    ) -> Result<()> {
        bail!("RAR format is proprietary. Adding files is not supported. Use ZIP instead.")
    }

    fn delete_entries(&mut self, _entries: &[String], _progress: &dyn Progress) -> Result<()> {
        bail!("RAR format is proprietary. Deleting entries is not supported. Use ZIP instead.")
    }

    fn format(&self) -> ArchiveFormat {
        ArchiveFormat::Rar
    }
}
