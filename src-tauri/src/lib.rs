pub mod archive;
pub mod cli;
mod commands;
pub mod progress;

use commands::ArchiveState;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_fs::init())
        .plugin(tauri_plugin_shell::init())
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_drag::init())
        .manage(ArchiveState::new())
        .setup(|app| {
            // Handle file association: when launched with a file path argument,
            // emit an event so the frontend can open it automatically.
            let args: Vec<String> = std::env::args().collect();
            if args.len() > 1 {
                let path = &args[1];
                if std::path::Path::new(path).is_file() {
                    let handle = app.handle().clone();
                    let path = path.to_string();
                    // Delay slightly so the frontend has time to mount listeners
                    std::thread::spawn(move || {
                        std::thread::sleep(std::time::Duration::from_millis(500));
                        use tauri::Emitter;
                        let _ = handle.emit("open-file-association", &path);
                    });
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
        ])
        .run(tauri::generate_context!())
        .expect("error while running ZipRS");
}
