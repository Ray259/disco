use tauri::State;
use uuid::Uuid;
use crate::core::db::EncyclopediaDb;
use crate::core::domain::models::figure::Figure;
use crate::core::domain::values::entity_ref::EntityType;
use crate::core::domain::values::rich_content::RichContent;
use crate::core::domain::values::date_range::DateRange;
use serde::Deserialize;
use chrono::NaiveDate;

// Helper to convert DB result to Figure
fn parse_figure(_id: Uuid, _name: String, data: String) -> Result<Figure, String> {
    serde_json::from_str(&data).map_err(|e| e.to_string())
}

#[derive(Deserialize)]
pub struct CreateFigureRequest {
    name: String,
    role: String,
    location: String,
    start_year: String,
    end_year: String,
    quote: Option<String>,
}

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

#[tauri::command]
pub async fn create_figure(state: State<'_, EncyclopediaDb>, request: CreateFigureRequest) -> Result<String, String> {
    let id = Uuid::new_v4();
    
    let role_content = RichContent::from_text(&request.role);
    let location_content = RichContent::from_text(&request.location);
    
    let start_date = NaiveDate::parse_from_str(&format!("{}-01-01", request.start_year), "%Y-%m-%d")
        .map_err(|_| "Invalid start year format".to_string())?;
    let end_date = NaiveDate::parse_from_str(&format!("{}-01-01", request.end_year), "%Y-%m-%d")
        .map_err(|_| "Invalid end year format".to_string())?;

    let life = DateRange {
        start: start_date,
        end: end_date,
    };

    let mut figure = Figure::new(
        id,
        request.name.clone(),
        life,
        role_content,
        location_content
    );

    if let Some(q) = request.quote {
        if !q.is_empty() {
             figure = figure.with_defining_quote(RichContent::from_text(&q));
        }
    }
    
    let data = serde_json::to_string(&figure).map_err(|e| e.to_string())?;
    
    state.insert_entity(
        id,
        EntityType::Figure,
        &figure.name,
        &data
    )
    .await
    .map_err(|e| e.to_string())?;

    Ok(id.to_string())
}
