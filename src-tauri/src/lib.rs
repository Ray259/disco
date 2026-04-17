// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

pub mod core;
mod commands;
use std::sync::Arc;
use std::path::PathBuf;
use tauri::Manager;
use crate::core::db::EncyclopediaDb;
use crate::core::vault::VaultManager;
use crate::core::watcher;
use crate::core::watcher::WatcherHandle;
use crate::core::claude_bridge;
use crate::core::gemini_bridge;
use crate::core::codex_bridge;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tracing_subscriber::fmt()
        .with_env_filter(tracing_subscriber::EnvFilter::from_default_env().add_directive(tracing::Level::INFO.into()))
        .init();

    tracing::info!("Starting DIS application...");

    tauri::Builder::default()
        .setup(|app| {
            tauri::async_runtime::block_on(async move {
                let db_path = "sqlite:encyclopedia.db?mode=rwc"; 
                let db = EncyclopediaDb::init(db_path).await.expect("failed to init db");
                
                // Get app data dir for config storage
                let app_data_dir = app.path().app_data_dir().unwrap_or_else(|_| PathBuf::from("."));
                
                // Initialize vault manager (loads config or defaults)
                let vault = VaultManager::new(app_data_dir, None).expect("failed to init vault");
                
                // Startup sync: load any vault files into SQLite
                match vault.full_sync(&db).await {
                    Ok(report) => if report.synced > 0 { tracing::info!("[vault] Startup sync: {} files synced", report.synced); },
                    Err(e) => tracing::error!("[vault] Startup sync error: {}", e),
                }
                
                // Start file watcher for live sync with Obsidian
                let vault_arc = Arc::new(vault.clone());
                let db_arc = Arc::new(db.clone());
                match watcher::start_watcher(vault_arc, db_arc) {
                    Ok(handle) => {
                        // Store the watcher handle to keep it alive
                        app.manage(handle);
                        tracing::info!("[vault] File watcher started");
                    }
                    Err(e) => tracing::error!("[vault] Failed to start file watcher: {}", e),
                }
                
                app.manage(db);
                app.manage(vault);
                app.manage(claude_bridge::init_claude_state());
                app.manage(gemini_bridge::init_gemini_state());
                app.manage(codex_bridge::init_codex_state());
            });
            Ok(())
        })
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            greet,
            get_vault_path,
            set_vault_path,
            commands::common::delete_entity,
            commands::figure::get_all_figures,
            commands::figure::get_figure,
            commands::figure::create_figure,
            commands::figure::update_figure,
            commands::institution::get_all_institutions,
            commands::institution::get_institution,
            commands::institution::create_institution,
            commands::institution::update_institution,
            commands::event::get_all_events,
            commands::event::get_event,
            commands::event::create_event,
            commands::event::update_event,
            commands::geo::get_all_geos,
            commands::geo::get_geo,
            commands::geo::create_geo,
            commands::geo::update_geo,
            commands::work::get_all_works,
            commands::work::get_work,
            commands::work::create_work,
            commands::work::update_work,
            commands::school::get_all_schools_of_thought,
            commands::school::get_school_of_thought,
            commands::school::create_school_of_thought,
            commands::school::update_school_of_thought,
            commands::search::search_entities,
            claude_bridge::start_claude_session,
            claude_bridge::resume_claude_session,
            claude_bridge::send_claude_input,
            claude_bridge::stop_claude_session,
            claude_bridge::get_claude_session_id,
            gemini_bridge::start_gemini_session,
            gemini_bridge::resume_gemini_session,
            gemini_bridge::send_gemini_input,
            gemini_bridge::stop_gemini_session,
            gemini_bridge::get_gemini_session_id,
            codex_bridge::start_codex_session,
            codex_bridge::resume_codex_session,
            codex_bridge::send_codex_input,
            codex_bridge::stop_codex_session,
            codex_bridge::get_codex_session_id,
            commands::terminal::launch_terminal_session
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

#[tauri::command]
fn get_vault_path(vault: tauri::State<'_, VaultManager>) -> Result<String, String> {
    Ok(match &vault.vault_path {
        Some(p) => p.to_string_lossy().to_string(),
        None => "".to_string(),
    })
}

#[tauri::command]
#[allow(deprecated)]
#[allow(unused_variables)]
async fn set_vault_path(
    app: tauri::AppHandle,
    vault: tauri::State<'_, VaultManager>,
    db: tauri::State<'_, EncyclopediaDb>,
    new_path: String
) -> Result<(), String> {
    let path_buf = PathBuf::from(&new_path);
    if !path_buf.exists() || !path_buf.is_dir() {
        return Err("Path does not exist or is not a directory".to_string());
    }

    app.unmanage::<WatcherHandle>();

    let app_data_dir = app.path().app_data_dir().unwrap_or_else(|_| PathBuf::from("."));
    let new_vault = VaultManager::new(app_data_dir, Some(path_buf.clone()))?;

    let mut new_vault_has_md_files = false;
    if let Ok(mut entries) = tokio::fs::read_dir(&path_buf).await {
        while let Ok(Some(entry)) = entries.next_entry().await {
            let path = entry.path();
            if path.is_dir() {
                if let Ok(mut sub_entries) = tokio::fs::read_dir(&path).await {
                    while let Ok(Some(sub_entry)) = sub_entries.next_entry().await {
                        if sub_entry.path().extension().map_or(false, |ext| ext == "md") {
                            new_vault_has_md_files = true;
                            break;
                        }
                    }
                }
            }
            if new_vault_has_md_files {
                break;
            }
        }
    }

    if !new_vault_has_md_files {
        match new_vault.export_all_from_db(&db).await {
            Ok(count) if count > 0 => tracing::info!("[vault] Exported {} existing entities into new vault", count),
            Ok(_) => tracing::info!("[vault] No pre-existing entities to export"),
            Err(e) => tracing::error!("[vault] Export error: {}", e),
        }
    }

    if let Err(e) = db.empty_database().await {
         return Err(format!("Failed to clear database before switching vaults: {}", e));
    }

    let app_data_dir = app.path().app_data_dir().unwrap_or_else(|_| PathBuf::from("."));
    let new_vault = VaultManager::new(app_data_dir, Some(path_buf.clone()))?;

    match new_vault.full_sync(&db).await {
        Ok(report) => {
            tracing::info!("[vault] Changed path to {}. Synced {} files.", new_path, report.synced);
        }
        Err(e) => return Err(format!("Failed to sync new vault: {}", e)),
    }

    let vault_arc = Arc::new(new_vault.clone());
    let db_arc = Arc::new(db.inner().clone());
    match watcher::start_watcher(vault_arc, db_arc) {
        Ok(handle) => {
            app.manage(handle);
            tracing::info!("[vault] Restarted file watcher for new path");
        }
        Err(e) => return Err(format!("Failed to start new watcher: {}", e)),
    }

    app.manage(new_vault);
    Ok(())
}
