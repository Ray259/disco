use super::common::{handle_create, handle_update};
use crate::core::db::EncyclopediaDb;
use crate::core::domain::models::geo::Geo;
use crate::core::domain::traits::InputDto;
use crate::core::domain::values::entity_ref::EntityType;
use crate::core::domain::values::rich_content::RichContent;
use crate::core::vault::VaultManager;
use serde::Deserialize;
use tauri::State;

#[derive(Deserialize)]
pub struct CreateGeoRequest {
    pub name: String,
    pub region: Option<RichContent>,
    pub description: Option<RichContent>,
}

impl InputDto<Geo> for CreateGeoRequest {
    fn to_entity(&self) -> Result<Geo, String> {
        let mut geo = Geo::new(self.name.clone());
        if let Some(r) = &self.region {
            if !r.is_empty() {
                geo.region = Some(r.clone());
            }
        }
        if let Some(d) = &self.description {
            if !d.is_empty() {
                geo.description = Some(d.clone());
            }
        }
        Ok(geo)
    }

    fn update_entity(&self, geo: &mut Geo) -> Result<(), String> {
        geo.name = self.name.clone();
        geo.region = self.region.as_ref().filter(|r| !r.is_empty()).cloned();
        geo.description = self.description.as_ref().filter(|d| !d.is_empty()).cloned();
        Ok(())
    }
}

#[tauri::command]
pub async fn get_all_geos(state: State<'_, EncyclopediaDb>) -> Result<Vec<Geo>, String> {
    let rows = state
        .list_entities(Some(EntityType::Geo))
        .await
        .map_err(|e| e.to_string())?;
    rows.into_iter()
        .map(|(_name, data)| serde_json::from_str(&data).map_err(|e| e.to_string()))
        .collect()
}

#[tauri::command]
pub async fn get_geo(
    state: State<'_, EncyclopediaDb>,
    name: String,
) -> Result<Option<Geo>, String> {
    match state
        .get_entity(EntityType::Geo, &name)
        .await
        .map_err(|e| e.to_string())?
    {
        Some(data) => serde_json::from_str(&data)
            .map(Some)
            .map_err(|e| e.to_string()),
        None => Ok(None),
    }
}

#[tauri::command]
pub async fn create_geo(
    state: State<'_, EncyclopediaDb>,
    vault: State<'_, VaultManager>,
    request: CreateGeoRequest,
) -> Result<String, String> {
    handle_create(state, vault, request).await
}

#[tauri::command]
pub async fn update_geo(
    state: State<'_, EncyclopediaDb>,
    vault: State<'_, VaultManager>,
    name: String,
    request: CreateGeoRequest,
) -> Result<String, String> {
    handle_update(state, vault, EntityType::Geo, name, request).await
}
