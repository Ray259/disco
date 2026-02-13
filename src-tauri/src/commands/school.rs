use tauri::State;
use uuid::Uuid;
use crate::core::db::EncyclopediaDb;
use crate::core::domain::models::school_of_thought::SchoolOfThought;
use crate::core::domain::values::entity_ref::EntityType;
use crate::core::domain::values::rich_content::RichContent;
use serde::Deserialize;

#[derive(Deserialize)]
pub struct CreateSchoolOfThoughtRequest {
    name: String,
    description: Option<String>,
}

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

    Ok(id.to_string())
}
