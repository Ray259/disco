use tauri::State;
use uuid::Uuid;
use crate::core::db::EncyclopediaDb;
use crate::core::domain::models::geo::Geo;
use crate::core::domain::values::entity_ref::EntityType;
use crate::core::domain::values::rich_content::RichContent;
use crate::core::domain::traits::InputDto;
use serde::Deserialize;
use super::RelationDto;
use super::common::{handle_create, handle_update};

/// DTO for creating a new Location.
#[derive(Deserialize)]
pub struct CreateGeoRequest {
    pub name: String,
    pub region: Option<String>,
    pub description: Option<String>,
    pub relations: Option<Vec<crate::commands::RelationDto>>,
}

impl InputDto<Geo> for CreateGeoRequest {
    fn to_entity(&self, id: Uuid) -> Result<Geo, String> {
        let mut geo = Geo::new(id, self.name.clone());

        if let Some(reg) = &self.region {
            if !reg.is_empty() {
                geo = geo.with_region(RichContent::from_text(reg));
            }
        }
        if let Some(desc) = &self.description {
            if !desc.is_empty() {
                 geo = geo.with_description(RichContent::from_text(desc));
            }
        }
        Ok(geo)
    }

    fn update_entity(&self, geo: &mut Geo) -> Result<(), String> {
        geo.name = self.name.clone();

        if let Some(reg) = &self.region {
            if !reg.is_empty() {
                geo.region = Some(RichContent::from_text(reg));
            } else {
                geo.region = None;
            }
        } else {
            geo.region = None;
        }
    
        if let Some(desc) = &self.description {
            if !desc.is_empty() {
                 geo.description = Some(RichContent::from_text(desc));
            } else {
                geo.description = None;
            }
        } else {
            geo.description = None;
        }

        Ok(())
    }

    fn get_relations(&self) -> Option<Vec<RelationDto>> {
        let rels = self.relations.as_ref()?;
        Some(rels.iter().map(|r| RelationDto {
            target_id: r.target_id,
            relation_type: r.relation_type.clone()
        }).collect())
    }
}

/// Retrieves all entities with type `Geo`.
#[tauri::command]
pub async fn get_all_geos(state: State<'_, EncyclopediaDb>) -> Result<Vec<Geo>, String> {
    let entities = state.list_entities(Some(EntityType::Geo))
        .await
        .map_err(|e| e.to_string())?;

    let items: Result<Vec<Geo>, String> = entities.into_iter()
        .map(|(id, _name, data)| {
             let mut entity: Geo = serde_json::from_str(&data).map_err(|e| e.to_string())?;
             entity.id = id;
             Ok(entity)
        })
        .collect();

    items
}

/// Creates a new Location and persists it.
#[tauri::command]
pub async fn create_geo(state: State<'_, EncyclopediaDb>, request: CreateGeoRequest) -> Result<String, String> {
    handle_create(state, request).await
}

#[tauri::command]
pub async fn update_geo(state: State<'_, EncyclopediaDb>, id: Uuid, request: CreateGeoRequest) -> Result<String, String> {
    handle_update(state, id, request).await
}
