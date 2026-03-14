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
        .manage(ArchiveState::new())
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
