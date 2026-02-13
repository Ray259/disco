use tauri::State;
use uuid::Uuid;
use crate::core::db::EncyclopediaDb;
use crate::core::domain::models::work::Work;
use crate::core::domain::values::entity_ref::EntityType;
use crate::core::domain::values::rich_content::RichContent;
use serde::Deserialize;

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
