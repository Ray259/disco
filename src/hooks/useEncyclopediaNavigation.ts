import { useState, useCallback } from "react";

export type EntityType = "figures" | "institutions" | "events" | "geos" | "works" | "schools";

export type View =
  | { type: "list"; entityType: EntityType }
  | { type: "detail"; entityType: EntityType; name: string }
  | { type: "create"; initialType?: string }
  | { type: "edit"; entityType: EntityType; name: string }
  | { type: "crew" };

export function useEncyclopediaNavigation() {
  const [view, setView] = useState<View>({ type: "list", entityType: "figures" });

  const navigateToList = useCallback((et: EntityType) => setView({ type: "list", entityType: et }), []);
  const navigateToDetail = useCallback((et: EntityType, name: string) => setView({ type: "detail", entityType: et, name }), []);
  const navigateToCreate = useCallback((initialType?: string) => setView({ type: "create", initialType }), []);
  const navigateToEdit = useCallback((et: EntityType, name: string) => setView({ type: "edit", entityType: et, name }), []);
  const navigateToCrew = useCallback(() => setView({ type: "crew" }), []);

  return {
    view,
    navigateToList,
    navigateToDetail,
    navigateToCreate,
    navigateToEdit,
    navigateToCrew,
  };
}
