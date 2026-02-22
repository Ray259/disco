use tauri::State;
use uuid::Uuid;
use crate::core::db::EncyclopediaDb;
use crate::core::vault::VaultManager;
use crate::core::domain::models::school_of_thought::SchoolOfThought;
use crate::core::domain::values::entity_ref::EntityType;
use crate::core::domain::values::rich_content::RichContent;
use crate::core::domain::traits::InputDto;
use serde::Deserialize;
use super::RelationDto;
use super::common::{handle_create, handle_update};

/// DTO for creating a new School of Thought.
#[derive(Deserialize)]
pub struct CreateSchoolOfThoughtRequest {
    pub name: String,
    pub description: Option<String>,
    pub relations: Option<Vec<crate::commands::RelationDto>>,
}

impl InputDto<SchoolOfThought> for CreateSchoolOfThoughtRequest {
    fn to_entity(&self, id: Uuid) -> Result<SchoolOfThought, String> {
        let mut school = SchoolOfThought::new(id, self.name.clone());

        if let Some(desc) = &self.description {
            if !desc.is_empty() {
                school = school.with_description(RichContent::from_text(desc));
            }
        }
        Ok(school)
    }

    fn update_entity(&self, school: &mut SchoolOfThought) -> Result<(), String> {
        school.name = self.name.clone();

        if let Some(desc) = &self.description {
            if !desc.is_empty() {
                school.description = Some(RichContent::from_text(desc));
            } else {
                school.description = None;
            }
        } else {
            school.description = None;
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

/// Retrieves all entities with type `SchoolOfThought`.
#[tauri::command]
pub async fn get_all_schools_of_thought(state: State<'_, EncyclopediaDb>) -> Result<Vec<SchoolOfThought>, String> {
    let entities = state.list_entities(Some(EntityType::SchoolOfThought))
        .await
        .map_err(|e| e.to_string())?;

    let items: Result<Vec<SchoolOfThought>, String> = entities.into_iter()
        .map(|(id, _name, data)| {
             let mut entity: SchoolOfThought = serde_json::from_str(&data).map_err(|e| e.to_string())?;
             entity.id = id;
             Ok(entity)
        })
        .collect();

    items
}

/// Retrieves a single SchoolOfThought by ID.
#[tauri::command]
pub async fn get_school_of_thought(state: State<'_, EncyclopediaDb>, id: Uuid) -> Result<Option<SchoolOfThought>, String> {
    let result = state.get_entity(id).await.map_err(|e| e.to_string())?;
    match result {
        Some((_type_str, _name, data)) => {
            let mut entity: SchoolOfThought = serde_json::from_str(&data).map_err(|e| e.to_string())?;
            entity.id = id;
            Ok(Some(entity))
        },
        None => Ok(None)
    }
}

/// Creates a new School of Thought and persists it.
#[tauri::command]
pub async fn create_school_of_thought(state: State<'_, EncyclopediaDb>, vault: State<'_, VaultManager>, request: CreateSchoolOfThoughtRequest) -> Result<String, String> {
    handle_create(state, vault, request).await
}

#[tauri::command]
pub async fn update_school_of_thought(state: State<'_, EncyclopediaDb>, vault: State<'_, VaultManager>, id: Uuid, request: CreateSchoolOfThoughtRequest) -> Result<String, String> {
    handle_update(state, vault, id, request).await
}
