use tauri::State;
use uuid::Uuid;
use crate::core::db::EncyclopediaDb;
use crate::core::domain::models::geo::Geo;
use crate::core::domain::values::entity_ref::EntityType;
use crate::core::domain::values::rich_content::RichContent;
use serde::Deserialize;

#[derive(Deserialize)]
pub struct CreateGeoRequest {
    name: String,
    region: Option<String>,
    description: Option<String>,
}

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
