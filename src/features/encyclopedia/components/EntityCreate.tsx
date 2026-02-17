import { useState, useEffect } from "react";
import { RelationManager, PendingRelation } from "./RelationManager";
import { ENTITY_CONFIG } from "./EntityConfig";

interface EntityCreateProps {
  onSuccess: () => void;
  onCancel: () => void;
  initialType?: string;
  editId?: string;
}

export function EntityCreate({ onSuccess, onCancel, initialType, editId }: EntityCreateProps) {
  // Config keys are lowercase plural (e.g. "figures"), but initialType might be "Figure"
  const findConfigKeyByType = (type: string) => {
    const entry = Object.entries(ENTITY_CONFIG).find(([_, cfg]) => cfg.createType === type);
    return entry ? entry[0] : "figures";
  };

  const initialKey = initialType ? findConfigKeyByType(initialType) : "figures";
  const [activeKey, setActiveKey] = useState<string>(initialKey);
  const [relations, setRelations] = useState<PendingRelation[]>([]);
  const [loading, setLoading] = useState(false);
  const [initialData, setInitialData] = useState<any>(null); // Store fetched data

  const activeConfig = ENTITY_CONFIG[activeKey];
  const FormComponent = activeConfig.formComponent;
  const currentColor = activeConfig.color;

  // Fetch data on mount if editId provided
  useEffect(() => {
    console.log("EntityCreate: mounted. editId=", editId, "activeKey=", activeKey);
    if (editId && activeConfig.getById) {
        setLoading(true);
        console.log("EntityCreate: Fetching data for", editId);
        activeConfig.getById(editId).then((data: any) => {
            console.log("EntityCreate: Fetched data", data);
            setInitialData(data);
            setLoading(false);
        }).catch((err: any) => {
            console.error("EntityCreate: Fetch failed", err);
            setLoading(false);
        });
    } else if (editId && !activeConfig.getById) {
        console.error("EntityCreate: No getById configured for", activeKey);
    }
  }, [editId, activeKey]);

  // Transform UI relations to API relations
  const getApiRelations = () => {
    return relations.map(r => ({
      target_id: r.targetId,
      relation_type: r.role || "RELATED_TO"
    }));
  };

  if (loading) return <div className="p-12 text-center">Loading...</div>;

  return (
    <div className="w-full h-full p-12 flex justify-center items-start overflow-y-auto">
      <div className="w-full max-w-2xl">
        <div className="mb-12 text-center">
           <h2 className="text-3xl font-[var(--font-header)] uppercase tracking-wide mb-2" style={{ color: currentColor }}>
             {editId ? "Edit Entry" : "New Entry"}
           </h2>
           <p className="font-[var(--font-body)] italic text-[#888]">
             {editId ? " refining the record..." : "Cataloging the world, one fragment at a time..."}
           </p>
        </div>

        {/* Classification Tabs - Hide in Edit Mode */}
        {!editId && (
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
        )}

        {/* Dynamic Form Render */}
        <FormComponent 
            onSuccess={() => {
                setRelations([]);
                onSuccess();
            }}
            onCancel={onCancel}
            extraRelations={getApiRelations()}
            initialValues={initialData} // Pass fetched data
            editId={editId} // Pass ID to indicate Update vs Create
        />
        
        {/* Relations Manager is shared across all types */}
        {/* Only show if not editing or if we can support it. For now show it but it won't pre-fill. */}
        <div className="mt-8 mb-20">
            <h3 className="text-sm font-bold uppercase mb-4 text-[#666]">Relations (Add New)</h3>
            <RelationManager relations={relations} onChange={setRelations} />
        </div>
      </div>
    </div>
  );
}
