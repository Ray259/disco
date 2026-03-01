import { useState } from "react";

export type EntityType = "figures" | "institutions" | "events" | "geos" | "works" | "schools";

export type View =
  | { type: "list"; entityType: EntityType }
  | { type: "detail"; entityType: EntityType; name: string }
  | { type: "create"; initialType?: string }
  | { type: "edit"; entityType: EntityType; name: string };

export function useEncyclopediaNavigation() {
  const [view, setView] = useState<View>({ type: "list", entityType: "figures" });
  return {
    view,
    navigateToList: (et: EntityType) => setView({ type: "list", entityType: et }),
    navigateToDetail: (et: EntityType, name: string) => setView({ type: "detail", entityType: et, name }),
    navigateToCreate: (initialType?: string) => setView({ type: "create", initialType }),
    navigateToEdit: (et: EntityType, name: string) => setView({ type: "edit", entityType: et, name }),
  };
}
