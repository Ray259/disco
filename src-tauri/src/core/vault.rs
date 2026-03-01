use std::path::{Path, PathBuf};
use std::fs;

use crate::core::db::EncyclopediaDb;
use crate::core::domain::traits::DomainEntity;
use crate::core::domain::values::entity_ref::EntityType;
use crate::core::markdown::{
    entity_to_markdown, markdown_to_entity_data, safe_filename, entity_type_dir,
    MarkdownSerializable,
};

#[derive(Clone)]
pub struct VaultManager {
    pub vault_path: Option<PathBuf>,
    pub app_data_dir: PathBuf,
}

impl VaultManager {
    fn get_config_path(app_data_dir: &Path) -> PathBuf {
        app_data_dir.join("vault_config.json")
    }

    fn load_config(app_data_dir: &Path) -> Option<PathBuf> {
        let config_path = Self::get_config_path(app_data_dir);
        if let Ok(content) = fs::read_to_string(&config_path) {
            if let Ok(json) = serde_json::from_str::<serde_json::Value>(&content) {
                if let Some(path_str) = json.get("vault_path").and_then(|v| v.as_str()) {
                    let path = PathBuf::from(path_str);
                    if path.exists() && path.is_dir() { return Some(path); }
                }
            }
        }
        None
    }

    pub fn save_config(&self) -> Result<(), String> {
        let config_path = Self::get_config_path(&self.app_data_dir);
        let path_str = self.vault_path.as_ref().map(|p| p.to_string_lossy().to_string()).unwrap_or_default();
        let config = serde_json::json!({ "vault_path": path_str });
        if let Some(parent) = config_path.parent() { let _ = fs::create_dir_all(parent); }
        fs::write(&config_path, serde_json::to_string_pretty(&config).unwrap())
            .map_err(|e| format!("Failed to save config: {}", e))
    }

    pub fn new(app_data_dir: PathBuf, explicit_vault_path: Option<PathBuf>) -> Result<Self, String> {
        let vault_path = explicit_vault_path.or_else(|| Self::load_config(&app_data_dir));
        let _ = fs::create_dir_all(&app_data_dir);
        if let Some(vp) = &vault_path {
            fs::create_dir_all(vp).map_err(|e| format!("Failed to create vault: {}", e))?;
            let types = [EntityType::Figure, EntityType::Work, EntityType::Event,
                         EntityType::Geo, EntityType::Institution, EntityType::SchoolOfThought];
            for et in &types {
                let dir = vp.join(entity_type_dir(et));
                fs::create_dir_all(&dir).map_err(|e| format!("Failed to create {:?}: {}", dir, e))?;
            }
        }
        let manager = Self { vault_path, app_data_dir };
        let _ = manager.save_config();
        Ok(manager)
    }

    pub async fn full_sync(&self, db: &EncyclopediaDb) -> Result<SyncReport, String> {
        let mut report = SyncReport::default();
        let vault_path = match &self.vault_path {
            Some(p) => p,
            None => return Ok(report),
        };
        for entry in walkdir(vault_path) {
            if entry.extension().map_or(true, |ext| ext != "md") { continue; }
            match self.sync_single_file(&entry, db).await {
                Ok(_) => report.synced += 1,
                Err(e) => report.errors.push(format!("{}: {}", entry.display(), e)),
            }
        }
        Ok(report)
    }

    pub async fn sync_single_file(&self, path: &Path, db: &EncyclopediaDb) -> Result<(), String> {
        let content = fs::read_to_string(path)
            .map_err(|e| format!("Failed to read {}: {}", path.display(), e))?;
        let parsed = markdown_to_entity_data(&content)?;
        let file_path_str = path.to_string_lossy().to_string();
        db.upsert_entity(parsed.entity_type, &parsed.name, &parsed.data, &file_path_str)
            .await.map_err(|e| e.to_string())?;
        Ok(())
    }

    pub fn write_entity<E: DomainEntity + MarkdownSerializable>(&self, entity: &E) -> Result<PathBuf, String> {
        let vault_path = self.vault_path.as_ref()
            .ok_or("No vault directory configured. Please select one in Settings.")?;
        let dir = vault_path.join(entity_type_dir(&entity.entity_type()));
        let filename = format!("{}.md", safe_filename(&entity.name()));
        let path = dir.join(&filename);
        let content = entity_to_markdown(entity);
        fs::write(&path, &content).map_err(|e| format!("Failed to write {}: {}", path.display(), e))?;
        Ok(path)
    }

    pub async fn delete_entity_file(&self, entity_type: EntityType, name: &str, db: &EncyclopediaDb) -> Result<(), String> {
        if let Some(file_path) = db.get_entity_file_path(entity_type, name).await.map_err(|e| e.to_string())? {
            let path = Path::new(&file_path);
            if path.exists() {
                fs::remove_file(path).map_err(|e| format!("Failed to delete {}: {}", path.display(), e))?;
            }
        }
        Ok(())
    }

    pub async fn handle_file_deleted(&self, path: &Path, db: &EncyclopediaDb) -> Result<(), String> {
        let file_path_str = path.to_string_lossy().to_string();
        db.delete_entity_by_file_path(&file_path_str).await.map_err(|e| e.to_string())?;
        Ok(())
    }

    pub async fn export_all_from_db(&self, db: &EncyclopediaDb) -> Result<u32, String> {
        use crate::core::domain::models::figure::Figure;
        use crate::core::domain::models::event::Event;
        use crate::core::domain::models::institution::Institution;
        use crate::core::domain::models::work::Work;
        use crate::core::domain::models::geo::Geo;
        use crate::core::domain::models::school_of_thought::SchoolOfThought;

        let mut count = 0u32;
        let all = db.search_entities("").await.map_err(|e| e.to_string())?;
        for (type_str, _name, data) in all {
            let result = match type_str.as_str() {
                "Figure" => serde_json::from_str::<Figure>(&data).ok().map(|e| self.write_entity(&e)),
                "Event" => serde_json::from_str::<Event>(&data).ok().map(|e| self.write_entity(&e)),
                "Institution" => serde_json::from_str::<Institution>(&data).ok().map(|e| self.write_entity(&e)),
                "Work" => serde_json::from_str::<Work>(&data).ok().map(|e| self.write_entity(&e)),
                "Geo" => serde_json::from_str::<Geo>(&data).ok().map(|e| self.write_entity(&e)),
                "SchoolOfThought" => serde_json::from_str::<SchoolOfThought>(&data).ok().map(|e| self.write_entity(&e)),
                _ => None,
            };
            if let Some(Ok(_)) = result { count += 1; }
        }
        Ok(count)
    }
}

#[derive(Default)]
pub struct SyncReport {
    pub synced: u32,
    pub errors: Vec<String>,
}

fn walkdir(dir: &Path) -> Vec<PathBuf> {
    let mut files = Vec::new();
    if let Ok(entries) = fs::read_dir(dir) {
        for entry in entries.flatten() {
            let path = entry.path();
            if path.is_dir() { files.extend(walkdir(&path)); } else { files.push(path); }
        }
    }
    files
}
