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

## 5. Current Input Gap

All form inputs are plain `<input>` / `<textarea>` - they produce strings converted via `RichContent::from_text()` (single `Text` segment). There is **no UI** for inserting `EntityRef` or `DateRef` inline.

## 6. Future: RichContentEditor

Planned approach is an **@mention**-style inline editor:
1. User types normally in a rich text field
2. Typing `@` triggers a search popup (reusing `RelationSearch`)
3. Selecting an entity inserts an inline chip representing an `EntityRef` segment
4. The editor state maps to `ContentSegment[]` - mix of `Text` and `EntityRef`

### Implementation needs
- `RichContentEditor` component (contenteditable or lightweight editor lib)
- Inline entity chips (styled, clickable, deletable)
- Serializer: editor state -> `ContentSegment[]`
- Deserializer: `ContentSegment[]` -> editor state (for edit mode)
- Replace `FormInput`/`FormTextArea` with `RichContentEditor` for relevant fields

