use tauri::State;
use crate::core::db::EncyclopediaDb;
use crate::core::vault::VaultManager;
use crate::core::domain::traits::{DomainEntity, InputDto};
use crate::core::markdown::MarkdownSerializable;
use chrono::NaiveDate;
use crate::core::domain::values::entity_ref::EntityType;

pub fn parse_flexible_date(s: &str, field: &str) -> Result<NaiveDate, String> {
    NaiveDate::parse_from_str(s, "%Y-%m-%d")
        .or_else(|_| NaiveDate::parse_from_str(&format!("{}-01", s), "%Y-%m-%d"))
        .or_else(|_| NaiveDate::parse_from_str(&format!("{}-01-01", s), "%Y-%m-%d"))
        .map_err(|_| format!("Invalid date for {field}: expected YYYY, YYYY-MM, or YYYY-MM-DD"))
}


/// Generic orchestration for entity creation.
/// Performs three steps:
/// 1. Materializes entity from DTO.
/// 2. Persists JSON blob to SQLite.
/// 3. Commits Obsidian-compatible Markdown to the Vault.
#[tracing::instrument(skip(state, vault, request))]
pub async fn handle_create<E, D>(
    state: State<'_, EncyclopediaDb>,
    vault: State<'_, VaultManager>,
    request: D
) -> Result<String, String>
where
    E: DomainEntity + MarkdownSerializable,
    D: InputDto<E>
{
    tracing::info!("Handling create request");
    let entity = request.to_entity()?;
    let name = entity.name();
    let data = serde_json::to_string(&entity).map_err(|e| e.to_string())?;

    state.insert_entity(entity.entity_type(), &name, &data)
        .await.map_err(|e| e.to_string())?;

    match vault.write_entity(&entity) {
        Ok(path) => {
            let fp = path.to_string_lossy().to_string();
            let _ = state.upsert_entity(entity.entity_type(), &name, &data, &fp).await;
        }
        Err(e) => tracing::error!("[vault] Failed to write: {}", e),
    }

    Ok(name)
}

/// Generic orchestration for entity updates.
/// Loads the current state from SQLite, applies the DTO diff, and re-syncs to the Vault.
#[tracing::instrument(skip(state, vault, request))]
pub async fn handle_update<E, D>(
    state: State<'_, EncyclopediaDb>,
    vault: State<'_, VaultManager>,
    entity_type: EntityType,
    name: String,
    request: D
) -> Result<String, String>
where
    E: DomainEntity + MarkdownSerializable,
    D: InputDto<E>
{
    tracing::info!(%entity_type, name, "Handling update request");
    let existing_data = state.get_entity(entity_type, &name).await
        .map_err(|e| e.to_string())?
        .ok_or("Entity not found")?;

    let mut entity: E = serde_json::from_str(&existing_data).map_err(|e| e.to_string())?;
    request.update_entity(&mut entity)?;
    entity.set_updated_at(chrono::Utc::now());

    let new_name = entity.name();
    let data = serde_json::to_string(&entity).map_err(|e| e.to_string())?;

    // If name changed, delete old entry
    if new_name != name {
        tracing::debug!("Entity name changed, re-syncing vault files");
        let _ = vault.delete_entity_file(entity_type, &name, &state).await;
        let _ = state.delete_entity(entity_type, &name).await;
        state.insert_entity(entity_type, &new_name, &data).await.map_err(|e| e.to_string())?;
    } else {
        state.update_entity(entity_type, &name, &data).await.map_err(|e| e.to_string())?;
    }

    match vault.write_entity(&entity) {
        Ok(path) => {
            let fp = path.to_string_lossy().to_string();
            let _ = state.upsert_entity(entity_type, &new_name, &data, &fp).await;
        }
        Err(e) => tracing::error!("[vault] Failed to update: {}", e),
    }

    Ok(new_name)
}

#[tauri::command]
#[tracing::instrument(skip(state, vault))]
pub async fn delete_entity(
    state: State<'_, EncyclopediaDb>,
    vault: State<'_, VaultManager>,
    entity_type: String,
    name: String
) -> Result<String, String> {
    tracing::info!(entity_type, name, "Delete command invoked");
    let et = EntityType::from_str(&entity_type).ok_or("Invalid entity type")?;

    if let Err(e) = vault.delete_entity_file(et, &name, &state).await {
        tracing::error!("[vault] Failed to delete file: {}", e);
    }

    match state.delete_entity(et, &name).await {
        Ok(count) if count > 0 => {
            tracing::info!("Entity deleted successfully");
            Ok(format!("Deleted {}", name))
        },
        Ok(_) => {
            tracing::warn!("Entity not found for deletion");
            Err("Entity not found".to_string())
        },
        Err(e) => {
            tracing::error!("Database error during deletion: {}", e);
            Err(e.to_string())
        },
    }
}
