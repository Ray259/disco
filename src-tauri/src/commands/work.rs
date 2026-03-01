use tauri::State;
use crate::core::db::EncyclopediaDb;
use crate::core::vault::VaultManager;
use crate::core::domain::models::work::Work;
use crate::core::domain::values::entity_ref::EntityType;
use crate::core::domain::values::rich_content::RichContent;
use crate::core::domain::traits::InputDto;
use serde::Deserialize;
use super::common::{handle_create, handle_update};

#[derive(Deserialize)]
pub struct CreateWorkRequest {
    pub title: String,
    pub summary: Option<RichContent>,
}

impl InputDto<Work> for CreateWorkRequest {
    fn to_entity(&self) -> Result<Work, String> {
        let mut work = Work::new(self.title.clone());
        if let Some(s) = &self.summary { if !s.is_empty() { work.summary = Some(s.clone()); } }
        Ok(work)
    }

    fn update_entity(&self, work: &mut Work) -> Result<(), String> {
        work.title = self.title.clone();
        work.summary = self.summary.as_ref().filter(|s| !s.is_empty()).cloned();
        Ok(())
    }
}

#[tauri::command]
pub async fn get_all_works(state: State<'_, EncyclopediaDb>) -> Result<Vec<Work>, String> {
    let rows = state.list_entities(Some(EntityType::Work)).await.map_err(|e| e.to_string())?;
    rows.into_iter().map(|(_name, data)| serde_json::from_str(&data).map_err(|e| e.to_string())).collect()
}

#[tauri::command]
pub async fn get_work(state: State<'_, EncyclopediaDb>, name: String) -> Result<Option<Work>, String> {
    match state.get_entity(EntityType::Work, &name).await.map_err(|e| e.to_string())? {
        Some(data) => serde_json::from_str(&data).map(Some).map_err(|e| e.to_string()),
        None => Ok(None),
    }
}

#[tauri::command]
pub async fn create_work(state: State<'_, EncyclopediaDb>, vault: State<'_, VaultManager>, request: CreateWorkRequest) -> Result<String, String> {
    handle_create(state, vault, request).await
}

#[tauri::command]
pub async fn update_work(state: State<'_, EncyclopediaDb>, vault: State<'_, VaultManager>, name: String, request: CreateWorkRequest) -> Result<String, String> {
    handle_update(state, vault, EntityType::Work, name, request).await
}
