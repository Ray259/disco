# Vault Synchronization Reference

This document describes how data is synced between the SQLite database `encyclopedia.db` and the Markdown files in the selected `/vault` directory.

## 1. Storage Components

### SQLite Database (`entities` table)
- `id` (TEXT PRIMARY KEY): UUID v4.
- `entity_type` (TEXT): Entity category.
- `name` (TEXT): Entity name.
- `data` (TEXT): JSON representation of the entity struct.
- `file_path` (TEXT): Absolute path to the corresponding `.md` file.

### Markdown Files (`[Vault]/[Type]/[Name].md`)
1. **YAML Frontmatter**
   - Contains: `id`, `entity_type`, `name`, `created_at`, `updated_at`.
   - Contains: All custom fields for the struct (e.g. `life`, `date_range`).
   - Contains: A `relations` array. Example:
     ```yaml
     relations:
       - target:
           entity_id: 123e4567-e89b-12d3-a456-426614174000
         kind: !Fixed Mentorship
     ```
2. **Markdown Body**
   - Placed below the `---` delimiters.
   - Used for the primary long-form text content (e.g. `description`).

## 2. Converting Markdown to SQLite (`markdown_to_entity_data`)

When a `.md` file is processed:
1. The file is split into the YAML frontmatter string and the body string.
2. The YAML string is parsed into a JSON object (`serde_yaml::from_str`).
3. The body string is inserted into the JSON object at the correct field.
4. The JSON object is serialized to a string (`data`).
5. Returns `(id, entity_type, name, data, file_path)`.

## 3. Saving to SQLite (`sync_single_file`)

`VaultManager::sync_single_file` takes the parsed data and executes:
1. `db.upsert_entity(id, type, name, data, file_path)`: Uses `INSERT OR REPLACE` to save the JSON blob `data`.
2. `db.clear_outgoing_relations(id)`: Deletes previous relations where `from_id = id`.
3. `db.insert_relation(id, target_id, kind)`: Re-inserts rows into the `relations` table by iterating over the `relations` array inside the JSON blob.

## 4. File Watcher (`watcher.rs`)

The file watcher monitors the active vault directory for changes.
1. `notify::RecommendedWatcher` listens for `Create`, `Modify`, `Remove` events on `.md` files.
2. Events are pushed to a `tokio::mpsc::channel`.
3. A background task reads paths from the channel. It waits 300ms after the first path is received to catch subsequent events from multi-step saves.
4. It deduplicates the paths.
5. If the path exists: `sync_single_file()` is called.
6. If the path does not exist: `handle_file_deleted()` is called, which runs `DELETE FROM entities WHERE file_path = path`.

## 5. Startup and Configuration State

1. **No Vault (Initial State)**: 
   - When `vault_config.json` returns `None` (or doesn't exist).
   - No vault directory reading occurs.
   - The file watcher is not started.
   - The SQLite database remains empty.
   - `write_entity` commands will fail and return an error.

2. **Connecting a Vault**:
   - When the Frontend calls `set_vault_path` via Settings.
   - The running file watcher process is stopped.
   - `db.empty_database()` executes `DELETE FROM relations` and `DELETE FROM entities`.
   - `VaultManager::full_sync` iterates through all `.md` files in the new path, calling `sync_single_file` to populate the empty DB.
   - A new file watcher process is started for the new path.
