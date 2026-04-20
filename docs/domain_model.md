# Domain Entity Specifications

> **Source**: `src-tauri/src/core/domain/models/*`

This document serves as the canonical reference for the Domain Models serialized into the database.

## 1. Figure (`figure.rs`)
**Discriminator**: `EntityType::Figure`

The most complex entity, representing a person with historical significance.

### Struct Definition
```rust
pub struct Figure {
    pub name: String,
    
    // Lifecycle
    pub life: DateRange, 
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,

    // Primary Metadata
    pub primary_role: RichContent,      // e.g. "Moralist Philosopher"
    pub primary_location: RichContent,  // e.g. "Revachol"
    pub defining_quote: Option<RichContent>,

    // Intellectual Framework (The "Mind")
    pub zeitgeist: Option<Zeitgeist>,          // Historic spirit alignment
    pub axiom: Option<RichContent>,            // Core belief
    pub key_terminology: HashMap<String, RichContent>, // Dictionary of terms
    pub argument_flow: Option<RichContent>,    // How they argue

    // Institutional Power (The "Body")
    pub primary_institution: Option<EntityRef>,
    pub funding_model: Option<RichContent>,
    pub institutional_product: Option<RichContent>,
    pub succession_plan: Option<RichContent>,

    // Timeline & Legacy
    pub major_contributions: Vec<MajorContribution>,
    pub short_term_success: Option<RichContent>,
    pub modern_relevance: Option<RichContent>,
    pub critical_flaw: Option<RichContent>,
    pub personal_synthesis: Option<RichContent>,

    // Graph Edges
    pub predecessors: Vec<EntityRef>,
    pub contemporary_rivals: Vec<EntityRef>,
    pub successors: Vec<EntityRef>,
    pub relations: Vec<Relation>, // Generic edges
}
```

### Sub-Types
*   **`MajorContribution`**: `{ title: String, date: DateRange, impact: RichContent }`
*   **`Zeitgeist`**: Enum/Struct defining the "spirit of the age" they belong to.

---

## 2. Institution (`institution.rs`)
**Discriminator**: `EntityType::Institution`

Organizations, governments, companies.

### Struct Definition
```rust
pub struct Institution {
    pub name: String,
    
    pub founded: Option<DateRange>,
    pub location_ref: Option<EntityRef>, // Head Office
    pub description: Option<RichContent>,
    
    pub founders: Vec<EntityRef>,    // Links to Figures
    pub products: Vec<RichContent>,  // What they produce (Laws, Cars, Ideas)
    
    pub relations: Vec<Relation>,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
}
```

---

## 3. Event (`event.rs`)
**Discriminator**: `EntityType::Event`

Moments in history.

### Struct Definition
```rust
pub struct Event {
    pub name: String,
    
    pub date_range: DateRange,
    pub description: Option<RichContent>,
    pub location_ref: Option<EntityRef>,
    
    pub participants: Vec<EntityRef>, // Who was there
    pub causes: Vec<RichContent>,     // Why it happened
    pub consequences: Vec<RichContent>, // What happened after
    
    pub relations: Vec<Relation>,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
}
```

---

## 4. Work (`work.rs`)
**Discriminator**: `EntityType::Work`

Books, Art, Theories.

### Struct Definition
```rust
pub struct Work {
    pub title: String, // Note: mapped to `name` in DB index
    
    pub authors: Vec<EntityRef>,
    pub publication_date: Option<DateRange>,
    pub summary: Option<RichContent>,
    pub key_ideas: Vec<RichContent>,
    
    pub relations: Vec<Relation>,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
}
```

---

## 5. Geo (`geo.rs`)
**Discriminator**: `EntityType::Geo`

Locations, Cities, Regions.

### Struct Definition
```rust
pub struct Geo {
    pub name: String,
    
    pub region: Option<RichContent>, // Broad container (e.g. "Insulinde")
    pub description: Option<RichContent>,
    pub aliases: Vec<String>,        // e.g. "Martinase"
    
    pub relations: Vec<Relation>,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
}
```

---

## 6. School of Thought (`school.rs`)
**Discriminator**: `EntityType::SchoolOfThought`

Ideologies (Communism, Moralism, Ultraliberalism).

### Struct Definition
```rust
pub struct SchoolOfThought {
    pub name: String,
    
    pub description: Option<RichContent>,
    pub relations: Vec<Relation>,
    pub sub_schools: Vec<String>, // e.g. "Mazovian Socio-Economics"
    
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
}
```
