import { useState, useRef, useEffect } from "react";
import { Settings as SettingsIcon } from "lucide-react";
import { SettingsUI } from "./components/SettingsUI";
import { Sidebar } from "./components/Layout/Sidebar";
import { FigureDetail } from "./features/encyclopedia/components/FigureDetail";
import { GenericEntityList } from "./features/encyclopedia/components/GenericEntityList";
import { EntityCreate } from "./features/encyclopedia/components/EntityCreate";
import { ENTITY_CONFIG } from "./features/encyclopedia/components/EntityConfig";
import { useEncyclopediaNavigation, EntityType as NavEntityType } from "./hooks/useEncyclopediaNavigation";
import { deleteEntity } from "./features/encyclopedia/api";
import "./App.css";

function App() {
  const { view, navigateToList, navigateToDetail, navigateToCreate, navigateToEdit } = useEncyclopediaNavigation();
  
  // Audio Settings State
  const [showSettings, setShowSettings] = useState(false);
  const [volume, setVolume] = useState(0.5);
  const [isMuted, setIsMuted] = useState(false);
  const audioRef = useRef<HTMLAudioElement>(null);

  // Sync state with audio element
  useEffect(() => {
    if (audioRef.current) {
      audioRef.current.volume = volume;
      audioRef.current.muted = isMuted;
    }
  }, [volume, isMuted]);

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

      case "edit":
        // We range over keys to find which config has this entityType?
        // view.entityType is like "figures", "institutions".
        // ENTITY_CONFIG keys match this.
        const editConfig = ENTITY_CONFIG[view.entityType];
        return (
          <EntityCreate
            initialType={editConfig?.createType || "Figure"} // Pass type to find correct config/color
            editId={view.id}
            onSuccess={() => navigateToList(view.entityType)}
            onCancel={() => navigateToList(view.entityType)}
          />
        );

      case "detail":
        // For now, only Figures have a dedicated detail view.
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
            onEdit={(item) => {
                 console.log("App: onEdit triggered", item.id);
                 navigateToEdit(view.entityType, item.id);
            }}
            onDelete={async (id) => { 
                console.log("App: onDelete triggered", id);
                await deleteEntity(id); 
            }}
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

      {/* Top right settings button */}
      <button 
        onClick={() => setShowSettings(true)}
        className="absolute top-4 right-4 z-40 p-2 text-[#bfa275] hover:text-[#f2e6d8] bg-[#1d1b19]/80 border border-[#594d3f] hover:border-[#bfa275] transition-all rounded shadow-lg backdrop-blur"
      >
        <SettingsIcon size={24} />
      </button>

      {/* Settings Modal */}
      {showSettings && (
        <SettingsUI
          volume={volume}
          isMuted={isMuted}
          onVolumeChange={setVolume}
          onMuteToggle={() => setIsMuted(!isMuted)}
          onClose={() => setShowSettings(false)}
        />
      )}

      {/* Background Audio */}
      <audio 
        ref={audioRef}
        src="/audio/tiger_king.mp3" 
        autoPlay 
        loop 
        style={{ display: "none" }}
      />
    </div>
  );
}

export default App;
