# Frontend Component Reference

> **Scope**: `src/features/encyclopedia` and `src/components`

## 1. Core Components

### `EntityCreate.tsx`
Page controller for create/edit. Driven by `ENTITY_CONFIG` — maps entity types to form components.
- `activeKey`: current entity tab
- Passes `extraRelations`, `initialValues`, `editId` to form components
- Renders `RelationManager` below the form

### `GenericEntityList.tsx`
Reusable list view for any entity type. Props: `title`, `fetcher`, `renderItem`.

### `FigureDetail.tsx`
View page for a single Figure. Uses `RichContentDisplay` for text fields.

### `RichContentDisplay.tsx`
Renders `RichContent` segments — discriminates `Text` | `EntityRef` | `DateRef`. EntityRef renders as a `Link`.

---

## 2. Shared Form Components (`forms/SharedFormComponents.tsx`)

### `FormLayout`
Common form wrapper. Handles error display, submit/cancel buttons.
- **Sticky buttons**: Internalize/Discard bar is `sticky bottom-0` with backdrop blur — floats at viewport bottom when scrolled past.

### `FormInput` / `FormTextArea`
Styled inputs with label. Accept standard HTML input props.

### `TemporalCoordinates`
Start/end date picker block using `DatePicker`.

---

## 3. DatePicker (`components/DatePicker.tsx`)

### Flexible date input
Accepts three formats:
- `YYYY` — year only (precision badge: YEAR)
- `YYYY-MM` — year-month (precision badge: MONTH)
- `YYYY-MM-DD` — full date (precision badge: DAY)

### Calendar interactions
- Click a day → sets YYYY-MM-DD
- Click the month header (e.g. "MAR 2025") → sets YYYY-MM
- Type directly in the text input for any format

---

## 4. RelationSearch (`RelationManager/RelationSearch.tsx`)

Generic entity search/select component. No domain logic.

### Props
| Prop | Type | Description |
|------|------|-------------|
| `onSelect` | `(entity: SearchResult) => void` | Called when user clicks a result |
| `entityType?` | `string` | Client-side filter (e.g. `"Geo"`) |
| `allowCreate?` | `boolean` | Show "+ Create" option when no exact match |
| `onCreate?` | `(name: string) => void` | Caller-owned creation callback |
| `onCreateLabel?` | `string` | Custom label for create button |
| `label?` | `string` | Input label |
| `placeholder?` | `string` | Input placeholder |

Search triggers at 2+ characters with 300ms debounce via `searchEntities` API.

---

## 5. FigureForm (`forms/FigureForm.tsx`)

### Form state
Uses `useFormState` hook (reducer-based). Fields: `name`, `role`, `location`, `quote`, `startYear`, `endYear`, plus 7 special field states.

### GeoPickerField (local component)
Handles the origin/location field. Two states:
- **Empty**: Shows `RelationSearch` filtered to Geo entities with `allowCreate`. When "+ Create" is clicked, calls `createGeo` API — onPick only fires on success.
- **Filled**: Shows selected name with "change" button to clear.

Stores `locationId` for future EntityRef wiring.

### Special Fields (UI-only, not backend-wired)
Defined in `forms/SpecialFields.tsx`:

| Component | State Shape | UI Pattern |
|-----------|------------|------------|
| `ZeitgeistField` | `{ era, catalyst, opposition }` | Side-by-side cards |
| `CoreIdeologyField` | `{ axiom, argumentFlow }` | Split panel |
| `LineageField` | `{ predecessors[], rivals[], successors[] }` | 3-column chip inputs |
| `LegacyField` | `{ shortTermSuccess, modernRelevance, criticalFlaw, personalSynthesis }` | 4-block grid |
| `TerminologyField` | `{ entries: [{term, definition}] }` | Dynamic key-value list |
| `ContributionsField` | `{ entries: [{year, contribution}] }` | Timeline with markers |
| `InstitutionalField` | `{ fundingModel, institutionalProduct, successionPlan }` | 3-input panel |

### Payload (current)
Submit sends: `{ name, role, location, start_year, end_year, quote?, relations }`.
Special fields are **not** included in the payload yet.

---

## 6. Other Forms

All forms follow the same pattern: local state → API call on submit → `FormLayout` wrapper.

- **EventForm**: `name`, `start_date`, `end_date`, `description`
- **InstitutionForm**: `name`, `founded_start`, `founded_end`, `description`
- **GeoForm**: `name`, `region`, `description`
- **WorkForm**: `title`, `summary`
- **SchoolForm**: `name`, `description`

---

## 7. API Layer (`api/index.ts`)

Thin wrappers around Tauri `invoke`. Each function types its argument and return to match backend structs. Key types:
- `SearchResult`: `{ id, name, entity_type, description? }`
- `RichContent`: `{ segments: ContentSegment[] }`
- `RelationDto`: `{ target_id: UUID, relation_type: string }`

---

## 8. Hooks

### `useFormState<T>(initial)` (`hooks/useFormState.ts`)
Reducer-based form state management. Returns `[state, setField, resetForm]`.
- `setField(field, value)` — updates a single field
- `resetForm(newState)` — replaces entire state
