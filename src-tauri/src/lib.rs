// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

pub mod core;
mod commands;
use tauri::Manager;
use crate::core::db::EncyclopediaDb;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .setup(|app| {
            tauri::async_runtime::block_on(async move {
                let db_path = "sqlite:encyclopedia.db?mode=rwc"; 
                // Ensure the database file exists or is created by sqlx if mode=rwc works, 
                // typically sqlx needs the file to exist for strict sqlite, but mode=rwc might help.
                // For simplicity, we point to a local file.
                let db = EncyclopediaDb::init(db_path).await.expect("failed to init db");
                app.manage(db);
            });
            Ok(())
        })
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            greet,
            commands::figure::get_all_figures,
            commands::figure::get_figure,
            commands::figure::create_figure,
            commands::institution::create_institution,
            commands::event::create_event,
            commands::geo::create_geo,
            commands::work::create_work,
            commands::school::create_school_of_thought
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
