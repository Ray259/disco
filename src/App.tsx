import { useState } from "react";
import { Sidebar } from "./components/Layout/Sidebar";
import { FigureList } from "./features/encyclopedia/components/FigureList";
import { FigureDetail } from "./features/encyclopedia/components/FigureDetail";
import { GenericEntityList } from "./features/encyclopedia/components/GenericEntityList";
import { EntityCreate } from "./features/encyclopedia/components/EntityCreate";
import { RichContentDisplay } from "./features/encyclopedia/components/RichContentDisplay";
import { 
  getAllInstitutions, 
  getAllEvents, 
  getAllGeos, 
  getAllWorks, 
  getAllSchoolsOfThought,
  Institution,
  Event,
  Geo,
  Work,
  SchoolOfThought
} from "./features/encyclopedia/api";
import "./App.css";

type View = 
  | { type: "list"; entityType: string } // entityType match Sidebar IDs
  | { type: "detail"; entityType: string; id: string }
  | { type: "create"; initialType?: string };

function App() {
  const [view, setView] = useState<View>({ type: "list", entityType: "figures" });

  const renderContent = () => {
    switch (view.type) {
      case "create":
        return (
          <EntityCreate 
            initialType={view.initialType as any}
            onSuccess={() => setView({ type: "list", entityType: "figures" })} 
            onCancel={() => setView({ type: "list", entityType: "figures" })} 
          />
        );

      case "detail":
        if (view.entityType === "figures") {
          return (
            <FigureDetail 
              id={view.id} 
              onBack={() => setView({ type: "list", entityType: "figures" })} 
            />
          );
        }
        return <div className="p-8 text-gray-500">Detail view for {view.entityType} coming soon.</div>;

      case "list":
        switch (view.entityType) {
          case "figures":
            return (
              <FigureList 
                onSelect={(id) => setView({ type: "detail", entityType: "figures", id })} 
                onCreate={() => setView({ type: "create", initialType: "Figure" })}
              />
            );
          
          case "institutions":
            return (
              <GenericEntityList<Institution>
                title="Institutions"
                fetcher={getAllInstitutions}
                renderItem={(item) => (
                  <div className="border border-transparent hover:border-[#444] hover:bg-[#222] p-3 transition-all">
                    <h3 className="text-xl font-[var(--font-header)] text-[var(--disco-accent-yellow)]">{item.name}</h3>
                    {item.description && <div className="text-sm text-gray-400"><RichContentDisplay content={item.description} /></div>}
                  </div>
                )}
                onSelect={(item) => setView({ type: "detail", entityType: "institutions", id: item.id })}
                onCreate={() => setView({ type: "create", initialType: "Institution" })}
              />
            );

          case "events":
            return (
              <GenericEntityList<Event>
                title="Historical Events"
                fetcher={getAllEvents}
                renderItem={(item) => (
                  <div className="border border-transparent hover:border-[#444] hover:bg-[#222] p-3 transition-all flex justify-between items-center">
                    <div>
                        <h3 className="text-lg font-[var(--font-header)] text-white">{item.name}</h3>
                        {item.description && <div className="text-xs text-gray-500 line-clamp-1"><RichContentDisplay content={item.description} /></div>}
                    </div>
                    <div className="text-xs font-[var(--font-mono)] text-[#666]">
                        {item.date_range.start} — {item.date_range.end}
                    </div>
                  </div>
                )}
                onSelect={(item) => setView({ type: "detail", entityType: "events", id: item.id })}
                onCreate={() => setView({ type: "create", initialType: "Event" })}
              />
            );

          case "geos":
            return (
              <GenericEntityList<Geo>
                title="Geography"
                fetcher={getAllGeos}
                renderItem={(item) => (
                  <div className="border border-transparent hover:border-[#444] hover:bg-[#222] p-3 transition-all">
                    <div className="flex justify-between items-baseline">
                        <h3 className="text-lg font-[var(--font-header)] text-[var(--disco-accent-teal)]">{item.name}</h3>
                        {item.region && <span className="text-xs font-[var(--font-mono)] uppercase text-[#555]"><RichContentDisplay content={item.region} /></span>}
                    </div>
                    {item.description && <div className="text-sm text-gray-400 mt-1"><RichContentDisplay content={item.description} /></div>}
                  </div>
                )}
                onSelect={(item) => setView({ type: "detail", entityType: "geos", id: item.id })}
                onCreate={() => setView({ type: "create", initialType: "Geo" })}
              />
            );

          case "works":
            return (
              <GenericEntityList<Work>
                title="Bibliography"
                fetcher={getAllWorks}
                renderItem={(item) => (
                  <div className="border border-transparent hover:border-[#444] hover:bg-[#222] p-3 transition-all">
                    <h3 className="text-lg font-serif italic text-white">"{item.title}"</h3>
                    {item.summary && <div className="text-sm text-gray-500 mt-1 line-clamp-2"><RichContentDisplay content={item.summary} /></div>}
                  </div>
                )}
                onSelect={(item) => setView({ type: "detail", entityType: "works", id: item.id })}
                onCreate={() => setView({ type: "create", initialType: "Work" })}
              />
            );

          case "schools":
            return (
              <GenericEntityList<SchoolOfThought>
                title="Schools of Thought"
                fetcher={getAllSchoolsOfThought}
                renderItem={(item) => (
                  <div className="border border-transparent hover:border-[#444] hover:bg-[#222] p-3 transition-all">
                    <h3 className="text-xl font-[var(--font-header)] uppercase text-[var(--disco-accent-orange)]">{item.name}</h3>
                    {item.description && <div className="text-sm text-gray-400 mt-1 border-l-2 border-[#333] pl-2"><RichContentDisplay content={item.description} /></div>}
                  </div>
                )}
                onSelect={(item) => setView({ type: "detail", entityType: "schools", id: item.id })}
                onCreate={() => setView({ type: "create", initialType: "School" })}
              />
            );

          default:
            return <div className="p-8">Select a category from the sidebar.</div>;
        }
    }
  };

  return (
    <div className="min-h-screen bg-black text-white w-full flex overflow-hidden">
      <Sidebar 
        currentView={view.type === "list" ? view.entityType : ""} 
        onChangeView={(type) => setView({ type: "list", entityType: type })} 
      />
      <main className="flex-1 h-full relative overflow-hidden">
        {renderContent()}
      </main>
    </div>
  );
}

export default App;
