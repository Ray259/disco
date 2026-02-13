use tauri::State;
use uuid::Uuid;
use crate::core::db::EncyclopediaDb;
use crate::core::domain::models::institution::Institution;
use crate::core::domain::values::entity_ref::EntityType;
use crate::core::domain::values::rich_content::RichContent;
use crate::core::domain::values::date_range::DateRange;
use serde::Deserialize;
use chrono::NaiveDate;

/// DTO for creating a new Institution.
#[derive(Deserialize)]
pub struct CreateInstitutionRequest {
    pub name: String,
    pub founded_start: Option<String>,
    pub founded_end: Option<String>,
    pub description: Option<String>,
    pub relations: Option<Vec<crate::commands::RelationDto>>,
}

/// Retrieves all entities with type `Institution`.
#[tauri::command]
pub async fn get_all_institutions(state: State<'_, EncyclopediaDb>) -> Result<Vec<Institution>, String> {
    let entities = state.list_entities(Some(EntityType::Institution))
        .await
        .map_err(|e| e.to_string())?;

    let items: Result<Vec<Institution>, String> = entities.into_iter()
        .map(|(_id, _name, data)| serde_json::from_str(&data).map_err(|e| e.to_string()))
        .collect();

    items
}

/// Creates a new Institution and persists it.
///
/// Handles optional date fields for `founded`.
#[tauri::command]
pub async fn create_institution(state: State<'_, EncyclopediaDb>, request: CreateInstitutionRequest) -> Result<String, String> {
    let id = Uuid::new_v4();
    let mut institution = Institution::new(id, request.name.clone());

    if let Some(desc) = request.description {
        if !desc.is_empty() {
            institution = institution.with_description(RichContent::from_text(&desc));
        }
    }

    if let (Some(start), Some(end)) = (request.founded_start, request.founded_end) {
        if !start.is_empty() && !end.is_empty() {
             let s = NaiveDate::parse_from_str(&start, "%Y-%m-%d")
                .or_else(|_| NaiveDate::parse_from_str(&format!("{}-01-01", start), "%Y-%m-%d"))
                .map_err(|_| "Invalid founded start year".to_string())?;
             let e = NaiveDate::parse_from_str(&end, "%Y-%m-%d")
                .or_else(|_| NaiveDate::parse_from_str(&format!("{}-01-01", end), "%Y-%m-%d"))
                .map_err(|_| "Invalid founded end year".to_string())?;
             
             institution.founded = Some(DateRange::new(s, e));
        }
    }

    let data = serde_json::to_string(&institution).map_err(|e| e.to_string())?;

    state.insert_entity(id, EntityType::Institution, &institution.name, &data)
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
