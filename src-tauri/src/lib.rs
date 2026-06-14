pub mod archive;
pub mod cli;
mod commands;
pub mod progress;

use commands::{ArchiveState, PendingOpen};
use std::sync::Mutex;
use tauri::Manager;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    // WebKit2GTK 2.46+ defaults to a DMA-BUF renderer that can break the
    // custom URI scheme handler (tauri://localhost) on some Linux desktops,
    // causing a "Connection refused" error on startup.  Disable it before
    // Tauri / Wry initialise the WebView.
    #[cfg(target_os = "linux")]
    {
        std::env::set_var("WEBKIT_DISABLE_DMABUF_RENDERER", "1");
    }

    tauri::Builder::default()
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_fs::init())
        .plugin(tauri_plugin_shell::init())
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_drag::init())
        .manage(ArchiveState::new())
        .manage(PendingOpen(Mutex::new(None)))
        .setup(|app| {
            let args: Vec<String> = std::env::args().skip(1).collect();
            for arg in &args {
                let p = std::path::Path::new(arg);
                if p.is_file() {
                    let abs = p.canonicalize().unwrap_or_else(|_| p.to_path_buf());
                    let s = abs.to_string_lossy().to_string();
                    eprintln!("[ziprs] pending open from argv: {}", s);
                    if let Some(state) = app.try_state::<PendingOpen>() {
                        *state.0.lock().unwrap() = Some(s);
                    }
                    break;
                }
            }
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            commands::open_archive,
            commands::extract_all,
            commands::extract_selected,
            commands::extract_here,
            commands::add_files,
            commands::delete_entries,
            commands::create_archive,
            commands::test_archive,
            commands::extract_and_open,
            commands::drag_out,
            commands::close_archive,
            commands::consume_pending_open,
        ])
        .run(tauri::generate_context!())
        .expect("error while running ZipRS");
}
