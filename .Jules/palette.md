## 2024-04-14 - Icon-Only Button Accessibility Pattern
**Learning:** Found multiple icon-only control buttons (Sidebar settings, SettingsUI close) missing ARIA labels and visible keyboard focus states, making them inaccessible to screen readers and keyboard users.
**Action:** Always add `aria-label` and `focus-visible:ring-2 focus-visible:outline-none rounded` classes to icon-only buttons (`lucide-react` icons).
