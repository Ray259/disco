use tauri::State;
use crate::core::db::EncyclopediaDb;
use crate::core::vault::VaultManager;
use crate::core::domain::models::figure::Figure;
use crate::core::domain::values::entity_ref::EntityType;
use crate::core::domain::values::rich_content::RichContent;
use crate::core::domain::values::date_range::DateRange;
use crate::core::domain::traits::InputDto;
use serde::Deserialize;
use super::common::{handle_create, handle_update, parse_flexible_date};

#[derive(Deserialize)]
pub struct CreateFigureRequest {
    pub name: String,
    pub role: RichContent,
    pub location: RichContent,
    pub start_year: String,
    pub end_year: String,
    pub quote: Option<RichContent>,
}

impl InputDto<Figure> for CreateFigureRequest {
    fn to_entity(&self) -> Result<Figure, String> {
        let start = parse_flexible_date(&self.start_year, "start")?;
        let end = parse_flexible_date(&self.end_year, "end")?;
        let mut fig = Figure::new(self.name.clone(), DateRange { start, end }, self.role.clone(), self.location.clone());
        if let Some(q) = &self.quote { if !q.is_empty() { fig = fig.with_defining_quote(q.clone()); } }
        Ok(fig)
    }

    fn update_entity(&self, fig: &mut Figure) -> Result<(), String> {
        fig.name = self.name.clone();
        fig.primary_role = self.role.clone();
        fig.primary_location = self.location.clone();
        fig.life = DateRange {
            start: parse_flexible_date(&self.start_year, "start")?,
            end: parse_flexible_date(&self.end_year, "end")?,
        };
        fig.defining_quote = self.quote.as_ref().filter(|q| !q.is_empty()).cloned();
        Ok(())
    }
}

#[tauri::command]
pub async fn get_all_figures(state: State<'_, EncyclopediaDb>) -> Result<Vec<Figure>, String> {
    let rows = state.list_entities(Some(EntityType::Figure)).await.map_err(|e| e.to_string())?;
    rows.into_iter().map(|(_name, data)| serde_json::from_str(&data).map_err(|e| e.to_string())).collect()
}

#[tauri::command]
pub async fn get_figure(state: State<'_, EncyclopediaDb>, name: String) -> Result<Option<Figure>, String> {
    match state.get_entity(EntityType::Figure, &name).await.map_err(|e| e.to_string())? {
        Some(data) => serde_json::from_str(&data).map(Some).map_err(|e| e.to_string()),
        None => Ok(None),
    }
}

#[tauri::command]
pub async fn create_figure(state: State<'_, EncyclopediaDb>, vault: State<'_, VaultManager>, request: CreateFigureRequest) -> Result<String, String> {
    handle_create(state, vault, request).await
}

#[tauri::command]
pub async fn update_figure(state: State<'_, EncyclopediaDb>, vault: State<'_, VaultManager>, name: String, request: CreateFigureRequest) -> Result<String, String> {
    handle_update(state, vault, EntityType::Figure, name, request).await
}
