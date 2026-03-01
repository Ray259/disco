use tauri::State;
use crate::core::db::EncyclopediaDb;
use serde::Serialize;

#[derive(Serialize)]
pub struct SearchResult {
    pub entity_type: String,
    pub name: String,
}

#[tauri::command]
pub async fn search_entities(state: State<'_, EncyclopediaDb>, query: String) -> Result<Vec<SearchResult>, String> {
    if query.len() < 2 { return Ok(Vec::new()); }
    let results = state.search_entities(&query).await.map_err(|e| e.to_string())?;
    Ok(results.into_iter().map(|(entity_type, name, _data)| SearchResult { entity_type, name }).collect())
}
