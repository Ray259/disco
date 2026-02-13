# Frontend Implementation Spec

## Component Hierarchy & State Flow

### `EntityCreate.tsx` (Controller)
*   **Configuration**: driven by `ENTITY_CONFIG` (maps type -> Form Component).
*   **State**: `activeTab` (derived from `ENTITY_CONFIG` keys).
*   **Render Logic**:
    *   Iterates `ENTITY_CONFIG` to generate tabs.
    *   Dynamically renders the mapped `formComponent`.
    *   **Crucial**: Does **not** retain form state when switching tabs.

### Generic Entity List (`App.tsx` / `GenericEntityList.tsx`)
*   **Role**: Replaces specific list components (like `FigureList`).
*   **Props**: Accepts `fetcher`, `renderer`, and `title` from `ENTITY_CONFIG`.
*   **State**: Manages loading/error/data generic to `T extends { id: string }`.

### Form Components (`forms/*.tsx`)
*   **Structure**: All forms now wrapper around **`SharedFormComponents`**.
*   **Shared Components**:
    *   `FormLayout`: Handles Title, Error Alert, Submit/Discard buttons.
    *   `FormInput` / `FormTextArea`: Standardized UI inputs.
    *   `TemporalCoordinates`: Standardized Date Range picker.
*   **Submission**:
    1.  Collects local state.
    2.  Calls `api` function.
    3.  Awaits Promise.
    4.  On Success: Calls `onSuccess` prop (navigates).
    5.  On Error: Sets local error state (displayed by `FormLayout`).

## API Layer (`api/index.ts`)
*   **Wrapper Pattern**: Thin wrappers around `invoke`.
*   **Type Assertions**: Response from `invoke<T>` is cast to TypeScript interfaces. Runtime validation of this buffer is **not** performed (e.g., no Zod/Io-TS).

## Rendering Logic
*   **RichContentDisplay**:
    *   Iterates `segments` array.
    *   Uses discriminated union narrowing on `segment` keys (`Text` | `EntityRef` | `DateRef`).
    *   `EntityRef` segments render as `Link` components to `/entity/:id`.
