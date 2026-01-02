# Domain Models

Encyclopedia entity models.

## Entities

| Entity | Description |
|--------|-------------|
| `Figure` | Historical/philosophical figure with profile, zeitgeist, ideology, lineage |
| `Work` | Book, treatise, artwork with authors (as EntityRef) |
| `Event` | Historical event with participants, causes, consequences |
| `Geo` | Geographic location with aliases |
| `Institution` | Organization, school, party with founders |

## Field Conventions

- **`name`/`title`**: Always `String` (identity, not content)
- **All other text fields**: `RichContent` (supports entity hyperlinks)
- **References to entities**: `EntityRef` or `Vec<EntityRef>`
- **Timestamps**: `created_at`, `updated_at` as `DateTime<Utc>`

## MajorContribution

Timeline item for figures linking to works/events:

```rust
MajorContribution {
    title: String,            // Display title
    entity_ref: Option<EntityRef>,  // Link to Work/Event
    date: DateRange,
    impact: RichContent,      // "So what?"
}
```
