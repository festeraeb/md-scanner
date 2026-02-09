// Tauri app entry point
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use wayfinder_tauri::commands;

fn main() {
    tauri::Builder::default()
        .plugin(tauri_plugin_dialog::init())
        .setup(|_app| {
            // Initialize app state if needed
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            commands::scan_directory,
            commands::generate_embeddings,
            commands::create_clusters,
            commands::search,
            commands::get_clusters_summary,
            commands::get_timeline,
            commands::get_stats,
            commands::validate_index,
            commands::get_system_info,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
