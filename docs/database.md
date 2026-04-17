# Database Implementation Details

> **Source**: `src-tauri/src/core/db.rs`

## 1. Store Implementation
*   **Engine**: SQLite 3 (via `sqlx`)
*   **Journal Mode**: WAL (Write-Ahead Logging) enabled via connection string `sqlite:encyclopedia.db?mode=rwc`.

### Connection Pooling
*   **Pool Size**: 5 connections (Hardcoded).
*   **Reasoning**: Tauri's async runtime is multi-threaded. SQLite handles concurrent reads well in WAL mode, but writes lock the file. Limiting the pool prevents `SQLITE_BUSY` contention during rapid IPC calls.

## 2. Query Analysis

### 2.1. Inserting an Entity
**Cost**: O(1) + JSON Serialization overhead.
**SQL**:
```sql
INSERT INTO entities (entity_type, name, data, created_at, updated_at)
VALUES ($1, $2, $3, $4, $5)
```
*   `$1 (entity_type)`: `EntityType::to_string()` (e.g., "Figure")
*   `$2 (name)`: Copied from struct to allow SQL-level indexing.
*   `$3 (data)`: `serde_json::to_string(&entity)`

### 2.2. Listing Entities
**Cost**: O(N) where N is number of entities of that type.
**SQL**:
```sql
SELECT name, data FROM entities WHERE entity_type = $1 ORDER BY name
```
*   **Index Usage**: `idx_entities_type` is used to filter.
*   **Deserialization**: `parse_figure` is called N times. This is the primary bottleneck for large datasets (10k+ items).

### 2.3. Fetching Single Entity
**Cost**: O(1) (Primary Key Lookup).
**SQL**:
```sql
SELECT data FROM entities WHERE entity_type = $1 AND name = $2
```

## 3. The Relation Graph

### Schema
```sql
CREATE TABLE relations (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    from_type TEXT NOT NULL,
    from_name TEXT NOT NULL,
    to_type TEXT NOT NULL,
    to_name TEXT NOT NULL,
    relation_type TEXT NOT NULL
)
```

### Graph Traversal (Planned)
To find "All Students of Figure X":
1.  **SQL**: `SELECT from_type, from_name FROM relations WHERE to_type = 'Figure' AND to_name = $1 AND relation_type = 'STUDENT_OF'`
2.  **Index**: Uses `idx_relations_to`.
3.  **Result**: Returns List of Types and Names.
4.  **Hydration**: Must run `SELECT * FROM entities WHERE entity_type = ... AND name = ...` to get the actual student data.

## 4. Schema Evolution & Limitations
*   **Refactoring Fields**: If you rename a field in `Figure` struct (e.g. `role` -> `job`), old JSON blobs in the DB will fail to deserialize.
    *   *Solution*: `serde(rename = "old_name")` or database migration script to rewrite JSON blobs.
*   **Missing Fields (Frontend Handling)**:
    *   The frontend `EntityRenderers` include defensive checks for optional fields (like `date_range`) to preventing crashing when displaying legacy data that might be missing these fields.
*   **Orphaned JSON**: The `entities` table structure effectively performs no validation on the JSON content. Malformed JSON (via manual edit) will cause `get_figure`/`get_all_*` to return `Err`.
