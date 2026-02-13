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
            commands::encyclopedia::get_all_figures,
            commands::encyclopedia::get_figure,
            commands::encyclopedia::create_figure,
            commands::encyclopedia::create_institution,
            commands::encyclopedia::create_event,
            commands::encyclopedia::create_geo,
            commands::encyclopedia::create_work
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
