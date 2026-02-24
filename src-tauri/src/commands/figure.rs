use tauri::State;
use uuid::Uuid;
use crate::core::db::EncyclopediaDb;
use crate::core::vault::VaultManager;
use crate::core::domain::models::figure::Figure;
use crate::core::domain::values::entity_ref::EntityType;
use crate::core::domain::values::rich_content::RichContent;
use crate::core::domain::values::date_range::DateRange;
use crate::core::domain::traits::InputDto;
use serde::Deserialize;
use chrono::NaiveDate;
use super::RelationDto;
use super::common::{handle_create, handle_update, parse_flexible_date};

// Helper to convert DB result to Figure
fn parse_figure(id: Uuid, _name: String, data: String) -> Result<Figure, String> {
    let mut figure: Figure = serde_json::from_str(&data).map_err(|e| e.to_string())?;
    figure.id = id; // Ensure ID matches the DB row ID
    Ok(figure)
}

/// DTO for creating a new Figure. 
#[derive(Deserialize)]
pub struct CreateFigureRequest {
    pub name: String,
    pub role: String,
    pub location: String,
    pub start_year: String,
    pub end_year: String,
    pub quote: Option<String>,
    pub relations: Option<Vec<RelationDto>>,
}

impl InputDto<Figure> for CreateFigureRequest {
    fn to_entity(&self, id: Uuid) -> Result<Figure, String> {
        let role_content = RichContent::from_text(&self.role);
        let location_content = RichContent::from_text(&self.location);
        
        let start_date = parse_flexible_date(&self.start_year, "start")?;
        let end_date = parse_flexible_date(&self.end_year, "end")?;

        let life = DateRange {
            start: start_date,
            end: end_date,
        };

        let mut figure = Figure::new(
            id,
            self.name.clone(),
            life,
            role_content,
            location_content
        );

        if let Some(q) = &self.quote {
            if !q.is_empty() {
                 figure = figure.with_defining_quote(RichContent::from_text(q));
            }
        }
        
        Ok(figure)
    }

    fn update_entity(&self, figure: &mut Figure) -> Result<(), String> {
        figure.name = self.name.clone();
        figure.primary_role = RichContent::from_text(&self.role);
        figure.primary_location = RichContent::from_text(&self.location);
        
        let start_date = parse_flexible_date(&self.start_year, "start")?;
        let end_date = parse_flexible_date(&self.end_year, "end")?;

        figure.life = DateRange { start: start_date, end: end_date };
        
        if let Some(q) = &self.quote {
             if !q.is_empty() {
                  figure.defining_quote = Some(RichContent::from_text(q));
             } else {
                  figure.defining_quote = None;
             }
        } else {
             figure.defining_quote = None;
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

/// Retrieves all entities with type `Figure`.
#[tauri::command]
pub async fn get_all_figures(state: State<'_, EncyclopediaDb>) -> Result<Vec<Figure>, String> {
    let entities = state.list_entities(Some(EntityType::Figure))
        .await
        .map_err(|e| e.to_string())?;
    
    let figures: Result<Vec<Figure>, String> = entities.into_iter()
        .map(|(_id, _name, data)| parse_figure(_id, _name, data))
        .collect();
        
    figures
}

/// Retrieves a single Figure by ID.
#[tauri::command]
pub async fn get_figure(state: State<'_, EncyclopediaDb>, id: Uuid) -> Result<Option<Figure>, String> {
    let result = state.get_entity(id)
        .await
        .map_err(|e| e.to_string())?;
    
    match result {
        Some((_type_str, name, data)) => {
            parse_figure(id, name, data).map(Some)
        },
        None => Ok(None)
    }
}

/// Creates a new Figure and persists it.
#[tauri::command]
pub async fn create_figure(state: State<'_, EncyclopediaDb>, vault: State<'_, VaultManager>, request: CreateFigureRequest) -> Result<String, String> {
    handle_create(state, vault, request).await
}

#[tauri::command]
pub async fn update_figure(state: State<'_, EncyclopediaDb>, vault: State<'_, VaultManager>, id: Uuid, request: CreateFigureRequest) -> Result<String, String> {
    handle_update(state, vault, id, request).await
}
