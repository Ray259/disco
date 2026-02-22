use tauri::State;
use uuid::Uuid;
use crate::core::db::EncyclopediaDb;
use crate::core::vault::VaultManager;
use crate::core::domain::traits::{DomainEntity, InputDto};
use crate::core::markdown::MarkdownSerializable;

/// Generic Create Handler
/// 
/// 1. Converts DTO to Domain Entity.
/// 2. Serializes data.
/// 3. Inserts Entity generic data into SQLite.
/// 4. Inserts Relations.
/// 5. Writes markdown file to vault.
pub async fn handle_create<E, D>(
    state: State<'_, EncyclopediaDb>, 
    vault: State<'_, VaultManager>,
    request: D
) -> Result<String, String>
where
    E: DomainEntity + MarkdownSerializable,
    D: InputDto<E>
{
    let id = Uuid::new_v4();
    let entity = request.to_entity(id)?;
    
    let data = serde_json::to_string(&entity).map_err(|e| e.to_string())?;

    state.insert_entity(
        id,
        entity.entity_type(),
        &entity.name(),
        &data
    )
    .await
    .map_err(|e| e.to_string())?;

    if let Some(relations) = request.get_relations() {
        for rel in relations {
            state.insert_relation(id, rel.target_id, &rel.relation_type)
                .await
                .map_err(|e| e.to_string())?;
        }
    }

    // Write markdown file to vault
    match vault.write_entity(&entity) {
        Ok(path) => {
            // Update the file_path in SQLite
            let file_path_str = path.to_string_lossy().to_string();
            let _ = state.upsert_entity(
                id,
                entity.entity_type(),
                &entity.name(),
                &data,
                &file_path_str,
            ).await;
        }
        Err(e) => {
            eprintln!("[vault] Failed to write entity file: {}", e);
        }
    }

    Ok(id.to_string())
}

/// Generic Update Handler
///
/// 1. Fetches existing data.
/// 2. Deserializes to Domain Entity.
/// 3. Applies updates from DTO.
/// 4. Serializes and updates generic data in SQLite.
/// 5. Replaces relations.
/// 6. Updates markdown file in vault.
pub async fn handle_update<E, D>(
    state: State<'_, EncyclopediaDb>,
    vault: State<'_, VaultManager>,
    id: Uuid,
    request: D
) -> Result<String, String>
where
    E: DomainEntity + MarkdownSerializable,
    D: InputDto<E>
{
    // 1. Fetch
    let (_, _, existing_data) = state.get_entity(id).await
        .map_err(|e| e.to_string())?
        .ok_or("Entity not found")?;
    
    // 2. Deserialize
    let mut entity: E = serde_json::from_str(&existing_data).map_err(|e| e.to_string())?;

    // 3. Update
    request.update_entity(&mut entity)?;
    entity.set_updated_at(chrono::Utc::now());

    // 4. Save to SQLite
    let data = serde_json::to_string(&entity).map_err(|e| e.to_string())?;
    state.update_entity(id, &entity.name(), &data).await.map_err(|e| e.to_string())?;

    // 5. Relations
    state.clear_outgoing_relations(id).await.map_err(|e| e.to_string())?;
    
    if let Some(relations) = request.get_relations() {
        for rel in relations {
            state.insert_relation(id, rel.target_id, &rel.relation_type)
                .await
                .map_err(|e| e.to_string())?;
        }
    }

    // 6. Write updated markdown file to vault 
    // First delete old file if the name changed (filename derives from name)
    let _ = vault.delete_entity_file(id, &state).await;
    match vault.write_entity(&entity) {
        Ok(path) => {
            let file_path_str = path.to_string_lossy().to_string();
            let _ = state.upsert_entity(
                id,
                entity.entity_type(),
                &entity.name(),
                &data,
                &file_path_str,
            ).await;
        }
        Err(e) => {
            eprintln!("[vault] Failed to update entity file: {}", e);
        }
    }

    Ok(id.to_string())
}

/// Deletes an entity by its ID.
/// Also removes the corresponding markdown file from the vault.
#[tauri::command]
pub async fn delete_entity(
    state: State<'_, EncyclopediaDb>,
    vault: State<'_, VaultManager>,
    id: String
) -> Result<String, String> {
    let uuid = Uuid::parse_str(&id).map_err(|_| "Invalid UUID".to_string())?;
    
    // Delete vault file first (needs the file_path from DB)
    if let Err(e) = vault.delete_entity_file(uuid, &state).await {
        eprintln!("[vault] Failed to delete entity file: {}", e);
    }

    match state.delete_entity(uuid).await {
        Ok(count) => {
            if count > 0 {
                Ok(format!("Deleted {} entity", count))
            } else {
                Err("Entity not found".to_string())
            }
        },
        Err(e) => Err(e.to_string())
    }
}
