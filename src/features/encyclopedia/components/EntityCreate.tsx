import { useState } from "react";
import { FigureForm } from "./forms/FigureForm";
import { InstitutionForm } from "./forms/InstitutionForm";
import { EventForm } from "./forms/EventForm";
import { GeoForm } from "./forms/GeoForm";
import { WorkForm } from "./forms/WorkForm";
import { SchoolForm } from "./forms/SchoolForm";
import { RelationManager, PendingRelation } from "./RelationManager";

interface EntityCreateProps {
  onSuccess: () => void;
  onCancel: () => void;
  initialType?: EntityType;
}

export type EntityType = "Figure" | "Institution" | "Event" | "Geo" | "Work" | "School";

export function EntityCreate({ onSuccess, onCancel, initialType }: EntityCreateProps) {
  const [type, setType] = useState<EntityType>(initialType || "Figure");
  const [relations, setRelations] = useState<PendingRelation[]>([]);

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

  // Helper to transform UI relations to API relations
  const getApiRelations = () => {
    return relations.map(r => ({
      target_id: r.targetId,
      relation_type: r.role || "RELATED_TO"
    }));
  };

  const renderForm = () => {
    const commonProps = {
      onSuccess: () => {
        setRelations([]); // Clear relations on success
        onSuccess();
      },
      onCancel,
      extraRelations: getApiRelations()
    };

    switch (type) {
      case "Figure": return <FigureForm {...commonProps} />;
      case "Institution": return <InstitutionForm {...commonProps} />;
      case "Event": return <EventForm {...commonProps} />;
      case "Geo": return <GeoForm {...commonProps} />;
      case "Work": return <WorkForm {...commonProps} />;
      case "School": return <SchoolForm {...commonProps} />;
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
        
        {/* Relations Manager is shared across all types */}
        <div className="mt-8 mb-20">
          <RelationManager relations={relations} onChange={setRelations} />
        </div>
      </div>
    </div>
  );
}
