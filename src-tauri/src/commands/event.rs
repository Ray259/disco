use super::common::{handle_create, handle_update, parse_flexible_date};
use crate::core::db::EncyclopediaDb;
use crate::core::domain::models::event::Event;
use crate::core::domain::traits::InputDto;
use crate::core::domain::values::date_range::DateRange;
use crate::core::domain::values::entity_ref::EntityType;
use crate::core::domain::values::rich_content::RichContent;
use crate::core::vault::VaultManager;
use serde::Deserialize;
use tauri::State;

#[derive(Deserialize)]
pub struct CreateEventRequest {
    pub name: String,
    pub start_date: String,
    pub end_date: String,
    pub description: Option<RichContent>,
}

impl InputDto<Event> for CreateEventRequest {
    fn to_entity(&self) -> Result<Event, String> {
        let start = parse_flexible_date(&self.start_date, "start")?;
        let end = parse_flexible_date(&self.end_date, "end")?;
        let mut ev = Event::new(self.name.clone(), DateRange { start, end });
        if let Some(d) = &self.description {
            if !d.is_empty() {
                ev.description = Some(d.clone());
            }
        }
        Ok(ev)
    }

    fn update_entity(&self, ev: &mut Event) -> Result<(), String> {
        ev.name = self.name.clone();
        ev.date_range = DateRange {
            start: parse_flexible_date(&self.start_date, "start")?,
            end: parse_flexible_date(&self.end_date, "end")?,
        };
        ev.description = self.description.as_ref().filter(|d| !d.is_empty()).cloned();
        Ok(())
    }
}

#[tauri::command]
pub async fn get_all_events(state: State<'_, EncyclopediaDb>) -> Result<Vec<Event>, String> {
    let rows = state
        .list_entities(Some(EntityType::Event))
        .await
        .map_err(|e| e.to_string())?;
    rows.into_iter()
        .map(|(_name, data)| serde_json::from_str(&data).map_err(|e| e.to_string()))
        .collect()
}

#[tauri::command]
pub async fn get_event(
    state: State<'_, EncyclopediaDb>,
    name: String,
) -> Result<Option<Event>, String> {
    match state
        .get_entity(EntityType::Event, &name)
        .await
        .map_err(|e| e.to_string())?
    {
        Some(data) => serde_json::from_str(&data)
            .map(Some)
            .map_err(|e| e.to_string()),
        None => Ok(None),
    }
}

#[tauri::command]
pub async fn create_event(
    state: State<'_, EncyclopediaDb>,
    vault: State<'_, VaultManager>,
    request: CreateEventRequest,
) -> Result<String, String> {
    handle_create(state, vault, request).await
}

#[tauri::command]
pub async fn update_event(
    state: State<'_, EncyclopediaDb>,
    vault: State<'_, VaultManager>,
    name: String,
    request: CreateEventRequest,
) -> Result<String, String> {
    handle_update(state, vault, EntityType::Event, name, request).await
}
