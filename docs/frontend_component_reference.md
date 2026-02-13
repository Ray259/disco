# Frontend Component Reference

> **Scope**: `src/features/encyclopedia`

This document details every React component, its props, state, and rendering logic.

## 1. Core Components

### `EntityCreate.tsx`
**Role**: The Page Controller for the "/create" route.
*   **State**:
    *   `activeTab`: String enum ("Figure" | "Institution" | ...). Defaults to "Figure".
*   **Logic**:
    *   Renders a Tab Bar (Tailwind grid).
    *   Based on `activeTab`, renders exactly one sub-form (e.g. `<FigureForm />`).
    *   **Crucial**: Does not pass state down. Each form manages its own state. Unmounting a tab clears that form's input.

### `FigureList.tsx`
**Role**: The Landing Page / Index.
*   **State**:
    *   `figures`: `Figure[]`.
    *   `loading`: Boolean.
    *   `error`: String | null.
*   **Effect**:
    *   On Mount: Calls `api.getAllFigures()`.
*   **Render**:
    *   Maps over `figures`.
    *   Renders a Card with `name`, `primary_role`, and `life`.
    *   Clicking a card navigates to `/figure/:id`.

### `FigureDetail.tsx`
**Role**: The View Page for a single figure.
*   **Params**: `id` (from URL).
*   **State**: `figure` (Figure | null).
*   **Render**:
    *   Displays full details including `RichContentDisplay` components for bio/quotes.
    *   **Missing**: Currently specific to Figures. Needs to be generalized or replicated for other entities.

### `RichContentDisplay.tsx`
**Role**: The "Markdown Renderer" of our system.
*   **Props**: `{ content: RichContent }`.
*   **Logic**:
    *   Checks `content.segments`.
    *   Iterates and discriminates based on key (`Text` vs `EntityRef` vs `DateRef`).
    *   `EntityRef` segments are wrapped in React Router `Link`.

---

## 2. Forms (`components/forms/*`)

All forms follow the `onSubmit` pattern:
1.  `e.preventDefault()`
2.  Construct Request Object.
3.  `await api.createX(req)`.
4.  `navigate('/')` (or to detail view).

### `FigureForm.tsx`
*   **Fields**: Name, Role, Location, Start Year, End Year, Quote (TextArea).
*   **Validation**: HTML5 `required` on Name, Role, Location, Years.

### `InstitutionForm.tsx`
*   **Fields**: Name, Founded Start (Opt), Founded End (Opt), Description (TextArea).

### `EventForm.tsx`
*   **Fields**: Name, Start Date (Date Picker), End Date (Date Picker), Description.
*   **Note**: Uses `type="date"` input, unlike Figure which uses `type="number"` for years.

### `GeoForm.tsx`
*   **Fields**: Name, Region, Description.

### `WorkForm.tsx`
*   **Fields**: Title, Summary.

### `SchoolForm.tsx`
*   **Fields**: Name, Description.

---

## 3. API Layer (`api/index.ts`)

Pure TypeScript functions. No state.
*   Each function explicitly types its argument to match the Backend `Deserialize` struct.
*   Each function explicitly types its return Promise to match Backend `Serialize` struct.

**Data Types**:
*   `RichContent`: `{ segments: [...] }`
*   `EntityRef`: `{ entity_type, entity_id, display_text }`
*   `DateRange`: `{ start, end }`
