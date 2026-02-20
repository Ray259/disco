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
            commands::search::search_entities
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
