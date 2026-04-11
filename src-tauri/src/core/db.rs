use sqlx::{sqlite::SqlitePoolOptions, Pool, Sqlite};
use crate::core::domain::values::entity_ref::EntityType;

/// Manages the SQLite database connection pool for the encyclopedia.
/// Interface for the SQLite backend.
/// Uses `sqlx` for asynchronous query execution.
#[derive(Clone)]
pub struct EncyclopediaDb {
    pub pool: Pool<Sqlite>,
}

impl EncyclopediaDb {
    /// Initializes the database connection pool and ensures the schema is created.
    #[tracing::instrument(skip(url))]
    pub async fn init(url: &str) -> Result<Self, sqlx::Error> {
        tracing::debug!("Initializing database pool...");
        let pool = SqlitePoolOptions::new()
            .max_connections(5)
            .connect(url)
            .await?;
        let db = Self { pool };
        db.init_schema().await?;
        tracing::info!("Database initialized successfully");
        Ok(db)
    }

    /// Executes DDL statements to create the required tables and indexes if they do not exist.
    /// Bootstraps the SQLite schema if it doesn't exist.
    /// Identity is enforced via `PRIMARY KEY (entity_type, name)` on the `entities` table.
    async fn init_schema(&self) -> Result<(), sqlx::Error> {
        sqlx::query(
            "
            CREATE TABLE IF NOT EXISTS entities (
                entity_type TEXT NOT NULL,
                name TEXT NOT NULL,
                data TEXT NOT NULL,
                file_path TEXT,
                created_at TEXT NOT NULL,
                updated_at TEXT NOT NULL,
                PRIMARY KEY (entity_type, name)
            );

            CREATE INDEX IF NOT EXISTS idx_entities_name ON entities(name);

            CREATE TABLE IF NOT EXISTS relations (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                from_type TEXT NOT NULL,
                from_name TEXT NOT NULL,
                to_type TEXT NOT NULL,
                to_name TEXT NOT NULL,
                relation_type TEXT NOT NULL
            );

            CREATE INDEX IF NOT EXISTS idx_relations_from ON relations(from_type, from_name);
            CREATE INDEX IF NOT EXISTS idx_relations_to ON relations(to_type, to_name);
            "
        )
        .execute(&self.pool)
        .await?;
        Ok(())
    }

    /// Purges all records from the entities and relations tables.
    #[tracing::instrument(skip(self))]
    pub async fn empty_database(&self) -> Result<(), sqlx::Error> {
        tracing::info!("Purging all database records");
        sqlx::query("DELETE FROM relations").execute(&self.pool).await?;
        sqlx::query("DELETE FROM entities").execute(&self.pool).await?;
        Ok(())
    }

    /// Inserts a new entity record into the entities table.
    /// Persists an entity's JSON representation.
    /// Overwrites existing entries if the `(entity_type, name)` composite key matches.
    #[tracing::instrument(skip(self, data))]
    pub async fn insert_entity(&self, entity_type: EntityType, name: &str, data: &str) -> Result<(), sqlx::Error> {
        tracing::info!(%entity_type, name, "Inserting new entity");
        let now = chrono::Utc::now().to_rfc3339();
        sqlx::query(
            "INSERT INTO entities (entity_type, name, data, created_at, updated_at) VALUES ($1, $2, $3, $4, $5)"
        )
        .bind(entity_type.to_string())
        .bind(name)
        .bind(data)
        .bind(&now)
        .bind(&now)
        .execute(&self.pool)
        .await?;
        Ok(())
    }

    /// Updates the data payload and modification timestamp for an existing entity.
    #[tracing::instrument(skip(self, data))]
    pub async fn update_entity(&self, entity_type: EntityType, name: &str, data: &str) -> Result<u64, sqlx::Error> {
        tracing::info!(%entity_type, name, "Updating existing entity");
        let now = chrono::Utc::now().to_rfc3339();
        let result = sqlx::query(
            "UPDATE entities SET data = $1, updated_at = $2 WHERE entity_type = $3 AND name = $4"
        )
        .bind(data)
        .bind(now)
        .bind(entity_type.to_string())
        .bind(name)
        .execute(&self.pool)
        .await?;
        Ok(result.rows_affected())
    }

    /// Inserts a new entity or updates an existing entity matched by its primary key.
    #[tracing::instrument(skip(self, data))]
    pub async fn upsert_entity(&self, entity_type: EntityType, name: &str, data: &str, file_path: &str) -> Result<(), sqlx::Error> {
        tracing::debug!(%entity_type, name, file_path, "Upserting entity");
        let now = chrono::Utc::now().to_rfc3339();
        sqlx::query(
            "INSERT OR REPLACE INTO entities (entity_type, name, data, file_path, created_at, updated_at) VALUES ($1, $2, $3, $4, $5, $6)"
        )
        .bind(entity_type.to_string())
        .bind(name)
        .bind(data)
        .bind(file_path)
        .bind(&now)
        .bind(&now)
        .execute(&self.pool)
        .await?;
        Ok(())
    }

    /// Removes an entity and all its associated relations from the database.
    #[tracing::instrument(skip(self))]
    pub async fn delete_entity(&self, entity_type: EntityType, name: &str) -> Result<u64, sqlx::Error> {
        tracing::info!(%entity_type, name, "Deleting entity and relations");
        let et = entity_type.to_string();
        sqlx::query("DELETE FROM relations WHERE (from_type = $1 AND from_name = $2) OR (to_type = $1 AND to_name = $2)")
            .bind(&et).bind(name).execute(&self.pool).await?;
        let result = sqlx::query("DELETE FROM entities WHERE entity_type = $1 AND name = $2")
            .bind(&et).bind(name).execute(&self.pool).await?;
        Ok(result.rows_affected())
    }

    /// Retrieves the JSON data payload for an entity by its type and name.
    pub async fn get_entity(&self, entity_type: EntityType, name: &str) -> Result<Option<String>, sqlx::Error> {
        let row: Option<(String,)> = sqlx::query_as(
            "SELECT data FROM entities WHERE entity_type = $1 AND name = $2"
        )
        .bind(entity_type.to_string())
        .bind(name)
        .fetch_optional(&self.pool)
        .await?;
        Ok(row.map(|(d,)| d))
    }

    /// Retrieves the local filesystem path associated with an entity.
    pub async fn get_entity_file_path(&self, entity_type: EntityType, name: &str) -> Result<Option<String>, sqlx::Error> {
        let row: Option<(String,)> = sqlx::query_as(
            "SELECT file_path FROM entities WHERE entity_type = $1 AND name = $2"
        )
        .bind(entity_type.to_string())
        .bind(name)
        .fetch_optional(&self.pool)
        .await?;
        Ok(row.map(|(p,)| p))
    }

    /// Deletes an entity and its relations based on its associated filesystem path.
    pub async fn delete_entity_by_file_path(&self, file_path: &str) -> Result<u64, sqlx::Error> {
        let row: Option<(String, String)> = sqlx::query_as(
            "SELECT entity_type, name FROM entities WHERE file_path = $1"
        )
        .bind(file_path)
        .fetch_optional(&self.pool)
        .await?;

        if let Some((et, nm)) = &row {
            sqlx::query("DELETE FROM relations WHERE (from_type = $1 AND from_name = $2) OR (to_type = $1 AND to_name = $2)")
                .bind(et).bind(nm).execute(&self.pool).await?;
        }

        let result = sqlx::query("DELETE FROM entities WHERE file_path = $1")
            .bind(file_path)
            .execute(&self.pool)
            .await?;
        Ok(result.rows_affected())
    }

    /// Returns a list of (name, data) pairs for all entities, optionally filtered by type.
    pub async fn list_entities(&self, entity_type: Option<EntityType>) -> Result<Vec<(String, String)>, sqlx::Error> {
        let rows: Vec<(String, String)> = if let Some(et) = entity_type {
            sqlx::query_as("SELECT name, data FROM entities WHERE entity_type = $1 ORDER BY name")
                .bind(et.to_string())
                .fetch_all(&self.pool)
                .await?
        } else {
            sqlx::query_as("SELECT name, data FROM entities ORDER BY name")
                .fetch_all(&self.pool)
                .await?
        };
        Ok(rows)
    }

    /// Performs a case-insensitive `LIKE` pattern match on the name field.
    /// Returns a maximum of 20 results to maintain performance.
    pub async fn search_entities(&self, query: &str) -> Result<Vec<(String, String, String)>, sqlx::Error> {
        let pattern = format!("%{}%", query);
        let rows: Vec<(String, String, String)> = sqlx::query_as(
            "SELECT entity_type, name, data FROM entities WHERE name LIKE $1 ORDER BY name LIMIT 20"
        )
        .bind(pattern)
        .fetch_all(&self.pool)
        .await?;
        Ok(rows)
    }

    /// Inserts a directed relationship between two entities.
    pub async fn insert_relation(&self, from_type: EntityType, from_name: &str, to_type: EntityType, to_name: &str, relation_type: &str) -> Result<(), sqlx::Error> {
        sqlx::query(
            "INSERT INTO relations (from_type, from_name, to_type, to_name, relation_type) VALUES ($1, $2, $3, $4, $5)"
        )
        .bind(from_type.to_string())
        .bind(from_name)
        .bind(to_type.to_string())
        .bind(to_name)
        .bind(relation_type)
        .execute(&self.pool)
        .await?;
        Ok(())
    }

    /// Removes all relationships originating from the specified entity.
    pub async fn clear_outgoing_relations(&self, entity_type: EntityType, name: &str) -> Result<u64, sqlx::Error> {
        let result = sqlx::query("DELETE FROM relations WHERE from_type = $1 AND from_name = $2")
            .bind(entity_type.to_string())
            .bind(name)
            .execute(&self.pool)
            .await?;
        Ok(result.rows_affected())
    }
}
