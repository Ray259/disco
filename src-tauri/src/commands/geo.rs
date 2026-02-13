use tauri::State;
use uuid::Uuid;
use crate::core::db::EncyclopediaDb;
use crate::core::domain::models::geo::Geo;
use crate::core::domain::values::entity_ref::EntityType;
use crate::core::domain::values::rich_content::RichContent;
use serde::Deserialize;

/// DTO for creating a new Location.
#[derive(Deserialize)]
pub struct CreateGeoRequest {
    pub name: String,
    pub region: Option<String>,
    pub description: Option<String>,
}

/// Retrieves all entities with type `Geo`.
#[tauri::command]
pub async fn get_all_geos(state: State<'_, EncyclopediaDb>) -> Result<Vec<Geo>, String> {
    let entities = state.list_entities(Some(EntityType::Geo))
        .await
        .map_err(|e| e.to_string())?;

    let items: Result<Vec<Geo>, String> = entities.into_iter()
        .map(|(_id, _name, data)| serde_json::from_str(&data).map_err(|e| e.to_string()))
        .collect();

    items
}

/// Creates a new Location and persists it.
#[tauri::command]
pub async fn create_geo(state: State<'_, EncyclopediaDb>, request: CreateGeoRequest) -> Result<String, String> {
    let id = Uuid::new_v4();
    let mut geo = Geo::new(id, request.name.clone());

    if let Some(reg) = request.region {
        if !reg.is_empty() {
            geo = geo.with_region(RichContent::from_text(&reg));
        }
    }
    if let Some(desc) = request.description {
        if !desc.is_empty() {
             geo = geo.with_description(RichContent::from_text(&desc));
        }
    }

    let data = serde_json::to_string(&geo).map_err(|e| e.to_string())?;

    state.insert_entity(id, EntityType::Geo, &geo.name, &data)
        .await
        .map_err(|e| e.to_string())?;

    Ok(id.to_string())
}
