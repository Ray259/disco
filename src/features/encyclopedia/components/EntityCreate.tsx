import { useState } from "react";
import { FigureForm } from "./forms/FigureForm";
import { InstitutionForm } from "./forms/InstitutionForm";
import { EventForm } from "./forms/EventForm";
import { GeoForm } from "./forms/GeoForm";
import { WorkForm } from "./forms/WorkForm";
import { SchoolForm } from "./forms/SchoolForm";

interface EntityCreateProps {
  onSuccess: () => void;
  onCancel: () => void;
}

type EntityType = "Figure" | "Institution" | "Event" | "Geo" | "Work" | "School";

export function EntityCreate({ onSuccess, onCancel }: EntityCreateProps) {
  const [type, setType] = useState<EntityType>("Figure");

  const getTypeColor = (t: EntityType) => {
    switch (t) {
      case "Figure": return "var(--disco-accent-orange)"; // Intellect
      case "Institution": return "var(--disco-accent-yellow)"; // Motorics
      case "Event": return "var(--disco-accent-purple)"; // Physique
      case "Geo": return "var(--disco-accent-teal)"; // Psyche
      case "Work": return "#d4d4d8"; // Neutral
      case "School": return "#ef4444"; // Red (Ideological)
      default: return "white";
    }
  };

  const currentColor = getTypeColor(type);

  const renderForm = () => {
    switch (type) {
      case "Figure": return <FigureForm onSuccess={onSuccess} onCancel={onCancel} />;
      case "Institution": return <InstitutionForm onSuccess={onSuccess} onCancel={onCancel} />;
      case "Event": return <EventForm onSuccess={onSuccess} onCancel={onCancel} />;
      case "Geo": return <GeoForm onSuccess={onSuccess} onCancel={onCancel} />;
      case "Work": return <WorkForm onSuccess={onSuccess} onCancel={onCancel} />;
      case "School": return <SchoolForm onSuccess={onSuccess} onCancel={onCancel} />;
      default: return null;
    }
  };

  return (
    <div className="w-full h-full p-12 flex justify-center items-start overflow-y-auto">
      <div className="w-full max-w-2xl">
        <div className="mb-12 text-center">
           <h2 className="text-3xl font-[var(--font-header)] uppercase tracking-wide mb-2" style={{ color: currentColor }}>
             New Entry
           </h2>
           <p className="font-[var(--font-body)] italic text-[#888]">
             "Cataloging the world, one fragment at a time..."
           </p>
        </div>

        {/* Classification Tabs */}
        <div className="flex flex-wrap gap-2 border-b border-[#333] pb-4 justify-center mb-12">
           {["Figure", "Institution", "Event", "Geo", "Work", "School"].map((t) => (
              <button
                key={t}
                type="button"
                onClick={() => setType(t as EntityType)}
                className={`px-3 py-1 text-xs font-[var(--font-header)] uppercase tracking-wider transition-all border border-transparent ${
                  type === t 
                    ? `bg-[${getTypeColor(t as EntityType)}] text-black font-bold` 
                    : "text-[#666] hover:text-white hover:border-[#444]"
                }`}
                style={type === t ? { backgroundColor: getTypeColor(t as EntityType) } : {}}
              >
                {t}
              </button>
           ))}
        </div>

        {renderForm()}
      </div>
    </div>
  );
}
