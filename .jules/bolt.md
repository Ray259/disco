## 2024-04-14 - React Re-Renders Optimizations
**Learning:** In a highly interactive app with persistent layout components (like `Sidebar` and `SettingsUI` in `App.tsx`), simple state updates at the root component (like updating volume via slider) can cause massive re-renders of the entire application.
**Action:** When a top-level component manages global or shared state (like UI settings), memoize heavy side-components and ensure their prop callbacks use `useCallback`. This is especially crucial for navigation and layout components.
