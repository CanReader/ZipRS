use std::path::PathBuf;
use std::sync::Mutex;

use serde::Serialize;
use tauri::{AppHandle, State};

use crate::archive::{self, ArchiveFormat};
use crate::archive::entry::ArchiveEntry;
use crate::progress::ProgressSender;

pub struct ArchiveState {
    pub path: Mutex<Option<PathBuf>>,
    pub format: Mutex<Option<ArchiveFormat>>,
}

impl ArchiveState {
    pub fn new() -> Self {
        Self {
            path: Mutex::new(None),
            format: Mutex::new(None),
        }
    }
}

#[derive(Serialize)]
pub struct OpenArchiveResponse {
    pub entries: Vec<ArchiveEntry>,
    pub format: String,
    pub path: String,
    pub supports_modification: bool,
}

#[tauri::command]
pub async fn open_archive(
    path: String,
    state: State<'_, ArchiveState>,
) -> Result<OpenArchiveResponse, String> {
    let path_buf = PathBuf::from(&path);

    let result = tokio::task::spawn_blocking(move || {
        let backend = archive::open_archive(&path_buf).map_err(|e| e.to_string())?;
        let entries = backend.list_entries().map_err(|e| e.to_string())?;
        let format = backend.format();
        Ok::<_, String>((entries, format, path_buf))
    })
    .await
    .map_err(|e| e.to_string())??;

    let (entries, format, path_buf) = result;
    *state.path.lock().unwrap() = Some(path_buf.clone());
    *state.format.lock().unwrap() = Some(format);

    Ok(OpenArchiveResponse {
        entries,
        format: format!("{:?}", format),
        path: path_buf.display().to_string(),
        supports_modification: format.supports_modification(),
    })
}

#[tauri::command]
pub async fn extract_all(
    dest: String,
    state: State<'_, ArchiveState>,
    app: AppHandle,
) -> Result<String, String> {
    let archive_path = state
        .path
        .lock()
        .unwrap()
        .clone()
        .ok_or("No archive open")?;
    let dest_path = PathBuf::from(dest);

    tokio::task::spawn_blocking(move || {
        let progress = ProgressSender::new(app);
        let backend = archive::open_archive(&archive_path).map_err(|e| e.to_string())?;
        backend
            .extract_all(&dest_path, progress)
            .map_err(|e| e.to_string())?;
        Ok(format!("Extracted to {}", dest_path.display()))
    })
    .await
    .map_err(|e| e.to_string())?
}

#[tauri::command]
pub async fn extract_selected(
    entries: Vec<String>,
    dest: String,
    state: State<'_, ArchiveState>,
    app: AppHandle,
) -> Result<String, String> {
    let archive_path = state
        .path
        .lock()
        .unwrap()
        .clone()
        .ok_or("No archive open")?;
    let dest_path = PathBuf::from(dest);
    let count = entries.len();

    tokio::task::spawn_blocking(move || {
        let progress = ProgressSender::new(app);
        let backend = archive::open_archive(&archive_path).map_err(|e| e.to_string())?;
        backend
            .extract_entries(&entries, &dest_path, progress)
            .map_err(|e| e.to_string())?;
        Ok(format!("Extracted {count} items to {}", dest_path.display()))
    })
    .await
    .map_err(|e| e.to_string())?
}

#[tauri::command]
pub async fn extract_here(
    state: State<'_, ArchiveState>,
    app: AppHandle,
) -> Result<String, String> {
    let archive_path = state
        .path
        .lock()
        .unwrap()
        .clone()
        .ok_or("No archive open")?;
    let dest = archive_path
        .parent()
        .unwrap_or(std::path::Path::new("."))
        .to_path_buf();

    tokio::task::spawn_blocking(move || {
        let progress = ProgressSender::new(app);
        let backend = archive::open_archive(&archive_path).map_err(|e| e.to_string())?;
        backend
            .extract_all(&dest, progress)
            .map_err(|e| e.to_string())?;
        Ok(format!("Extracted to {}", dest.display()))
    })
    .await
    .map_err(|e| e.to_string())?
}

#[tauri::command]
pub async fn add_files(
    files: Vec<String>,
    archive_prefix: String,
    state: State<'_, ArchiveState>,
    app: AppHandle,
) -> Result<OpenArchiveResponse, String> {
    let archive_path = state
        .path
        .lock()
        .unwrap()
        .clone()
        .ok_or("No archive open")?;
    let file_paths: Vec<PathBuf> = files.iter().map(PathBuf::from).collect();
    let count = file_paths.len();

    let path_clone = archive_path.clone();
    tokio::task::spawn_blocking(move || {
        let progress = ProgressSender::new(app);
        let mut backend = archive::open_archive(&path_clone).map_err(|e| e.to_string())?;
        backend
            .add_files(&file_paths, &archive_prefix, progress)
            .map_err(|e| e.to_string())?;
        Ok::<_, String>(())
    })
    .await
    .map_err(|e| e.to_string())??;

    // Re-read the archive to get updated entries
    let path_clone = archive_path.clone();
    let result = tokio::task::spawn_blocking(move || {
        let backend = archive::open_archive(&path_clone).map_err(|e| e.to_string())?;
        let entries = backend.list_entries().map_err(|e| e.to_string())?;
        let format = backend.format();
        Ok::<_, String>((entries, format))
    })
    .await
    .map_err(|e| e.to_string())??;

    let (entries, format) = result;
    Ok(OpenArchiveResponse {
        entries,
        format: format!("{:?}", format),
        path: archive_path.display().to_string(),
        supports_modification: format.supports_modification(),
    })
}

#[tauri::command]
pub async fn delete_entries(
    entries: Vec<String>,
    state: State<'_, ArchiveState>,
    app: AppHandle,
) -> Result<OpenArchiveResponse, String> {
    let archive_path = state
        .path
        .lock()
        .unwrap()
        .clone()
        .ok_or("No archive open")?;

    let path_clone = archive_path.clone();
    tokio::task::spawn_blocking(move || {
        let progress = ProgressSender::new(app);
        let mut backend = archive::open_archive(&path_clone).map_err(|e| e.to_string())?;
        backend
            .delete_entries(&entries, progress)
            .map_err(|e| e.to_string())?;
        Ok::<_, String>(())
    })
    .await
    .map_err(|e| e.to_string())??;

    // Re-read
    let path_clone = archive_path.clone();
    let result = tokio::task::spawn_blocking(move || {
        let backend = archive::open_archive(&path_clone).map_err(|e| e.to_string())?;
        let entries = backend.list_entries().map_err(|e| e.to_string())?;
        let format = backend.format();
        Ok::<_, String>((entries, format))
    })
    .await
    .map_err(|e| e.to_string())??;

    let (entries, format) = result;
    Ok(OpenArchiveResponse {
        entries,
        format: format!("{:?}", format),
        path: archive_path.display().to_string(),
        supports_modification: format.supports_modification(),
    })
}

#[tauri::command]
pub async fn create_archive(
    path: String,
    files: Vec<String>,
    base_dir: String,
    format: String,
    app: AppHandle,
) -> Result<String, String> {
    let path_buf = PathBuf::from(&path);
    let file_paths: Vec<PathBuf> = files.iter().map(PathBuf::from).collect();
    let base = PathBuf::from(&base_dir);
    let fmt = ArchiveFormat::from_string(&format).map_err(|e| e.to_string())?;

    tokio::task::spawn_blocking(move || {
        let progress = ProgressSender::new(app);
        archive::create_archive(&path_buf, &file_paths, &base, fmt, progress)
            .map_err(|e| e.to_string())?;
        Ok(path)
    })
    .await
    .map_err(|e| e.to_string())?
}

#[tauri::command]
pub async fn test_archive(
    state: State<'_, ArchiveState>,
    app: AppHandle,
) -> Result<String, String> {
    let archive_path = state
        .path
        .lock()
        .unwrap()
        .clone()
        .ok_or("No archive open")?;

    tokio::task::spawn_blocking(move || {
        let progress = ProgressSender::new(app);
        let backend = archive::open_archive(&archive_path).map_err(|e| e.to_string())?;
        let count = archive::test_archive(&*backend, progress).map_err(|e| e.to_string())?;
        Ok(format!(
            "Archive OK — {count} entries verified successfully"
        ))
    })
    .await
    .map_err(|e| e.to_string())?
}

#[tauri::command]
pub async fn extract_and_open(
    entry_path: String,
    state: State<'_, ArchiveState>,
    app: AppHandle,
) -> Result<(), String> {
    let archive_path = state
        .path
        .lock()
        .unwrap()
        .clone()
        .ok_or("No archive open")?;

    let extracted_path = tokio::task::spawn_blocking(move || {
        let tmp_dir = std::env::temp_dir().join("ziprs_preview");
        let _ = std::fs::create_dir_all(&tmp_dir);

        let progress = ProgressSender::new(app);
        let backend = archive::open_archive(&archive_path).map_err(|e| e.to_string())?;
        backend
            .extract_entries(&[entry_path.clone()], &tmp_dir, progress)
            .map_err(|e| e.to_string())?;

        Ok::<_, String>(tmp_dir.join(&entry_path))
    })
    .await
    .map_err(|e| e.to_string())??;

    open::that(&extracted_path).map_err(|e| e.to_string())?;
    Ok(())
}

#[tauri::command]
pub async fn drag_out(
    entries: Vec<String>,
    state: State<'_, ArchiveState>,
    app: AppHandle,
) -> Result<(), String> {
    let archive_path = state
        .path
        .lock()
        .unwrap()
        .clone()
        .ok_or("No archive open")?;

    // Extract selected entries to a temp directory
    let tmp_dir = std::env::temp_dir().join("ziprs_drag");
    let _ = std::fs::remove_dir_all(&tmp_dir);
    std::fs::create_dir_all(&tmp_dir).map_err(|e| e.to_string())?;

    let tmp_clone = tmp_dir.clone();
    let entries_clone = entries.clone();
    tokio::task::spawn_blocking(move || {
        let progress = ProgressSender::new(app);
        let backend = archive::open_archive(&archive_path).map_err(|e| e.to_string())?;
        backend
            .extract_entries(&entries_clone, &tmp_clone, progress)
            .map_err(|e| e.to_string())?;
        Ok::<_, String>(())
    })
    .await
    .map_err(|e| e.to_string())??;

    // Collect the extracted file paths
    let mut file_paths: Vec<String> = Vec::new();
    for entry in &entries {
        let full_path = tmp_dir.join(entry);
        if full_path.exists() {
            file_paths.push(full_path.display().to_string());
        }
    }

    if file_paths.is_empty() {
        return Err("No files extracted for drag".to_string());
    }

    // Spawn ripdrag with native Wayland backend (our app forces GDK_BACKEND=x11
    // but ripdrag needs native backend for proper drag-and-drop)
    let mut cmd = std::process::Command::new("ripdrag");
    cmd.args(&file_paths);
    cmd.args(["--no-click", "--and-exit"]);
    cmd.env_remove("GDK_BACKEND");
    cmd.spawn().map_err(|e| format!("Failed to start ripdrag: {e}"))?;

    Ok(())
}

#[tauri::command]
pub async fn close_archive(state: State<'_, ArchiveState>) -> Result<(), String> {
    *state.path.lock().unwrap() = None;
    *state.format.lock().unwrap() = None;
    Ok(())
}
