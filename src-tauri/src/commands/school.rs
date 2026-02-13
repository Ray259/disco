use tauri::State;
use uuid::Uuid;
use crate::core::db::EncyclopediaDb;
use crate::core::domain::models::school_of_thought::SchoolOfThought;
use crate::core::domain::values::entity_ref::EntityType;
use crate::core::domain::values::rich_content::RichContent;
use serde::Deserialize;

/// DTO for creating a new School of Thought.
#[derive(Deserialize)]
pub struct CreateSchoolOfThoughtRequest {
    pub name: String,
    pub description: Option<String>,
    pub relations: Option<Vec<crate::commands::RelationDto>>,
}

/// Retrieves all entities with type `SchoolOfThought`.
#[tauri::command]
pub async fn get_all_schools_of_thought(state: State<'_, EncyclopediaDb>) -> Result<Vec<SchoolOfThought>, String> {
    let entities = state.list_entities(Some(EntityType::SchoolOfThought))
        .await
        .map_err(|e| e.to_string())?;

    let items: Result<Vec<SchoolOfThought>, String> = entities.into_iter()
        .map(|(_id, _name, data)| serde_json::from_str(&data).map_err(|e| e.to_string()))
        .collect();

    items
}

/// Creates a new School of Thought and persists it.
#[tauri::command]
pub async fn create_school_of_thought(state: State<'_, EncyclopediaDb>, request: CreateSchoolOfThoughtRequest) -> Result<String, String> {
    let id = Uuid::new_v4();
    let mut school = SchoolOfThought::new(id, request.name.clone());

    if let Some(desc) = request.description {
        if !desc.is_empty() {
            school = school.with_description(RichContent::from_text(&desc));
        }
    }

    let data = serde_json::to_string(&school).map_err(|e| e.to_string())?;

    state.insert_entity(id, EntityType::SchoolOfThought, &school.name, &data)
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
