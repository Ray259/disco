use tauri::State;
use uuid::Uuid;
use crate::core::db::EncyclopediaDb;
use crate::core::vault::VaultManager;
use crate::core::domain::models::event::Event;
use crate::core::domain::values::entity_ref::EntityType;
use crate::core::domain::values::rich_content::RichContent;
use crate::core::domain::values::date_range::DateRange;
use crate::core::domain::traits::InputDto;
use serde::Deserialize;
use chrono::NaiveDate;
use super::RelationDto;
use super::common::{handle_create, handle_update, parse_flexible_date};

/// DTO for creating a new Event.
#[derive(Deserialize)]
pub struct CreateEventRequest {
    pub name: String,
    pub start_date: String,
    pub end_date: String,
    pub description: Option<String>,
    pub relations: Option<Vec<RelationDto>>,
}

impl InputDto<Event> for CreateEventRequest {
    fn to_entity(&self, id: Uuid) -> Result<Event, String> {
        let start_date = parse_flexible_date(&self.start_date, "start")?;
        let end_date = parse_flexible_date(&self.end_date, "end")?;

        let date_range = DateRange { start: start_date, end: end_date };
        let mut event = Event::new(id, self.name.clone(), date_range);

        if let Some(desc) = &self.description {
            if !desc.is_empty() {
                event = event.with_description(RichContent::from_text(desc));
            }
        }
        Ok(event)
    }

    fn update_entity(&self, event: &mut Event) -> Result<(), String> {
        event.name = self.name.clone();
        
        let start_date = parse_flexible_date(&self.start_date, "start")?;
        let end_date = parse_flexible_date(&self.end_date, "end")?;

        event.date_range = DateRange { start: start_date, end: end_date };
        
        if let Some(desc) = &self.description {
             if !desc.is_empty() {
                  event.description = Some(RichContent::from_text(desc));
             } else {
                  event.description = None;
             }
        } else {
             event.description = None;
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

/// Retrieves all entities with type `Event`.
#[tauri::command]
pub async fn get_all_events(state: State<'_, EncyclopediaDb>) -> Result<Vec<Event>, String> {
    let entities = state.list_entities(Some(EntityType::Event))
        .await
        .map_err(|e| e.to_string())?;

    let items: Result<Vec<Event>, String> = entities.into_iter()
        .map(|(id, _name, data)| {
             let mut entity: Event = serde_json::from_str(&data).map_err(|e| e.to_string())?;
             entity.id = id;
             Ok(entity)
        })
        .collect();

    items
}

/// Retrieves a single Event by ID.
#[tauri::command]
pub async fn get_event(state: State<'_, EncyclopediaDb>, id: Uuid) -> Result<Option<Event>, String> {
    let result = state.get_entity(id).await.map_err(|e| e.to_string())?;
    match result {
        Some((_type_str, _name, data)) => {
            let mut entity: Event = serde_json::from_str(&data).map_err(|e| e.to_string())?;
            entity.id = id;
            Ok(Some(entity))
        },
        None => Ok(None)
    }
}

/// Creates a new Event and persists it.
#[tauri::command]
pub async fn create_event(state: State<'_, EncyclopediaDb>, vault: State<'_, VaultManager>, request: CreateEventRequest) -> Result<String, String> {
    handle_create(state, vault, request).await
}

#[tauri::command]
pub async fn update_event(state: State<'_, EncyclopediaDb>, vault: State<'_, VaultManager>, id: Uuid, request: CreateEventRequest) -> Result<String, String> {
    handle_update(state, vault, id, request).await
}
