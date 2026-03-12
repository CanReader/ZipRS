use chrono::NaiveDateTime;
use serde::Serialize;

#[derive(Debug, Clone, Serialize)]
pub struct ArchiveEntry {
    pub path: String,
    pub name: String,
    pub is_directory: bool,
    pub compressed_size: u64,
    pub uncompressed_size: u64,
    pub modified: Option<String>,
    pub compression_method: String,
    pub crc32: Option<u32>,
    pub encrypted: bool,
}

impl ArchiveEntry {
    pub fn parent_path(&self) -> &str {
        if let Some(idx) = self.path.rfind('/') {
            &self.path[..idx]
        } else {
            ""
        }
    }

    pub fn extension(&self) -> String {
        if self.is_directory {
            return String::new();
        }
        self.name
            .rsplit('.')
            .next()
            .unwrap_or("")
            .to_uppercase()
    }

    pub fn file_type_display(&self) -> String {
        if self.is_directory {
            "Folder".to_string()
        } else {
            let ext = self.extension();
            if ext.is_empty() {
                "File".to_string()
            } else {
                format!("{ext} File")
            }
        }
    }

    pub fn compression_ratio(&self) -> String {
        if self.is_directory || self.uncompressed_size == 0 {
            return String::new();
        }
        let ratio = (1.0 - self.compressed_size as f64 / self.uncompressed_size as f64) * 100.0;
        if ratio < 0.0 {
            "0%".to_string()
        } else {
            format!("{ratio:.0}%")
        }
    }
}

pub fn format_datetime(dt: Option<NaiveDateTime>) -> Option<String> {
    dt.map(|d| d.format("%Y-%m-%d %H:%M").to_string())
}
