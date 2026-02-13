# Rich Content Protocol Specification

> **Source**: `src-tauri/src/core/domain/values/rich_content.rs`

The `RichContent` struct is a custom Hypermedia format designed to prevent link rot and ensure type safety in text.

## 1. Data Structure

It is **NOT** a string. It is a Vector of Enums.

```rust
pub struct RichContent {
    pub segments: Vec<ContentSegment>,
}

#[derive(Serialize, Deserialize)]
pub enum ContentSegment {
    Text(String),
    EntityRef(EntityRef),
    DateRef(DateRange),
}
```

## 2. Serialization (Wire Format)

The `serde` default for Enums is **External Tagging**.

### Payload Example
```json
{
  "segments": [
    { "Text": "The city of " },
    { 
      "EntityRef": { 
        "entity_type": "Geo",
        "entity_id": "a1b2c3d4-...",
        "display_text": "Revachol"
      } 
    },
    { "Text": " fell in " },
    {
      "DateRef": {
        "start": "1902-01-01",
        "end": "1902-12-31"
      }
    }
  ]
}
```

## 3. Parsing Logic (Backend)
*   **Input**: `RichContent::from_text(s)` creates a single `Text` segment.
*   **Parsing**: There is currently **NO** default parser for Markdown-like syntax (e.g., `[Link](...)`).
    *   *Implication*: Content created via the current API is always "Plain Text" wrapped in the struct.
    *   *Future*: A parser command will be needed to convert `[Revachol](geo:uuid)` strings into `ContentSegment::EntityRef`.

## 4. Rendering Logic (Frontend)
*   **Component**: `src/features/encyclopedia/components/RichContentDisplay.tsx`
*   **Narrowing**:
    ```typescript
    props.content.segments.map((segment) => {
      if ("Text" in segment) return <span>{segment.Text}</span>;
      if ("EntityRef" in segment) return <Link to={`/entity/${segment.EntityRef.entity_id}`}>{segment.EntityRef.display_text}</Link>;
      // ...
    })
    ```
