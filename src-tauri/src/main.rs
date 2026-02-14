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
            commands::save_azure_config,
            commands::load_azure_config,
            commands::get_clusters_data,
            // Progress and error tracking
            commands::get_embedding_progress,
            commands::get_error_log,
            commands::clear_error_log,
            // Git Clippy Assistant commands
            commands::get_git_clippy_report,
            commands::execute_clippy_action,
            commands::is_git_repo,
            commands::delete_duplicate_files,
            // File Intelligence commands
            commands::scan_for_documents,
            commands::get_organization_suggestions,
            commands::get_scan_statistics,
            commands::dismiss_suggestion,
            // File Watcher commands
            commands::start_file_watcher,
            commands::stop_file_watcher,
            commands::get_file_events,
            commands::get_watcher_status,
            commands::validate_azure_config,
            commands::validate_all_azure_configs,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
