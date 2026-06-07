pub mod commands;
pub mod sandbox;
pub mod ai;
pub mod git;
pub mod mcp;
pub mod vector;
pub mod config;
pub mod logging;
pub mod types;

use std::sync::Arc;
use tokio::sync::RwLock;

use crate::config::Config;
use crate::git::GitEngine;
use crate::logging::init_logging;
use crate::sandbox::SandboxEngine;
use crate::vector::VectorStore;
use crate::types::AppState;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    init_logging();

    let config = Arc::new(RwLock::new(Config::default()));
    let sandbox = Arc::new(SandboxEngine::new());
    let vault_path = std::path::PathBuf::from(".");

    let state = AppState {
        config: config.clone(),
        sandbox: sandbox.clone(),
        git: Arc::new(RwLock::new(GitEngine::new(&vault_path))),
        vector: Arc::new(VectorStore::new(&vault_path)),
        vault_path: vault_path.clone(),
    };

    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .plugin(tauri_plugin_fs::init())
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_clipboard_manager::init())
        .manage(state)
        .invoke_handler(tauri::generate_handler![
            commands::editor::read_file,
            commands::editor::save_file,
            commands::editor::list_files,
            commands::editor::create_file,
            commands::editor::create_folder,
            commands::editor::rename_file,
            commands::editor::delete_file,
            commands::editor::duplicate_file,
            commands::editor::get_tags,
            commands::editor::get_recent_files,
            commands::search::search_files,
            commands::git::git_commit,
            commands::git::git_history,
            commands::git::git_diff,
            commands::git::git_status,
            commands::git::git_push,
            commands::git::git_pull,
            commands::ai::ai_chat,
            commands::ai::ai_template,
            commands::ai::ai_list_models,
            commands::ai::test_provider_connection,
            commands::ingest::ingest_file,
            commands::ingest::ingest_url,
            commands::ingest::list_converters,
            commands::mcp::mcp_list_servers,
            commands::mcp::mcp_add_server,
            commands::mcp::mcp_remove_server,
            commands::mcp::mcp_test_connection,
            commands::mcp::mcp_call_tool,
        ])
        .run(tauri::generate_context!())
        .expect("error while running ZarishNote");
}
