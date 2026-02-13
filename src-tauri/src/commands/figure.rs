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

/// DTO for creating a new Figure. 
///
/// Front-end sends this structure. `relations` is optional and handled after
/// the main entity insertion.
#[derive(Deserialize)]
pub struct CreateFigureRequest {
    pub name: String,
    pub role: String,
    pub location: String,
    pub start_year: String,
    pub end_year: String,
    pub quote: Option<String>,
    pub relations: Option<Vec<crate::commands::RelationDto>>,
}

/// Retrieves all entities with type `Figure`.
///
/// Returns a list of full `Figure` domain objects.
/// Note: This fetches all data, which might be heavy if the DB grows large.
/// Pagination should be considered in the future.
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

/// Creates a new Figure and persists it to the database.
///
/// 1. Generates a new UUID.
/// 2. Parses dates (handles "YYYY" and "YYYY-MM-DD").
/// 3. Constructs the `Figure` domain object.
/// 4. Serializes to JSON and inserts into `entities`.
/// 5. Inserts any provided `relations` into the `relations` table.
#[tauri::command]
pub async fn create_figure(state: State<'_, EncyclopediaDb>, request: CreateFigureRequest) -> Result<String, String> {
    let id = Uuid::new_v4();
    
    let role_content = RichContent::from_text(&request.role);
    let location_content = RichContent::from_text(&request.location);
    
    let start_date = NaiveDate::parse_from_str(&request.start_year, "%Y-%m-%d")
        .or_else(|_| NaiveDate::parse_from_str(&format!("{}-01-01", request.start_year), "%Y-%m-%d"))
        .map_err(|_| "Invalid start year format".to_string())?;
    let end_date = NaiveDate::parse_from_str(&request.end_year, "%Y-%m-%d")
        .or_else(|_| NaiveDate::parse_from_str(&format!("{}-01-01", request.end_year), "%Y-%m-%d"))
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
