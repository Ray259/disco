use tauri::State;
use uuid::Uuid;
use crate::core::db::EncyclopediaDb;
use serde::{Serialize, Deserialize};

/// Lightweight search result structure.
///
/// This avoids deserializing the full entity blob, just returning identifying info.
#[derive(Serialize, Deserialize)]
pub struct SearchResult {
    pub id: Uuid,
    pub entity_type: String,
    pub name: String,
    pub description: Option<String>, // Extracted from data if possible, or just None for now
}

/// Searches for entities by name.
///
/// Returns a list of `SearchResult`s.
/// Requires at least 2 characters in query.
#[tauri::command]
pub async fn search_entities(state: State<'_, EncyclopediaDb>, query: String) -> Result<Vec<SearchResult>, String> {
    if query.len() < 2 {
        return Ok(Vec::new());
    }

    let results = state.search_entities(&query)
        .await
        .map_err(|e| e.to_string())?;

    let search_results = results.into_iter().map(|(id, entity_type, name, _data)| {
        // In the future we could parse `_data` to get a snippet/description
        SearchResult {
            id,
            entity_type,
            name,
            description: None,
        }
    }).collect();

    Ok(search_results)
}
