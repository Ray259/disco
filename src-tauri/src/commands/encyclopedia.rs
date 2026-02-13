use tauri::{State, AppHandle};
use uuid::Uuid;
use crate::core::db::EncyclopediaDb;
use crate::core::domain::models::figure::Figure;
use crate::core::domain::models::institution::Institution;
use crate::core::domain::models::event::Event;
use crate::core::domain::models::geo::Geo;
use crate::core::domain::models::work::Work;
use crate::core::domain::values::entity_ref::EntityType;
use crate::core::domain::values::rich_content::RichContent;
use crate::core::domain::values::date_range::DateRange;
use serde::Deserialize;
use chrono::NaiveDate;

// ... (existing parse_figure, get_all_figures, get_figure, etc.)

// ... (existing create_figure, create_institution code)

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
    
    let start = NaiveDate::parse_from_str(&request.start_date, "%Y-%m-%d") // Expect full date or handle partial logic
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

#[derive(Deserialize)]
pub struct CreateGeoRequest {
    name: String,
    region: Option<String>,
    description: Option<String>,
}

#[tauri::command]
pub async fn create_geo(state: State<'_, EncyclopediaDb>, request: CreateGeoRequest) -> Result<String, String> {
    let id = Uuid::new_v4();
    let mut geo = Geo::new(id, request.name.clone());

    if let Some(reg) = request.region {
        if !reg.is_empty() {
            geo = geo.with_region(RichContent::from_text(&reg));
        }
    }
    if let Some(desc) = request.description {
        if !desc.is_empty() {
             geo = geo.with_description(RichContent::from_text(&desc));
        }
    }

    let data = serde_json::to_string(&geo).map_err(|e| e.to_string())?;

    state.insert_entity(id, EntityType::Geo, &geo.name, &data)
        .await
        .map_err(|e| e.to_string())?;

    Ok(id.to_string())
}

#[derive(Deserialize)]
pub struct CreateWorkRequest {
    title: String,
    summary: Option<String>,
}

#[tauri::command]
pub async fn create_work(state: State<'_, EncyclopediaDb>, request: CreateWorkRequest) -> Result<String, String> {
    let id = Uuid::new_v4();
    let mut work = Work::new(id, request.title.clone());

    if let Some(sum) = request.summary {
         if !sum.is_empty() {
             work = work.with_summary(RichContent::from_text(&sum));
         }
    }

    let data = serde_json::to_string(&work).map_err(|e| e.to_string())?;

    state.insert_entity(id, EntityType::Work, &work.title, &data)
        .await
        .map_err(|e| e.to_string())?;

    Ok(id.to_string())
}

// Helper to convert DB result to Figure (placeholders for now as DB returns JSON string)
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
            // check type if needed, but ID should be unique
            parse_figure(id, name, data).map(Some)
        },
        None => Ok(None)
    }
}

#[tauri::command]
pub async fn create_figure(state: State<'_, EncyclopediaDb>, request: CreateFigureRequest) -> Result<String, String> {
    let id = Uuid::new_v4();
    
    // Construct rich content from simple strings
    let role_content = RichContent::from_text(&request.role);
    let location_content = RichContent::from_text(&request.location);
    
    // Parse years (basic implementation)
    // Default to Jan 1st if only year provided
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
