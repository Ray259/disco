use tauri::State;
use crate::core::db::EncyclopediaDb;
use crate::core::vault::VaultManager;
use crate::core::domain::models::institution::Institution;
use crate::core::domain::values::entity_ref::EntityType;
use crate::core::domain::values::rich_content::RichContent;
use crate::core::domain::values::date_range::DateRange;
use crate::core::domain::traits::InputDto;
use serde::Deserialize;
use super::common::{handle_create, handle_update, parse_flexible_date};

#[derive(Deserialize)]
pub struct CreateInstitutionRequest {
    pub name: String,
    pub founded_start: Option<String>,
    pub founded_end: Option<String>,
    pub description: Option<RichContent>,
}

impl InputDto<Institution> for CreateInstitutionRequest {
    fn to_entity(&self) -> Result<Institution, String> {
        let mut inst = Institution::new(self.name.clone());
        if let (Some(s), Some(e)) = (&self.founded_start, &self.founded_end) {
            if !s.is_empty() && !e.is_empty() {
                inst.founded = Some(DateRange {
                    start: parse_flexible_date(s, "founded_start")?,
                    end: parse_flexible_date(e, "founded_end")?,
                });
            }
        }
        if let Some(d) = &self.description { if !d.is_empty() { inst.description = Some(d.clone()); } }
        Ok(inst)
    }

    fn update_entity(&self, inst: &mut Institution) -> Result<(), String> {
        inst.name = self.name.clone();
        if let (Some(s), Some(e)) = (&self.founded_start, &self.founded_end) {
            if !s.is_empty() && !e.is_empty() {
                inst.founded = Some(DateRange {
                    start: parse_flexible_date(s, "founded_start")?,
                    end: parse_flexible_date(e, "founded_end")?,
                });
            }
        }
        inst.description = self.description.as_ref().filter(|d| !d.is_empty()).cloned();
        Ok(())
    }
}

#[tauri::command]
pub async fn get_all_institutions(state: State<'_, EncyclopediaDb>) -> Result<Vec<Institution>, String> {
    let rows = state.list_entities(Some(EntityType::Institution)).await.map_err(|e| e.to_string())?;
    rows.into_iter().map(|(_name, data)| serde_json::from_str(&data).map_err(|e| e.to_string())).collect()
}

#[tauri::command]
pub async fn get_institution(state: State<'_, EncyclopediaDb>, name: String) -> Result<Option<Institution>, String> {
    match state.get_entity(EntityType::Institution, &name).await.map_err(|e| e.to_string())? {
        Some(data) => serde_json::from_str(&data).map(Some).map_err(|e| e.to_string()),
        None => Ok(None),
    }
}

#[tauri::command]
pub async fn create_institution(state: State<'_, EncyclopediaDb>, vault: State<'_, VaultManager>, request: CreateInstitutionRequest) -> Result<String, String> {
    handle_create(state, vault, request).await
}

#[tauri::command]
pub async fn update_institution(state: State<'_, EncyclopediaDb>, vault: State<'_, VaultManager>, name: String, request: CreateInstitutionRequest) -> Result<String, String> {
    handle_update(state, vault, EntityType::Institution, name, request).await
}
