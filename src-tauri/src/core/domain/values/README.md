# Domain Values

Value objects for the encyclopedia domain.

## Types

| Type | Description |
|------|-------------|
| `DateRange` | Start/end date pair for lifespans, periods, durations |
| `EntityType` | Enum: Figure, Work, Event, Geo, Institution |
| `EntityRef` | Typed reference to another entity (for hyperlinks) |
| `ContentSegment` | Text, EntityRef, or DateRef segment |
| `RichContent` | Collection of ContentSegments (replaces plain strings) |
| `Zeitgeist` | Context: catalyst, opposition, influences |

## RichContent Pattern

```rust
// "Opposed by [Napoleon] during the [French Revolution]"
RichContent::new()
    .push_text("Opposed by ")
    .push_entity_ref(EntityRef::figure(napoleon_id, "Napoleon".into()))
    .push_text(" during the ")
    .push_entity_ref(EntityRef::event(rev_id, "French Revolution".into()))
```
