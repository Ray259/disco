use tauri::State;
use crate::core::db::EncyclopediaDb;
use crate::core::vault::VaultManager;
use crate::core::domain::models::school_of_thought::SchoolOfThought;
use crate::core::domain::values::entity_ref::EntityType;
use crate::core::domain::values::rich_content::RichContent;
use crate::core::domain::traits::InputDto;
use serde::Deserialize;
use super::common::{handle_create, handle_update};

#[derive(Deserialize)]
pub struct CreateSchoolOfThoughtRequest {
    pub name: String,
    pub description: Option<RichContent>,
}

impl InputDto<SchoolOfThought> for CreateSchoolOfThoughtRequest {
    fn to_entity(&self) -> Result<SchoolOfThought, String> {
        let mut s = SchoolOfThought::new(self.name.clone());
        if let Some(d) = &self.description { if !d.is_empty() { s.description = Some(d.clone()); } }
        Ok(s)
    }

    fn update_entity(&self, s: &mut SchoolOfThought) -> Result<(), String> {
        s.name = self.name.clone();
        s.description = self.description.as_ref().filter(|d| !d.is_empty()).cloned();
        Ok(())
    }
}

#[tauri::command]
pub async fn get_all_schools_of_thought(state: State<'_, EncyclopediaDb>) -> Result<Vec<SchoolOfThought>, String> {
    let rows = state.list_entities(Some(EntityType::SchoolOfThought)).await.map_err(|e| e.to_string())?;
    rows.into_iter().map(|(_name, data)| serde_json::from_str(&data).map_err(|e| e.to_string())).collect()
}

#[tauri::command]
pub async fn get_school_of_thought(state: State<'_, EncyclopediaDb>, name: String) -> Result<Option<SchoolOfThought>, String> {
    match state.get_entity(EntityType::SchoolOfThought, &name).await.map_err(|e| e.to_string())? {
        Some(data) => serde_json::from_str(&data).map(Some).map_err(|e| e.to_string()),
        None => Ok(None),
    }
}

#[tauri::command]
pub async fn create_school_of_thought(state: State<'_, EncyclopediaDb>, vault: State<'_, VaultManager>, request: CreateSchoolOfThoughtRequest) -> Result<String, String> {
    handle_create(state, vault, request).await
}

#[tauri::command]
pub async fn update_school_of_thought(state: State<'_, EncyclopediaDb>, vault: State<'_, VaultManager>, name: String, request: CreateSchoolOfThoughtRequest) -> Result<String, String> {
    handle_update(state, vault, EntityType::SchoolOfThought, name, request).await
}
