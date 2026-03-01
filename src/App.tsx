import { useState, useRef, useEffect } from "react";
import { SettingsUI } from "./components/SettingsUI";
import { Sidebar } from "./components/Layout/Sidebar";
import { EntityDetail } from "./features/encyclopedia/components/EntityDetail";
import { GenericEntityList } from "./features/encyclopedia/components/GenericEntityList";
import { EntityCreate } from "./features/encyclopedia/components/EntityCreate";
import { ENTITY_CONFIG } from "./features/encyclopedia/components/EntityConfig";
import { useEncyclopediaNavigation, EntityType as NavEntityType } from "./hooks/useEncyclopediaNavigation";
import { deleteEntity } from "./features/encyclopedia/api";
import "./App.css";

function App() {
  const { view, navigateToList, navigateToDetail, navigateToCreate, navigateToEdit } = useEncyclopediaNavigation();
  const [showSettings, setShowSettings] = useState(false);
  const [volume, setVolume] = useState(0.5);
  const [isMuted, setIsMuted] = useState(false);
  const audioRef = useRef<HTMLAudioElement>(null);

  useEffect(() => { if (audioRef.current) { audioRef.current.volume = volume; audioRef.current.muted = isMuted; } }, [volume, isMuted]);

  const getName = (item: any) => item.name || item.title || "";

  const renderContent = () => {
    switch (view.type) {
      case "create":
        return <EntityCreate initialType={view.initialType as any} onSuccess={() => navigateToList("figures")} onCancel={() => navigateToList("figures")} />;
      case "edit": {
        const cfg = ENTITY_CONFIG[view.entityType];
        return <EntityCreate initialType={cfg?.createType || "Figure"} editName={view.name} onSuccess={() => navigateToList(view.entityType)} onCancel={() => navigateToList(view.entityType)} />;
      }
      case "detail":
        return <EntityDetail entityType={view.entityType} name={view.name} onBack={() => navigateToList(view.entityType)} onEntityClick={(t, n) => navigateToDetail(t as NavEntityType, n)} />;
      case "list": {
        const cfg = ENTITY_CONFIG[view.entityType];
        if (!cfg) return <div className="p-8">Unknown category.</div>;
        return (
          <GenericEntityList
            title={cfg.title} fetcher={cfg.fetcher} renderItem={cfg.renderer}
            onSelect={(item) => navigateToDetail(view.entityType, getName(item))}
            onCreate={() => navigateToCreate(cfg.createType)}
            onEdit={(item) => navigateToEdit(view.entityType, getName(item))}
            onDelete={async (name) => await deleteEntity(cfg.createType, name)}
          />
        );
      }
    }
  };

  return (
    <div className="h-screen w-full flex overflow-hidden bg-cover bg-center bg-no-repeat" style={{ backgroundImage: "url('/images/library_hero.jpg')", backgroundColor: "var(--disco-bg)" }}>
      <Sidebar currentView={view.type === "list" ? view.entityType : ""} onChangeView={(t) => navigateToList(t as NavEntityType)} onOpenSettings={() => setShowSettings(true)} />
      <main className="flex-1 h-full relative overflow-hidden">{renderContent()}</main>
      {showSettings && <SettingsUI volume={volume} isMuted={isMuted} onVolumeChange={setVolume} onMuteToggle={() => setIsMuted(!isMuted)} onClose={() => setShowSettings(false)} />}
      <audio ref={audioRef} src="/audio/tiger_king.mp3" autoPlay loop style={{ display: "none" }} />
    </div>
  );
}

export default App;
