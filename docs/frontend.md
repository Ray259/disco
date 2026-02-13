# Frontend Implementation Spec

## Component Hierarchy & State Flow

### `EntityCreate.tsx` (Controller)
*   **State**: `activeTab` (String enum).
*   **Render Logic**: Conditional rendering based on `activeTab`.
    *   Does **not** retain form state when switching tabs. Switching tabs unmounts the previous form, effectively resetting state.

### Form Components (`forms/*.tsx`)
*   **State Isolation**: Each form maintains its own local `useState` for fields.
*   **Submission**:
    1.  Collects local state.
    2.  Validates basic constraints (HTML5 `required`).
    3.  Calls `api` function.
    4.  Awaits Promise.
    5.  On Success: Navigates via `useNavigate`.
    6.  On Error: Logs to console (Alerting not implemented).

## API Layer (`api/index.ts`)
*   **Wrapper Pattern**: Thin wrappers around `invoke`.
*   **Type Assertions**: Response from `invoke<T>` is cast to TypeScript interfaces. Runtime validation of this buffer is **not** performed (e.g., no Zod/Io-TS).

## Rendering Logic
*   **RichContentDisplay**:
    *   Iterates `segments` array.
    *   Uses discriminated union narrowing on `segment` keys (`Text` | `EntityRef` | `DateRef`).
    *   `EntityRef` segments render as `Link` components to `/entity/:id`.
