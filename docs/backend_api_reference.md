# Backend API Reference

> **Scope**: `src-tauri/src/commands` & `src-tauri/src/core`

## Shared Utilities (`commands/common.rs`)

### `parse_flexible_date(s, field) -> Result<NaiveDate, String>`
Accepts three date formats:
1. `YYYY-MM-DD` (full) → parsed directly
2. `YYYY-MM` (month) → day defaults to 01
3. `YYYY` (year) → month+day default to 01-01

Used by all entity commands that accept date strings.

### `handle_create<E, D>(state, vault, request) -> Result<String, String>`
Generic create pipeline: DTO → Entity → serialize → SQLite insert → relation insert → vault markdown write.

### `handle_update<E, D>(state, vault, entity_type, name, request) -> Result<String, String>`
Generic update pipeline: fetch → deserialize → apply DTO → serialize → SQLite update → rebuild relations → vault re-write.

### `delete_entity(state, vault, entity_type, name) -> Result<String, String>`
Deletes entity from SQLite and removes vault markdown file.

---

## 1. Figure Commands (`commands/figure.rs`)

### `create_figure(state, vault, request)`
```rust
struct CreateFigureRequest {
    name: String,
    role: String,        // → RichContent
    location: String,    // → RichContent
    start_year: String,  // flexible: YYYY, YYYY-MM, or YYYY-MM-DD
    end_year: String,    // flexible
    quote: Option<String>,
    relations: Option<Vec<RelationDto>>,
}
```
Delegates to `handle_create`.

### `update_figure(state, vault, name, request)`
Same DTO. Delegates to `handle_update`.

### `get_all_figures(state) -> Vec<Figure>`
### `get_figure(state, name) -> Option<Figure>`

---

## 2. Institution Commands (`commands/institution.rs`)

### `create_institution` / `update_institution`
```rust
struct CreateInstitutionRequest {
    name: String,
    founded_start: Option<String>,  // flexible date
    founded_end: Option<String>,    // flexible date
    description: Option<String>,
    relations: Option<Vec<RelationDto>>,
}
```

### `get_all_institutions` / `get_institution`

---

## 3. Event Commands (`commands/event.rs`)

### `create_event` / `update_event`
```rust
struct CreateEventRequest {
    name: String,
    start_date: String,     // flexible date
    end_date: String,       // flexible date
    description: Option<String>,
    relations: Option<Vec<RelationDto>>,
}
```

### `get_all_events` / `get_event`

---

## 4. Geo Commands (`commands/geo.rs`)

### `create_geo` / `update_geo`
Input: `{ name, region?, description?, relations? }`

### `get_all_geos` / `get_geo`

---

## 5. Work Commands (`commands/work.rs`)

### `create_work` / `update_work`
Input: `{ title, summary?, relations? }`

### `get_all_works` / `get_work`

---

## 6. School of Thought Commands (`commands/school.rs`)

### `create_school_of_thought` / `update_school_of_thought`
Input: `{ name, description?, relations? }`

### `get_all_schools_of_thought` / `get_school_of_thought`

---

## 7. Search (`commands/search.rs` or inline)

### `search_entities(query) -> Vec<SearchResult>`
```rust
struct SearchResult {
    entity_type: String,
    name: String,
}
```
SQL LIKE search across all entity types.

---

## Value Objects (`core/domain/values/`)

### `RichContent` (`rich_content.rs`)
`Vec<ContentSegment>` where segment is `Text(String)` | `EntityRef(EntityRef)` | `DateRef(DateRange)`.
- `from_text(str)` — creates single Text segment (current default for all form inputs).

### `DateRange` (`date_range.rs`)
`{ start: NaiveDate, end: NaiveDate }`.

### `Zeitgeist` (`zeitgeist.rs`)
`{ era, catalyst, opposition, influences: Vec<EntityRef> }`.
