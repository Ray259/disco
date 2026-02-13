import { useState } from "react";
import { RelationManager, PendingRelation } from "./RelationManager";
import { ENTITY_CONFIG } from "./EntityConfig";

interface EntityCreateProps {
  onSuccess: () => void;
  onCancel: () => void;
  initialType?: string;
}

export function EntityCreate({ onSuccess, onCancel, initialType }: EntityCreateProps) {
  // Config keys are lowercase plural (e.g. "figures"), but initialType might be "Figure"
  // We need a way to map them or pass the config key directly. 
  // For now, let's reverse lookup or expect the valid config key.
  
  // Actually, App.tsx passes "Figure" (the CreateType). 
  // Let's find the config entry that matches this CreateType or default to "figures".
  const findConfigKeyByType = (type: string) => {
    const entry = Object.entries(ENTITY_CONFIG).find(([_, cfg]) => cfg.createType === type);
    return entry ? entry[0] : "figures";
  };

  const initialKey = initialType ? findConfigKeyByType(initialType) : "figures";
  const [activeKey, setActiveKey] = useState<string>(initialKey);
  const [relations, setRelations] = useState<PendingRelation[]>([]);

  const activeConfig = ENTITY_CONFIG[activeKey];
  const FormComponent = activeConfig.formComponent;
  const currentColor = activeConfig.color;

  // Helper to transform UI relations to API relations
  const getApiRelations = () => {
    return relations.map(r => ({
      target_id: r.targetId,
      relation_type: r.role || "RELATED_TO"
    }));
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
           {Object.entries(ENTITY_CONFIG).map(([key, config]) => (
              <button
                key={key}
                type="button"
                onClick={() => setActiveKey(key)}
                className={`px-3 py-1 text-xs font-[var(--font-header)] uppercase tracking-wider transition-all border border-transparent ${
                  activeKey === key 
                    ? "text-black font-bold" 
                    : "text-[#666] hover:text-white hover:border-[#444]"
                }`}
                style={activeKey === key ? { backgroundColor: config.color } : {}}
              >
                {config.createType}
              </button>
           ))}
        </div>

        {/* Dynamic Form Render */}
        <FormComponent 
            onSuccess={() => {
                setRelations([]);
                onSuccess();
            }}
            onCancel={onCancel}
            extraRelations={getApiRelations()}
        />
        
        {/* Relations Manager is shared across all types */}
        <div className="mt-8 mb-20">
          <RelationManager relations={relations} onChange={setRelations} />
        </div>
      </div>
    </div>
  );
}
