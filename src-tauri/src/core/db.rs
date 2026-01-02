use rusqlite::{Connection, Result, params};
use std::path::Path;
use uuid::Uuid;

use crate::core::domain::values::entity_ref::EntityType;

pub struct EncyclopediaDb {
    conn: Connection,
}

impl EncyclopediaDb {
    pub fn open<P: AsRef<Path>>(path: P) -> Result<Self> {
        let conn = Connection::open(path)?;
        let db = Self { conn };
        db.init_schema()?;
        Ok(db)
    }

    pub fn open_in_memory() -> Result<Self> {
        let conn = Connection::open_in_memory()?;
        let db = Self { conn };
        db.init_schema()?;
        Ok(db)
    }

    fn init_schema(&self) -> Result<()> {
        self.conn.execute_batch(
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
    }

    // Entity CRUD
    pub fn insert_entity(&self, id: Uuid, entity_type: EntityType, name: &str, data: &str) -> Result<()> {
        let now = chrono::Utc::now().to_rfc3339();
        self.conn.execute(
            "INSERT INTO entities (id, entity_type, name, data, created_at, updated_at) VALUES (?1, ?2, ?3, ?4, ?5, ?6)",
            params![id.to_string(), entity_type.to_string(), name, data, now, now],
        )?;
        Ok(())
    }

    pub fn update_entity(&self, id: Uuid, name: &str, data: &str) -> Result<usize> {
        let now = chrono::Utc::now().to_rfc3339();
        self.conn.execute(
            "UPDATE entities SET name = ?1, data = ?2, updated_at = ?3 WHERE id = ?4",
            params![name, data, now, id.to_string()],
        )
    }

    pub fn delete_entity(&self, id: Uuid) -> Result<usize> {
        // Delete relations first
        self.conn.execute("DELETE FROM relations WHERE from_id = ?1 OR to_id = ?1", params![id.to_string()])?;
        self.conn.execute("DELETE FROM entities WHERE id = ?1", params![id.to_string()])
    }

    pub fn get_entity(&self, id: Uuid) -> Result<Option<(String, String, String)>> {
        let mut stmt = self.conn.prepare("SELECT entity_type, name, data FROM entities WHERE id = ?1")?;
        let mut rows = stmt.query(params![id.to_string()])?;
        
        if let Some(row) = rows.next()? {
            Ok(Some((row.get(0)?, row.get(1)?, row.get(2)?)))
        } else {
            Ok(None)
        }
    }

    pub fn list_entities(&self, entity_type: Option<EntityType>) -> Result<Vec<(Uuid, String, String)>> {
        let sql = match entity_type {
            Some(_) => "SELECT id, name, data FROM entities WHERE entity_type = ?1",
            None => "SELECT id, name, data FROM entities",
        };
        
        let mut stmt = self.conn.prepare(sql)?;
        let rows = match entity_type {
            Some(t) => stmt.query_map(params![t.to_string()], |row| {
                let id_str: String = row.get(0)?;
                Ok((Uuid::parse_str(&id_str).unwrap(), row.get(1)?, row.get(2)?))
            })?,
            None => stmt.query_map([], |row| {
                let id_str: String = row.get(0)?;
                Ok((Uuid::parse_str(&id_str).unwrap(), row.get(1)?, row.get(2)?))
            })?,
        };
        
        rows.collect()
    }

    // Relation CRUD
    pub fn insert_relation(&self, from_id: Uuid, to_id: Uuid, relation_type: &str) -> Result<()> {
        self.conn.execute(
            "INSERT INTO relations (from_id, to_id, relation_type) VALUES (?1, ?2, ?3)",
            params![from_id.to_string(), to_id.to_string(), relation_type],
        )?;
        Ok(())
    }

    pub fn delete_relation(&self, from_id: Uuid, to_id: Uuid, relation_type: &str) -> Result<usize> {
        self.conn.execute(
            "DELETE FROM relations WHERE from_id = ?1 AND to_id = ?2 AND relation_type = ?3",
            params![from_id.to_string(), to_id.to_string(), relation_type],
        )
    }

    pub fn get_relations_from(&self, from_id: Uuid) -> Result<Vec<(Uuid, String)>> {
        let mut stmt = self.conn.prepare("SELECT to_id, relation_type FROM relations WHERE from_id = ?1")?;
        let rows = stmt.query_map(params![from_id.to_string()], |row| {
            let id_str: String = row.get(0)?;
            Ok((Uuid::parse_str(&id_str).unwrap(), row.get(1)?))
        })?;
        rows.collect()
    }

    pub fn get_relations_to(&self, to_id: Uuid) -> Result<Vec<(Uuid, String)>> {
        let mut stmt = self.conn.prepare("SELECT from_id, relation_type FROM relations WHERE to_id = ?1")?;
        let rows = stmt.query_map(params![to_id.to_string()], |row| {
            let id_str: String = row.get(0)?;
            Ok((Uuid::parse_str(&id_str).unwrap(), row.get(1)?))
        })?;
        rows.collect()
    }
}
