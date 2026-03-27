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

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .setup(|app| {
            tauri::async_runtime::block_on(async move {
                let db_path = "sqlite:encyclopedia.db?mode=rwc"; 
                let db = EncyclopediaDb::init(db_path).await.expect("failed to init db");
                
                // Get app data dir for config storage
                let app_data_dir = app.path().app_data_dir().unwrap_or_else(|_| PathBuf::from("."));
                
                // Initialize vault manager (loads config or defaults)
                let vault = VaultManager::new(app_data_dir, None).expect("failed to init vault");
                
                // Export existing DB entities to vault on first run (migration)
                // Only exports if vault is empty
                if let Some(vp) = &vault.vault_path {
                    let vault_files: Vec<_> = std::fs::read_dir(vp)
                        .map(|entries| entries.filter_map(|e| e.ok()).collect())
                        .unwrap_or_default();
                    
                    let has_md_files = vault_files.iter().any(|entry: &std::fs::DirEntry| {
                        entry.path().is_dir() && {
                            std::fs::read_dir(entry.path())
                                .map(|e| e.filter_map(|e| e.ok())
                                    .any(|f| f.path().extension().map_or(false, |ext| ext == "md")))
                                .unwrap_or(false)
                        }
                    });
                    
                    if !has_md_files {
                        match vault.export_all_from_db(&db).await {
                            Ok(count) if count > 0 => println!("[vault] Migrated {} entities from DB to vault", count),
                            Ok(_) => println!("[vault] No existing entities to migrate"),
                            Err(e) => eprintln!("[vault] Migration error: {}", e),
                        }
                    }
                }
                
                // Startup sync: load any vault files into SQLite
                match vault.full_sync(&db).await {
                    Ok(report) => {
                        println!("[vault] Startup sync: {} files synced", report.synced);
                        for err in &report.errors {
                            eprintln!("[vault] Sync error: {}", err);
                        }
                    }
                    Err(e) => eprintln!("[vault] Startup sync failed: {}", e),
                }
                
                // Start file watcher for live sync with Obsidian
                let vault_arc = Arc::new(vault.clone());
                let db_arc = Arc::new(db.clone());
                match watcher::start_watcher(vault_arc, db_arc) {
                    Ok(handle) => {
                        // Store the watcher handle to keep it alive
                        app.manage(handle);
                        println!("[vault] File watcher started");
                    }
                    Err(e) => eprintln!("[vault] Failed to start file watcher: {}", e),
                }
                
                app.manage(db);
                app.manage(vault);
                app.manage(claude_bridge::init_claude_state());
                app.manage(gemini_bridge::init_gemini_state());
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
            commands::school::create_school_of_thought,
            commands::school::update_school_of_thought,
            commands::search::search_entities,
            claude_bridge::start_claude_session,
            claude_bridge::send_claude_input,
            claude_bridge::stop_claude_session,
            gemini_bridge::start_gemini_session,
            gemini_bridge::send_gemini_input,
            gemini_bridge::stop_gemini_session
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

    // Stop old watcher by unmanaging the old handle (it drops)
    app.unmanage::<WatcherHandle>();

    // Create a new VaultManager with the new path
    let app_data_dir = app.path().app_data_dir().unwrap_or_else(|_| PathBuf::from("."));
    let new_vault = VaultManager::new(app_data_dir, Some(path_buf.clone()))?;

    // Check if the NEW vault directory is empty of md files
    let vault_files: Vec<_> = std::fs::read_dir(&path_buf)
        .map(|entries| entries.filter_map(|e| e.ok()).collect())
        .unwrap_or_default();
    let new_vault_has_md_files = vault_files.iter().any(|entry: &std::fs::DirEntry| {
        entry.path().is_dir() && {
            std::fs::read_dir(entry.path())
                .map(|e| e.filter_map(|e| e.ok())
                    .any(|f| f.path().extension().map_or(false, |ext| ext == "md")))
                .unwrap_or(false)
        }
    });

    // If the new vault is empty, user might have created entities before picking a vault.
    // Export their existing DB content into the new vault so they don't lose data.
    if !new_vault_has_md_files {
        match new_vault.export_all_from_db(&db).await {
            Ok(count) if count > 0 => println!("[vault] Exported {} existing entities into new vault", count),
            Ok(_) => println!("[vault] No pre-existing entities to export"),
            Err(e) => eprintln!("[vault] Export error: {}", e),
        }
    }

    // Now empty the database completely to ensure 1 vault maps to 1 db version
    if let Err(e) = db.empty_database().await {
         return Err(format!("Failed to clear database before switching vaults: {}", e));
    }

    // Create a new VaultManager with the new path
    let app_data_dir = app.path().app_data_dir().unwrap_or_else(|_| PathBuf::from("."));
    let new_vault = VaultManager::new(app_data_dir, Some(path_buf.clone()))?;

    // Perform a full sync on the new directory
    match new_vault.full_sync(&db).await {
        Ok(report) => {
            println!("[vault] Changed path to {}. Synced {} files.", new_path, report.synced);
        }
        Err(e) => return Err(format!("Failed to sync new vault: {}", e)),
    }

    // Start new watcher
    let vault_arc = Arc::new(new_vault.clone());
    let db_arc = Arc::new(db.inner().clone());
    match watcher::start_watcher(vault_arc, db_arc) {
        Ok(handle) => {
            app.manage(handle);
            println!("[vault] Restarted file watcher for new path");
        }
        Err(e) => return Err(format!("Failed to start new watcher: {}", e)),
    }

    // Update the managed state with the new VaultManager
    app.manage(new_vault);
    
    Ok(())
}
