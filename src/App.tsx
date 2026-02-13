import { Sidebar } from "./components/Layout/Sidebar";
import { FigureDetail } from "./features/encyclopedia/components/FigureDetail";
import { GenericEntityList } from "./features/encyclopedia/components/GenericEntityList";
import { EntityCreate } from "./features/encyclopedia/components/EntityCreate";
import { ENTITY_CONFIG } from "./features/encyclopedia/components/EntityConfig";
import { useEncyclopediaNavigation, EntityType as NavEntityType } from "./hooks/useEncyclopediaNavigation";
import "./App.css";

function App() {
  const { view, navigateToList, navigateToDetail, navigateToCreate } = useEncyclopediaNavigation();

  const renderContent = () => {
    switch (view.type) {
      case "create":
        return (
          <EntityCreate 
            initialType={view.initialType as any}
            onSuccess={() => navigateToList("figures")} 
            onCancel={() => navigateToList("figures")} 
          />
        );

      case "detail":
        // For now, only Figures have a dedicated detail view.
        // In the future, we can add this to ENTITY_CONFIG too.
        if (view.entityType === "figures") {
          return (
            <FigureDetail 
              id={view.id} 
              onBack={() => navigateToList("figures")} 
            />
          );
        }
        return <div className="p-8 text-gray-500">Detail view for {view.entityType} coming soon.</div>;

      case "list":
        const config = ENTITY_CONFIG[view.entityType];
        if (!config) return <div className="p-8">Unknown category.</div>;

        return (
          <GenericEntityList
            title={config.title}
            fetcher={config.fetcher}
            renderItem={config.renderer}
            onSelect={(item) => navigateToDetail(view.entityType, item.id)}
            onCreate={() => navigateToCreate(config.createType)}
          />
        );
    }
  };

  return (
    <div className="h-screen bg-black text-white w-full flex overflow-hidden">
      <Sidebar 
        currentView={view.type === "list" ? view.entityType : ""} 
        onChangeView={(type) => navigateToList(type as NavEntityType)} 
      />
      <main className="flex-1 h-full relative overflow-hidden">
        {renderContent()}
      </main>
    </div>
  );
}

export default App;
