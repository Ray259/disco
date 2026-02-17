use tauri::State;
use uuid::Uuid;
use crate::core::db::EncyclopediaDb;
use crate::core::domain::traits::{DomainEntity, InputDto};

/// Generic Create Handler
/// 
/// 1. Converts DTO to Domain Entity.
/// 2. Serializes data.
/// 3. Inserts Entity generic data.
/// 4. Inserts Relations.
pub async fn handle_create<E, D>(
    state: State<'_, EncyclopediaDb>, 
    request: D
) -> Result<String, String>
where
    E: DomainEntity,
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

    Ok(id.to_string())
}

/// Generic Update Handler
///
/// 1. Fetches existing data.
/// 2. Deserializes to Domain Entity.
/// 3. Applies updates from DTO.
/// 4. Serializes and updates generic data.
/// 5. Replaces relations.
pub async fn handle_update<E, D>(
    state: State<'_, EncyclopediaDb>,
    id: Uuid,
    request: D
) -> Result<String, String>
where
    E: DomainEntity,
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

    // 4. Save
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

    Ok(id.to_string())
}

/// Deletes an entity by its ID.
#[tauri::command]
pub async fn delete_entity(state: State<'_, EncyclopediaDb>, id: String) -> Result<String, String> {
    let uuid = Uuid::parse_str(&id).map_err(|_| "Invalid UUID".to_string())?;
    
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
