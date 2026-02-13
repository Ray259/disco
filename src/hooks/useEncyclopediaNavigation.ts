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
  | { type: "create"; initialType?: string };

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

  return {
    view,
    navigateToList,
    navigateToDetail,
    navigateToCreate,
  };
}
