use tauri::State;
use uuid::Uuid;
use crate::core::db::EncyclopediaDb;
use crate::core::domain::models::work::Work;
use crate::core::domain::values::entity_ref::EntityType;
use crate::core::domain::values::rich_content::RichContent;
use serde::Deserialize;

/// DTO for creating a new Work (Book, Theory, etc.).
#[derive(Deserialize)]
pub struct CreateWorkRequest {
    pub title: String,
    pub summary: Option<String>,
    pub relations: Option<Vec<crate::commands::RelationDto>>,
}

/// Retrieves all entities with type `Work`.
#[tauri::command]
pub async fn get_all_works(state: State<'_, EncyclopediaDb>) -> Result<Vec<Work>, String> {
    let entities = state.list_entities(Some(EntityType::Work))
        .await
        .map_err(|e| e.to_string())?;

    let items: Result<Vec<Work>, String> = entities.into_iter()
        .map(|(_id, _name, data)| serde_json::from_str(&data).map_err(|e| e.to_string()))
        .collect();

    items
}

/// Creates a new Work and persists it.
#[tauri::command]
pub async fn create_work(state: State<'_, EncyclopediaDb>, request: CreateWorkRequest) -> Result<String, String> {
    let id = Uuid::new_v4();
    let mut work = Work::new(id, request.title.clone());

    if let Some(sum) = request.summary {
        if !sum.is_empty() {
            work = work.with_summary(RichContent::from_text(&sum));
        }
    }

    let data = serde_json::to_string(&work).map_err(|e| e.to_string())?;

    state.insert_entity(id, EntityType::Work, &work.title, &data)
        .await
        .map_err(|e| e.to_string())?;

    // Handle Relations
    if let Some(relations) = request.relations {
        for rel in relations {
            state.insert_relation(id, rel.target_id, &rel.relation_type)
                .await
                .map_err(|e| e.to_string())?;
        }
    }

    Ok(id.to_string())
}
