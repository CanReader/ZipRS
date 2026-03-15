pub mod entry;
pub mod rar_backend;
pub mod tar_backend;
pub mod zip_backend;

use anyhow::{Context, Result, bail};
use serde::Serialize;
use std::path::{Path, PathBuf};

use crate::progress::Progress;
use entry::ArchiveEntry;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub enum ArchiveFormat {
    Zip,
    TarGz,
    TarBz2,
    TarZst,
    Tar,
    Rar,
    Gz,
    Bz2,
    Zst,
}

impl ArchiveFormat {
    pub fn from_path(path: &Path) -> Result<Self> {
        let name = path
            .file_name()
            .and_then(|n| n.to_str())
            .unwrap_or("")
            .to_lowercase();

        if name.ends_with(".tar.gz") || name.ends_with(".tgz") {
            Ok(Self::TarGz)
        } else if name.ends_with(".tar.bz2") || name.ends_with(".tbz2") {
            Ok(Self::TarBz2)
        } else if name.ends_with(".tar.zst") || name.ends_with(".tzst") {
            Ok(Self::TarZst)
        } else if name.ends_with(".tar") {
            Ok(Self::Tar)
        } else if name.ends_with(".zip") || name.ends_with(".jar") || name.ends_with(".war") {
            Ok(Self::Zip)
        } else if name.ends_with(".rar") {
            Ok(Self::Rar)
        } else if name.ends_with(".gz") {
            Ok(Self::Gz)
        } else if name.ends_with(".bz2") {
            Ok(Self::Bz2)
        } else if name.ends_with(".zst") {
            Ok(Self::Zst)
        } else {
            bail!("Unsupported archive format: {name}")
        }
    }

    pub fn from_string(s: &str) -> Result<Self> {
        match s {
            "Zip" => Ok(Self::Zip),
            "TarGz" => Ok(Self::TarGz),
            "TarBz2" => Ok(Self::TarBz2),
            "TarZst" => Ok(Self::TarZst),
            "Tar" => Ok(Self::Tar),
            "Rar" => Ok(Self::Rar),
            _ => bail!("Unknown format: {s}"),
        }
    }

    pub fn display_name(&self) -> &str {
        match self {
            Self::Zip => "ZIP",
            Self::TarGz => "TAR.GZ",
            Self::TarBz2 => "TAR.BZ2",
            Self::TarZst => "TAR.ZST",
            Self::Tar => "TAR",
            Self::Rar => "RAR",
            Self::Gz => "GZ",
            Self::Bz2 => "BZ2",
            Self::Zst => "ZST",
        }
    }

    pub fn supports_modification(&self) -> bool {
        matches!(self, Self::Zip)
    }
}

pub trait ArchiveBackend: Send {
    fn list_entries(&self) -> Result<Vec<ArchiveEntry>>;
    fn extract_all(&self, dest: &Path, progress: &dyn Progress) -> Result<()>;
    fn extract_entries(
        &self,
        entries: &[String],
        dest: &Path,
        progress: &dyn Progress,
    ) -> Result<()>;
    fn add_files(
        &mut self,
        files: &[PathBuf],
        archive_path_prefix: &str,
        progress: &dyn Progress,
    ) -> Result<()>;
    fn delete_entries(&mut self, entries: &[String], progress: &dyn Progress) -> Result<()>;
    fn format(&self) -> ArchiveFormat;
}

pub fn open_archive(path: &Path) -> Result<Box<dyn ArchiveBackend>> {
    let format = ArchiveFormat::from_path(path)
        .with_context(|| format!("Cannot determine format for: {}", path.display()))?;

    match format {
        ArchiveFormat::Zip => {
            let backend = zip_backend::ZipBackend::open(path)?;
            Ok(Box::new(backend))
        }
        ArchiveFormat::Tar
        | ArchiveFormat::TarGz
        | ArchiveFormat::TarBz2
        | ArchiveFormat::TarZst => {
            let backend = tar_backend::TarBackend::open(path, format)?;
            Ok(Box::new(backend))
        }
        ArchiveFormat::Rar => {
            let backend = rar_backend::RarBackend::open(path)?;
            Ok(Box::new(backend))
        }
        _ => bail!(
            "Format {} is not yet supported for browsing",
            format.display_name()
        ),
    }
}

pub fn test_archive(backend: &dyn ArchiveBackend, progress: &dyn Progress) -> Result<usize> {
    let entries = backend.list_entries()?;
    let total = entries.len() as u64;
    for (i, entry) in entries.iter().enumerate() {
        progress.send_progress(i as u64, total, format!("Testing: {}", entry.path));
    }
    progress.send_progress(total, total, "Test complete".to_string());
    Ok(entries.len())
}

pub fn create_archive(
    path: &Path,
    files: &[PathBuf],
    base_dir: &Path,
    format: ArchiveFormat,
    progress: &dyn Progress,
) -> Result<()> {
    match format {
        ArchiveFormat::Zip => zip_backend::ZipBackend::create(path, files, base_dir, progress),
        ArchiveFormat::TarGz
        | ArchiveFormat::TarBz2
        | ArchiveFormat::TarZst
        | ArchiveFormat::Tar => {
            tar_backend::TarBackend::create(path, files, base_dir, format, progress)
        }
        _ => bail!(
            "Creating {} archives is not yet supported",
            format.display_name()
        ),
    }
}
