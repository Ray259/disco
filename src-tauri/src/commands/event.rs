use tauri::State;
use uuid::Uuid;
use crate::core::db::EncyclopediaDb;
use crate::core::domain::models::event::Event;
use crate::core::domain::values::entity_ref::EntityType;
use crate::core::domain::values::rich_content::RichContent;
use crate::core::domain::values::date_range::DateRange;
use serde::Deserialize;
use chrono::NaiveDate;

#[derive(Deserialize)]
pub struct CreateEventRequest {
    name: String,
    start_date: String,
    end_date: String,
    description: Option<String>,
}

#[tauri::command]
pub async fn create_event(state: State<'_, EncyclopediaDb>, request: CreateEventRequest) -> Result<String, String> {
    let id = Uuid::new_v4();
    
    let start = NaiveDate::parse_from_str(&request.start_date, "%Y-%m-%d")
        .or_else(|_| NaiveDate::parse_from_str(&format!("{}-01-01", request.start_date), "%Y-%m-%d"))
        .map_err(|_| "Invalid start date".to_string())?;
        
    let end = NaiveDate::parse_from_str(&request.end_date, "%Y-%m-%d")
        .or_else(|_| NaiveDate::parse_from_str(&format!("{}-01-01", request.end_date), "%Y-%m-%d"))
        .map_err(|_| "Invalid end date".to_string())?;

    let date_range = DateRange { start, end };
    
    let mut event = Event::new(id, request.name.clone(), date_range);

    if let Some(desc) = request.description {
        if !desc.is_empty() {
            event = event.with_description(RichContent::from_text(&desc));
        }
    }

    let data = serde_json::to_string(&event).map_err(|e| e.to_string())?;

    state.insert_entity(id, EntityType::Event, &event.name, &data)
        .await
        .map_err(|e| e.to_string())?;

    Ok(id.to_string())
}
