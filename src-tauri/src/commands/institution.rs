use tauri::State;
use uuid::Uuid;
use crate::core::db::EncyclopediaDb;
use crate::core::domain::models::institution::Institution;
use crate::core::domain::values::entity_ref::EntityType;
use crate::core::domain::values::rich_content::RichContent;
use crate::core::domain::values::date_range::DateRange;
use serde::Deserialize;
use chrono::NaiveDate;

#[derive(Deserialize)]
pub struct CreateInstitutionRequest {
    name: String,
    founded_start: Option<String>,
    founded_end: Option<String>,
    description: Option<String>,
}

#[tauri::command]
pub async fn create_institution(state: State<'_, EncyclopediaDb>, request: CreateInstitutionRequest) -> Result<String, String> {
    let id = Uuid::new_v4();
    
    let mut institution = Institution::new(id, request.name.clone());

    if let (Some(start), Some(end)) = (request.founded_start, request.founded_end) {
        if !start.is_empty() && !end.is_empty() {
            let start_date = NaiveDate::parse_from_str(&format!("{}-01-01", start), "%Y-%m-%d")
                .map_err(|_| "Invalid founded start year".to_string())?;
            let end_date = NaiveDate::parse_from_str(&format!("{}-01-01", end), "%Y-%m-%d")
                .map_err(|_| "Invalid founded end year".to_string())?;

            institution.founded = Some(DateRange {
                start: start_date,
                end: end_date,
            });
             institution.updated_at = chrono::Utc::now();
        }
    }

    if let Some(desc) = request.description {
        if !desc.is_empty() {
            institution = institution.with_description(RichContent::from_text(&desc));
        }
    }

    let data = serde_json::to_string(&institution).map_err(|e| e.to_string())?;

    state.insert_entity(
        id,
        EntityType::Institution,
        &institution.name,
        &data
    )
    .await
    .map_err(|e| e.to_string())?;

    Ok(id.to_string())
}
