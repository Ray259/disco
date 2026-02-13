use tauri::State;
use uuid::Uuid;
use crate::core::db::EncyclopediaDb;
use crate::core::domain::models::event::Event;
use crate::core::domain::values::entity_ref::EntityType;
use crate::core::domain::values::rich_content::RichContent;
use crate::core::domain::values::date_range::DateRange;
use serde::Deserialize;
use chrono::NaiveDate;

/// DTO for creating a new Event.
#[derive(Deserialize)]
pub struct CreateEventRequest {
    pub name: String,
    pub start_date: String,
    pub end_date: String,
    pub description: Option<String>,
    pub relations: Option<Vec<crate::commands::RelationDto>>,
}

/// Retrieves all entities with type `Event`.
#[tauri::command]
pub async fn get_all_events(state: State<'_, EncyclopediaDb>) -> Result<Vec<Event>, String> {
    let entities = state.list_entities(Some(EntityType::Event))
        .await
        .map_err(|e| e.to_string())?;

    let items: Result<Vec<Event>, String> = entities.into_iter()
        .map(|(_id, _name, data)| serde_json::from_str(&data).map_err(|e| e.to_string()))
        .collect();

    items
}

/// Creates a new Event and persists it.
///
/// Handles date parsing for `start_date` and `end_date` to create a `DateRange`.
#[tauri::command]
pub async fn create_event(state: State<'_, EncyclopediaDb>, request: CreateEventRequest) -> Result<String, String> {
    let id = Uuid::new_v4();
    
    let start_date = NaiveDate::parse_from_str(&request.start_date, "%Y-%m-%d")
        .or_else(|_| NaiveDate::parse_from_str(&format!("{}-01-01", request.start_date), "%Y-%m-%d"))
        .map_err(|_| "Invalid start date".to_string())?;
        
    let end_date = NaiveDate::parse_from_str(&request.end_date, "%Y-%m-%d")
        .or_else(|_| NaiveDate::parse_from_str(&format!("{}-01-01", request.end_date), "%Y-%m-%d"))
        .map_err(|_| "Invalid end date".to_string())?;

    let date_range = DateRange { start: start_date, end: end_date };
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
