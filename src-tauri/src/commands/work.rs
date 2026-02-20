use tauri::State;
use uuid::Uuid;
use crate::core::db::EncyclopediaDb;
use crate::core::domain::models::work::Work;
use crate::core::domain::values::entity_ref::EntityType;
use crate::core::domain::values::rich_content::RichContent;
use crate::core::domain::traits::InputDto;
use serde::Deserialize;
use super::RelationDto;
use super::common::{handle_create, handle_update};

/// DTO for creating a new Work (Book, Theory, etc.).
#[derive(Deserialize)]
pub struct CreateWorkRequest {
    pub title: String,
    pub summary: Option<String>,
    pub relations: Option<Vec<crate::commands::RelationDto>>,
}

impl InputDto<Work> for CreateWorkRequest {
    fn to_entity(&self, id: Uuid) -> Result<Work, String> {
        let mut work = Work::new(id, self.title.clone());

        if let Some(sum) = &self.summary {
            if !sum.is_empty() {
                work = work.with_summary(RichContent::from_text(sum));
            }
        }
        Ok(work)
    }

    fn update_entity(&self, work: &mut Work) -> Result<(), String> {
        work.title = self.title.clone();

        if let Some(sum) = &self.summary {
            if !sum.is_empty() {
                work.summary = Some(RichContent::from_text(sum));
            } else {
                work.summary = None;
            }
        } else {
            work.summary = None;
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

/// Retrieves all entities with type `Work`.
#[tauri::command]
pub async fn get_all_works(state: State<'_, EncyclopediaDb>) -> Result<Vec<Work>, String> {
    let entities = state.list_entities(Some(EntityType::Work))
        .await
        .map_err(|e| e.to_string())?;

    let items: Result<Vec<Work>, String> = entities.into_iter()
        .map(|(id, _name, data)| {
             let mut entity: Work = serde_json::from_str(&data).map_err(|e| e.to_string())?;
             entity.id = id;
             Ok(entity)
        })
        .collect();

    items
}

/// Retrieves a single Work by ID.
#[tauri::command]
pub async fn get_work(state: State<'_, EncyclopediaDb>, id: Uuid) -> Result<Option<Work>, String> {
    let result = state.get_entity(id).await.map_err(|e| e.to_string())?;
    match result {
        Some((_type_str, _name, data)) => {
            let mut entity: Work = serde_json::from_str(&data).map_err(|e| e.to_string())?;
            entity.id = id;
            Ok(Some(entity))
        },
        None => Ok(None)
    }
}

/// Creates a new Work and persists it.
#[tauri::command]
pub async fn create_work(state: State<'_, EncyclopediaDb>, request: CreateWorkRequest) -> Result<String, String> {
    handle_create(state, request).await
}

#[tauri::command]
pub async fn update_work(state: State<'_, EncyclopediaDb>, id: Uuid, request: CreateWorkRequest) -> Result<String, String> {
    handle_update(state, id, request).await
}
