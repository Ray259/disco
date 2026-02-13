use sqlx::{sqlite::SqlitePoolOptions, Pool, Sqlite};
use std::path::Path;
use uuid::Uuid;
use crate::core::domain::values::entity_ref::EntityType;

#[derive(Clone)]
pub struct EncyclopediaDb {
    pub pool: Pool<Sqlite>,
}

impl EncyclopediaDb {
    pub async fn init(url: &str) -> Result<Self, sqlx::Error> {
        // Ensure the file exists if using sqlite
        // sqlx requires sqlite:// protocol
        let pool = SqlitePoolOptions::new()
            .max_connections(5)
            .connect(url)
            .await?;
            
        let db = Self { pool };
        db.init_schema().await?;
        Ok(db)
    }

    async fn init_schema(&self) -> Result<(), sqlx::Error> {
        sqlx::query(
            "
            CREATE TABLE IF NOT EXISTS entities (
                id TEXT PRIMARY KEY,
                entity_type TEXT NOT NULL,
                name TEXT NOT NULL,
                data TEXT NOT NULL,
                created_at TEXT NOT NULL,
                updated_at TEXT NOT NULL
            );

            CREATE INDEX IF NOT EXISTS idx_entities_type ON entities(entity_type);
            CREATE INDEX IF NOT EXISTS idx_entities_name ON entities(name);

            CREATE TABLE IF NOT EXISTS relations (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                from_id TEXT NOT NULL,
                to_id TEXT NOT NULL,
                relation_type TEXT NOT NULL,
                FOREIGN KEY (from_id) REFERENCES entities(id),
                FOREIGN KEY (to_id) REFERENCES entities(id)
            );

            CREATE INDEX IF NOT EXISTS idx_relations_from ON relations(from_id);
            CREATE INDEX IF NOT EXISTS idx_relations_to ON relations(to_id);
            CREATE INDEX IF NOT EXISTS idx_relations_type ON relations(relation_type);
            "
        )
        .execute(&self.pool)
        .await?;
        Ok(())
    }

    // Entity CRUD
    pub async fn insert_entity(&self, id: Uuid, entity_type: EntityType, name: &str, data: &str) -> Result<(), sqlx::Error> {
        let now = chrono::Utc::now().to_rfc3339();
        sqlx::query(
            "INSERT INTO entities (id, entity_type, name, data, created_at, updated_at) VALUES ($1, $2, $3, $4, $5, $6)"
        )
        .bind(id.to_string())
        .bind(entity_type.to_string())
        .bind(name)
        .bind(data)
        .bind(&now)
        .bind(&now)
        .execute(&self.pool)
        .await?;
        Ok(())
    }

    pub async fn update_entity(&self, id: Uuid, name: &str, data: &str) -> Result<u64, sqlx::Error> {
        let now = chrono::Utc::now().to_rfc3339();
        let result = sqlx::query(
            "UPDATE entities SET name = $1, data = $2, updated_at = $3 WHERE id = $4"
        )
        .bind(name)
        .bind(data)
        .bind(now)
        .bind(id.to_string())
        .execute(&self.pool)
        .await?;
        
        Ok(result.rows_affected())
    }

    pub async fn delete_entity(&self, id: Uuid) -> Result<u64, sqlx::Error> {
        // Transaction manually or just sequential queries
        // Delete relations first
        sqlx::query("DELETE FROM relations WHERE from_id = $1 OR to_id = $1")
            .bind(id.to_string())
            .execute(&self.pool)
            .await?;
            
        let result = sqlx::query("DELETE FROM entities WHERE id = $1")
            .bind(id.to_string())
            .execute(&self.pool)
            .await?;
            
        Ok(result.rows_affected())
    }

    pub async fn get_entity(&self, id: Uuid) -> Result<Option<(String, String, String)>, sqlx::Error> {
        let row: Option<(String, String, String)> = sqlx::query_as(
            "SELECT entity_type, name, data FROM entities WHERE id = $1"
        )
        .bind(id.to_string())
        .fetch_optional(&self.pool)
        .await?;
        
        Ok(row)
    }

    pub async fn list_entities(&self, entity_type: Option<EntityType>) -> Result<Vec<(Uuid, String, String)>, sqlx::Error> {
        let rows: Vec<(String, String, String)> = if let Some(et) = entity_type {
            sqlx::query_as(
                "SELECT id, name, data FROM entities WHERE entity_type = $1 ORDER BY name"
            )
            .bind(et.to_string())
            .fetch_all(&self.pool)
            .await?
        } else {
            sqlx::query_as(
                "SELECT id, name, data FROM entities ORDER BY name"
            )
            .fetch_all(&self.pool)
            .await?
        };

        let mut entries = Vec::new();
        for (id_str, name, data) in rows {
            if let Ok(id) = Uuid::parse_str(&id_str) {
                entries.push((id, name, data));
            }
        }
        Ok(entries)
    }

    // Relation CRUD
    pub async fn insert_relation(&self, from_id: Uuid, to_id: Uuid, relation_type: &str) -> Result<(), sqlx::Error> {
        sqlx::query(
            "INSERT INTO relations (from_id, to_id, relation_type) VALUES ($1, $2, $3)"
        )
        .bind(from_id.to_string())
        .bind(to_id.to_string())
        .bind(relation_type)
        .execute(&self.pool)
        .await?;
        Ok(())
    }

    pub async fn delete_relation(&self, from_id: Uuid, to_id: Uuid, relation_type: &str) -> Result<u64, sqlx::Error> {
        let result = sqlx::query(
            "DELETE FROM relations WHERE from_id = $1 AND to_id = $2 AND relation_type = $3"
        )
        .bind(from_id.to_string())
        .bind(to_id.to_string())
        .bind(relation_type)
        .execute(&self.pool)
        .await?;
        Ok(result.rows_affected())
    }

    pub async fn get_relations_from(&self, from_id: Uuid) -> Result<Vec<(Uuid, String)>, sqlx::Error> {
        let rows: Vec<(String, String)> = sqlx::query_as(
            "SELECT to_id, relation_type FROM relations WHERE from_id = $1"
        )
        .bind(from_id.to_string())
        .fetch_all(&self.pool)
        .await?;

        let mut relations = Vec::new();
        for (id_str, rel_type) in rows {
             if let Ok(id) = Uuid::parse_str(&id_str) {
                relations.push((id, rel_type));
            }
        }
        Ok(relations)
    }

    pub async fn get_relations_to(&self, to_id: Uuid) -> Result<Vec<(Uuid, String)>, sqlx::Error> {
        let rows: Vec<(String, String)> = sqlx::query_as(
            "SELECT from_id, relation_type FROM relations WHERE to_id = $1"
        )
        .bind(to_id.to_string())
        .fetch_all(&self.pool)
        .await?;

        let mut relations = Vec::new();
        for (id_str, rel_type) in rows {
             if let Ok(id) = Uuid::parse_str(&id_str) {
                relations.push((id, rel_type));
            }
        }
        Ok(relations)
    }
}
