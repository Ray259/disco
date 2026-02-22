use std::path::{Path, PathBuf};
use std::fs;
use uuid::Uuid;

use crate::core::db::EncyclopediaDb;
use crate::core::domain::traits::DomainEntity;
use crate::core::domain::values::entity_ref::EntityType;
use crate::core::markdown::{
    entity_to_markdown, markdown_to_entity_data, safe_filename, entity_type_dir,
    MarkdownSerializable,
};

/// Manages the Obsidian-compatible vault directory.
///
/// Coordinates between the filesystem (Markdown files) and SQLite (read cache).
/// Markdown files are the source of truth; SQLite is rebuilt from them on startup
/// and kept in sync via the file watcher.
#[derive(Clone)]
pub struct VaultManager {
    pub vault_path: Option<PathBuf>,
    pub app_data_dir: PathBuf,
}

impl VaultManager {
    /// Gets the path to the configuration file
    fn get_config_path(app_data_dir: &Path) -> PathBuf {
        app_data_dir.join("vault_config.json")
    }

    /// Loads the vault path from config. Returns None if not set.
    fn load_config(app_data_dir: &Path) -> Option<PathBuf> {
        let config_path = Self::get_config_path(app_data_dir);
        if let Ok(content) = fs::read_to_string(&config_path) {
            if let Ok(json) = serde_json::from_str::<serde_json::Value>(&content) {
                if let Some(path_str) = json.get("vault_path").and_then(|v| v.as_str()) {
                    let path = PathBuf::from(path_str);
                    if path.exists() && path.is_dir() {
                        return Some(path);
                    }
                }
            }
        }
        None
    }

    /// Saves the current vault path to config
    pub fn save_config(&self) -> Result<(), String> {
        let config_path = Self::get_config_path(&self.app_data_dir);
        let path_str = match &self.vault_path {
            Some(p) => p.to_string_lossy().to_string(),
            None => "".to_string(),
        };
        
        let config = serde_json::json!({
            "vault_path": path_str
        });
        
        // Ensure directory exists
        if let Some(parent) = config_path.parent() {
            let _ = fs::create_dir_all(parent);
        }
        
        fs::write(&config_path, serde_json::to_string_pretty(&config).unwrap())
            .map_err(|e| format!("Failed to save config: {}", e))
    }

    /// Creates a new VaultManager. 
    /// If vault_path is None, loads from config or remains None.
    pub fn new(app_data_dir: PathBuf, explicit_vault_path: Option<PathBuf>) -> Result<Self, String> {
        let vault_path = explicit_vault_path.or_else(|| Self::load_config(&app_data_dir));
        
        // Create config directory if needed
        let _ = fs::create_dir_all(&app_data_dir);
        
        // Create subdirectories for each entity type ONLY if path is set
        if let Some(vp) = &vault_path {
            fs::create_dir_all(vp)
                .map_err(|e| format!("Failed to create vault directory: {}", e))?;
                
            let entity_types = [
                EntityType::Figure,
                EntityType::Work,
                EntityType::Event,
                EntityType::Geo,
                EntityType::Institution,
                EntityType::SchoolOfThought,
            ];
            
            for et in &entity_types {
                let dir = vp.join(entity_type_dir(et));
                fs::create_dir_all(&dir)
                    .map_err(|e| format!("Failed to create directory {:?}: {}", dir, e))?;
            }
        }
        
        let manager = Self { vault_path, app_data_dir };
        let _ = manager.save_config(); // Ensure config file exists
        Ok(manager)
    }

    /// Performs a full sync on startup: scans all .md files in the vault
    /// and upserts them into the SQLite database.
    pub async fn full_sync(&self, db: &EncyclopediaDb) -> Result<SyncReport, String> {
        let mut report = SyncReport::default();
        
        let vault_path = match &self.vault_path {
            Some(p) => p,
            None => return Ok(report), // Nothing to sync if no vault
        };
        
        // Walk all subdirectories  
        for entry in walkdir(vault_path) {
            if entry.extension().map_or(true, |ext| ext != "md") {
                continue;
            }
            
            match self.sync_single_file(&entry, db).await {
                Ok(_) => report.synced += 1,
                Err(e) => {
                    report.errors.push(format!("{}: {}", entry.display(), e));
                }
            }
        }
        
        Ok(report)
    }

    /// Syncs a single markdown file into SQLite.
    /// Called by both the startup scan and the file watcher.
    pub async fn sync_single_file(&self, path: &Path, db: &EncyclopediaDb) -> Result<Uuid, String> {
        let content = fs::read_to_string(path)
            .map_err(|e| format!("Failed to read {}: {}", path.display(), e))?;
        
        let parsed = markdown_to_entity_data(&content)?;
        let file_path_str = path.to_string_lossy().to_string();
        
        // Upsert into SQLite — this replaces existing data if the UUID matches
        db.upsert_entity(
            parsed.id,
            parsed.entity_type,
            &parsed.name,
            &parsed.data,
            &file_path_str,
        ).await.map_err(|e| e.to_string())?;
        
        // Rebuild relations from the entity's JSON data
        db.clear_outgoing_relations(parsed.id).await.map_err(|e| e.to_string())?;
        
        // Parse relations from the JSON data and re-insert them
        if let Ok(relations) = extract_relations_from_json(&parsed.data) {
            for (target_id, relation_type) in relations {
                let _ = db.insert_relation(parsed.id, target_id, &relation_type).await;
            }
        }
        
        Ok(parsed.id)
    }

    /// Writes an entity to a markdown file in the vault.
    /// Called when the app UI creates or updates an entity.
    pub fn write_entity<E: DomainEntity + MarkdownSerializable>(&self, entity: &E) -> Result<PathBuf, String> {
        let vault_path = match &self.vault_path {
            Some(p) => p,
            None => return Err("No vault directory configured. Please select one in Settings.".into()),
        };
        
        let dir = vault_path.join(entity_type_dir(&entity.entity_type()));
        let filename = format!("{}.md", safe_filename(&entity.name()));
        let path = dir.join(&filename);
        
        let content = entity_to_markdown(entity);
        
        fs::write(&path, &content)
            .map_err(|e| format!("Failed to write {}: {}", path.display(), e))?;
        
        Ok(path)
    }

    /// Deletes an entity's markdown file from the vault.
    /// Looks up the file path from SQLite, then removes it.
    pub async fn delete_entity_file(&self, id: Uuid, db: &EncyclopediaDb) -> Result<(), String> {
        if let Some(file_path) = db.get_entity_file_path(id).await.map_err(|e| e.to_string())? {
            let path = Path::new(&file_path);
            if path.exists() {
                fs::remove_file(path)
                    .map_err(|e| format!("Failed to delete {}: {}", path.display(), e))?;
            }
        }
        Ok(())
    }

    /// Handles a file deletion event from the watcher.
    /// Removes the entity from SQLite based on the file path.
    pub async fn handle_file_deleted(&self, path: &Path, db: &EncyclopediaDb) -> Result<(), String> {
        let file_path_str = path.to_string_lossy().to_string();
        db.delete_entity_by_file_path(&file_path_str).await.map_err(|e| e.to_string())?;
        Ok(())
    }

    /// Exports all existing entities from SQLite to markdown files (migration).
    /// Used for first-time migration from DB-only to vault mode.
    pub async fn export_all_from_db(&self, db: &EncyclopediaDb) -> Result<u32, String> {
        use crate::core::domain::models::figure::Figure;
        use crate::core::domain::models::event::Event;
        use crate::core::domain::models::institution::Institution;
        use crate::core::domain::models::work::Work;
        use crate::core::domain::models::geo::Geo;
        use crate::core::domain::models::school_of_thought::SchoolOfThought;

        let mut count = 0u32;
        
        let all_entities = db.list_entities(None).await.map_err(|e| e.to_string())?;
        
        for (id, _name, _data) in all_entities {
            // We need to determine entity type from the data
            if let Ok(entity_row) = db.get_entity(id).await {
                if let Some((type_str, _name, data)) = entity_row {
                    let result = match type_str.as_str() {
                        "Figure" => {
                            if let Ok(e) = serde_json::from_str::<Figure>(&data) {
                                self.write_entity(&e)
                            } else { continue; }
                        }
                        "Event" => {
                            if let Ok(e) = serde_json::from_str::<Event>(&data) {
                                self.write_entity(&e)
                            } else { continue; }
                        }
                        "Institution" => {
                            if let Ok(e) = serde_json::from_str::<Institution>(&data) {
                                self.write_entity(&e)
                            } else { continue; }
                        }
                        "Work" => {
                            if let Ok(e) = serde_json::from_str::<Work>(&data) {
                                self.write_entity(&e)
                            } else { continue; }
                        }
                        "Geo" => {
                            if let Ok(e) = serde_json::from_str::<Geo>(&data) {
                                self.write_entity(&e)
                            } else { continue; }
                        }
                        "School of Thought" | "SchoolOfThought" => {
                            if let Ok(e) = serde_json::from_str::<SchoolOfThought>(&data) {
                                self.write_entity(&e)
                            } else { continue; }
                        }
                        _ => continue,
                    };
                    
                    if result.is_ok() {
                        count += 1;
                    }
                }
            }
        }
        
        Ok(count)
    }
}

// ─── Helpers ─────────────────────────────────────────────────────────────────

/// Sync status report.
#[derive(Default)]
pub struct SyncReport {
    pub synced: u32,
    pub errors: Vec<String>,
}

/// Recursively walks a directory and collects all file paths.
fn walkdir(dir: &Path) -> Vec<PathBuf> {
    let mut files = Vec::new();
    if let Ok(entries) = fs::read_dir(dir) {
        for entry in entries.flatten() {
            let path = entry.path();
            if path.is_dir() {
                files.extend(walkdir(&path));
            } else {
                files.push(path);
            }
        }
    }
    files
}

/// Extracts relation target IDs and types from the JSON serialized entity data.
/// This is a generic approach that works for the `relations` field present on all entities.
fn extract_relations_from_json(json_data: &str) -> Result<Vec<(Uuid, String)>, String> {
    let val: serde_json::Value = serde_json::from_str(json_data).map_err(|e| e.to_string())?;
    let mut results = Vec::new();
    
    if let Some(relations) = val.get("relations").and_then(|v| v.as_array()) {
        for rel in relations {
            if let (Some(target), Some(kind)) = (rel.get("target"), rel.get("kind")) {
                if let Some(entity_id) = target.get("entity_id").and_then(|v| v.as_str()) {
                    if let Ok(uuid) = Uuid::parse_str(entity_id) {
                        let relation_type = match kind {
                            serde_json::Value::Object(map) => {
                                if let Some(custom) = map.get("Custom").and_then(|v| v.as_str()) {
                                    custom.to_string()
                                } else if let Some(fixed) = map.get("Fixed").and_then(|v| v.as_str()) {
                                    fixed.to_string()
                                } else {
                                    "Unknown".to_string()
                                }
                            }
                            _ => "Unknown".to_string(),
                        };
                        results.push((uuid, relation_type));
                    }
                }
            }
        }
    }
    
    Ok(results)
}
