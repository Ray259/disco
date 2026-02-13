# Backend API Reference

> **Scope**: `src-tauri/src/commands` & `src-tauri/src/core`

This document details every public command exposed to the frontend and the internal logic governing it.

## 1. Figure Commands (`commands/figure.rs`)

### `create_figure`
*   **Signature**: `fn(state, request: CreateFigureRequest) -> Result<String, String>`
*   **Input**:
    ```rust
    struct CreateFigureRequest {
        name: String,
        role: String,      // Converted to RichContent
        location: String,  // Converted to RichContent
        start_year: String,// Parsed as YYYY (defaulting to Jan 1)
        end_year: String,  // Parsed as YYYY
        quote: Option<String>
    }
    ```
*   **Logic**:
    1.  Generates new `Uuid::new_v4()`.
    2.  Parses years into `NaiveDate` (Jan 1st).
    3.  Constructs `Figure` struct.
    4.  Serializes to JSON.
    5.  Inserts into `entities` table with type `Figure`.
*   **Error Handling**: Returns `Err` if date parsing fails or DB insert fails.

### `get_all_figures`
*   **Signature**: `fn(state) -> Result<Vec<Figure>, String>`
*   **Logic**:
    1.  Calls `search_entities(EntityType::Figure)`.
    2.  Deserializes every JSON blob returned.
    3.  **Warning**: If any blob fails deserialization (e.g. schema change), this implementation currently propagates the error, potentially failing the whole list.

### `get_figure`
*   **Input**: `id: Uuid`
*   **Logic**: Fetches single row. Returns `Ok(None)` if not found.

---

## 2. Institution Commands (`commands/institution.rs`)

### `create_institution`
*   **Input**:
    ```rust
    struct CreateInstitutionRequest {
        name: String,
        founded_start: Option<String>,
        founded_end: Option<String>,
        description: Option<String>
    }
    ```
*   **Logic**:
    *   Dates are optional here. simpler parsing logic than Figure.
    *   Initializes empty vectors for `founders`, `products`, `relations`.

### `get_all_institutions`
*   **Signature**: `fn(state) -> Result<Vec<Institution>, String>`
*   **Logic**: Fetches all entities where `entity_type = "Institution"`.

---

## 3. Event Commands (`commands/event.rs`)

### `create_event`
*   **Input**:
    ```rust
    struct CreateEventRequest {
        name: String,
        start_date: String, // Required YYYY-MM-DD
        end_date: String,   // Required YYYY-MM-DD
        description: Option<String>
    }
    ```
*   **Logic**:
    *   Strict date parsing (`%Y-%m-%d`).
    *   Converts dates to `DateRange`.

### `get_all_events`
*   **Signature**: `fn(state) -> Result<Vec<Event>, String>`
*   **Logic**: Fetches all entities where `entity_type = "Event"`.

---

## 4. Geo Commands (`commands/geo.rs`)

### `create_geo`
*   **Input**: `{ name, region?, description? }`
*   **Logic**: Plain text fields converted to `RichContent`.

### `get_all_geos`
*   **Signature**: `fn(state) -> Result<Vec<Geo>, String>`
*   **Logic**: Fetches all entities where `entity_type = "Geo"`.

---

## 5. Work Commands (`commands/work.rs`)

### `create_work`
*   **Input**: `{ title, summary? }`
*   **Logic**: `title` maps to `entities.name`.

### `get_all_works`
*   **Signature**: `fn(state) -> Result<Vec<Work>, String>`
*   **Logic**: Fetches all entities where `entity_type = "Work"`.

---

## 6. School of Thought Commands (`commands/school.rs`)

### `create_school_of_thought`
*   **Input**: `{ name, description? }`
*   **Logic**:
    *   Creates a new SchoolOfThought.
    *   Initializes `sub_schools` as empty vector (future feature).

### `get_all_schools_of_thought`
*   **Signature**: `fn(state) -> Result<Vec<SchoolOfThought>, String>`
*   **Logic**: Fetches all entities where `entity_type = "SchoolOfThought"`.

---

## 7. Value Objects & Logic (`core/core/domain/values`)

### `RichContent`
*   **File**: `rich_content.rs`
*   **Purpose**: Stores text that can contain links.
*   **Internal**: `Vec<ContentSegment>`.
*   **Methods**:
    *   `from_text(str)`: Creates a single-segment text block.
    *   `push_entity_ref(ref)`: Appends a link.

### `DateRange`
*   **File**: `date_range.rs`
*   **Purpose**: Wrapper for `start` and `end` dates.
*   **Methods**:
    *   `contains(date)`: Returns true if date is within range.
    *   `duration_days()`: Returns `end - start` in days.

### `Zeitgeist`
*   **File**: `zeitgeist.rs`
*   **Purpose**: Represents the "Spirit of the Age" for a Figure.
*   **Fields**:
    *   `era`: The governing time period (e.g. "The Thirties").
    *   `catalyst`: What sparked this era.
    *   `opposition`: What this era fought against.
    *   `influences`: List of `EntityRef` pointers to people who defined this time.
