import { useState } from "react";

export type EntityType = 
  | "figures"
  | "institutions"
  | "events"
  | "geos"
  | "works"
  | "schools";

export type View = 
  | { type: "list"; entityType: EntityType }
  | { type: "detail"; entityType: EntityType; id: string }
  | { type: "create"; initialType?: string }
  | { type: "edit"; entityType: EntityType; id: string };

export function useEncyclopediaNavigation() {
  const [view, setView] = useState<View>({ type: "list", entityType: "figures" });

  const navigateToList = (entityType: EntityType) => {
    setView({ type: "list", entityType });
  };

  const navigateToDetail = (entityType: EntityType, id: string) => {
    setView({ type: "detail", entityType, id });
  };

  const navigateToCreate = (initialType?: string) => {
    setView({ type: "create", initialType });
  };

  const navigateToEdit = (entityType: EntityType, id: string) => {
    setView({ type: "edit", entityType, id });
  };

  return {
    view,
    navigateToList,
    navigateToDetail,
    navigateToCreate,
    navigateToEdit
  };
}
