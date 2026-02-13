import { useState } from "react";
import { FigureList } from "./features/encyclopedia/components/FigureList";
import { FigureDetail } from "./features/encyclopedia/components/FigureDetail";
import { EntityCreate } from "./features/encyclopedia/components/EntityCreate";
import "./App.css";

type View = 
  | { type: "list" }
  | { type: "detail"; id: string }
  | { type: "create" };

function App() {
  const [view, setView] = useState<View>({ type: "list" });

  const renderView = () => {
    switch (view.type) {
      case "list":
        return (
          <FigureList 
            onSelect={(id) => setView({ type: "detail", id })} 
            onCreate={() => setView({ type: "create" })}
          />
        );
      case "detail":
        return (
          <FigureDetail 
            id={view.id} 
            onBack={() => setView({ type: "list" })} 
          />
        );
      case "create":
        return (
          <EntityCreate 
            onSuccess={() => setView({ type: "list" })} 
            onCancel={() => setView({ type: "list" })} 
          />
        );
    }
  };

  return (
    <div className="min-h-screen bg-black text-white w-full">
      <main className="w-full h-full">
        {renderView()}
      </main>
    </div>
  );
}

export default App;
