# Frontend Component Reference

> **Scope**: `src/features/encyclopedia`

This document details every React component, its props, state, and rendering logic.

## 1. Core Components

### `EntityCreate.tsx`
**Role**: The Page Controller for the creation route.
*   **Config**: Imports `ENTITY_CONFIG` to know which entities exist.
*   **State**: `activeTab` (defaults to first key in config).
*   **Logic**:
    *   Renders a dynamically generated Tab Bar.
    *   Renders the specific Form Component from config (e.g. `<FigureForm />`).
    *   Passes `extraRelations` (mocked for now) to forms.

### `GenericEntityList.tsx`
**Role**: A reusable list view for ANY entity type.
*   **Props**:
    *   `title`: String.
    *   `fetcher`: Async function returning `T[]`.
    *   `renderItem`: Function returning ReactNode for a `T`.
*   **Logic**:
    *   Fetches data on mount using `fetcher`.
    *   Renders each item using `renderItem`.
    *   Handles Loading/Error states uniformly.

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

All forms now use **`SharedFormComponents`** for layout and consistent styling.

*   **`SharedFormComponents.tsx`**:
    *   `FormLayout`: The common wrapper.
    *   `FormInput`, `FormTextArea`: Styled inputs.
    *   `TemporalCoordinates`: The "start-end" date block.

### Specific Forms
Each form (`FigureForm`, `InstitutionForm`, etc.) is now a thin wrapper that:
1.  Defines local state for specific fields.
2.  Calls the specific API creator (e.g. `createFigure`).
3.   renders `<FormLayout>` containing `<FormInput>`s.

*   **`FigureForm`**: Adds `TemporalCoordinates` for life span.
*   **`InstitutionForm`**: Adds `TemporalCoordinates` for founding dates.
*   **`EventForm`**: Adds `TemporalCoordinates` for event duration.
*   **`GeoForm`**: Standard fields.
*   **`WorkForm`**: Uses a custom date input for Publication Date.
*   **`SchoolForm`**: Standard fields.

---

## 3. API Layer (`api/index.ts`)

Pure TypeScript functions. No state.
*   Each function explicitly types its argument to match the Backend `Deserialize` struct.
*   Each function explicitly types its return Promise to match Backend `Serialize` struct.

**Data Types**:
*   `RichContent`: `{ segments: [...] }`
*   `EntityRef`: `{ entity_type, entity_id, display_text }`
*   `DateRange`: `{ start, end }`
