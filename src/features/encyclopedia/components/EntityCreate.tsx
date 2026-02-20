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
  const findConfigKeyByType = (type: string) => {
    const entry = Object.entries(ENTITY_CONFIG).find(([_, cfg]) => cfg.createType === type);
    return entry ? entry[0] : "figures";
  };

  const initialKey = initialType ? findConfigKeyByType(initialType) : "figures";
  const [activeKey, setActiveKey] = useState<string>(initialKey);
  const [relations, setRelations] = useState<PendingRelation[]>([]);
  const [loading, setLoading] = useState(false);
  const [initialData, setInitialData] = useState<any>(null);

  const activeConfig = ENTITY_CONFIG[activeKey];
  const FormComponent = activeConfig.formComponent;
  const currentColor = activeConfig.color;

  useEffect(() => {
    if (editId && activeConfig.getById) {
        setLoading(true);
        activeConfig.getById(editId).then((data: any) => {
            setInitialData(data);
            setLoading(false);
        }).catch(() => setLoading(false));
    }
  }, [editId, activeKey]);

  const getApiRelations = () => relations.map(r => ({
    target_id: r.targetId,
    relation_type: r.role || "RELATED_TO"
  }));

  if (loading) return <div className="p-12 text-center">Loading...</div>;

  return (
    <div className="w-full h-full p-12 flex justify-center items-start overflow-y-auto bg-[var(--c-dark)]/70 backdrop-blur-sm">
      <div className="w-full max-w-2xl bg-[var(--c-dark)]/60 border border-[var(--c-border)] p-10">
        <div className="mb-12 text-center">
           <h2 className="text-3xl font-header uppercase tracking-wide mb-2" style={{ color: currentColor }}>
             {editId ? "Edit Entry" : "New Entry"}
           </h2>
           <p className="font-body italic text-[var(--c-dim)]">
             {editId ? " refining the record..." : "Cataloging the world, one fragment at a time..."}
           </p>
        </div>

        {!editId && (
            <div className="flex flex-wrap gap-2 border-b border-[var(--c-border)] pb-4 justify-center mb-12">
            {Object.entries(ENTITY_CONFIG).map(([key, config]) => (
                <button
                    key={key}
                    type="button"
                    onClick={() => setActiveKey(key)}
                    className={`px-3 py-1 text-xs font-header uppercase tracking-wider transition-all border border-transparent ${
                    activeKey === key 
                        ? "text-black font-bold" 
                        : "text-[var(--c-muted)] hover:text-white hover:border-[var(--c-border-light)]"
                    }`}
                    style={activeKey === key ? { backgroundColor: config.color } : {}}
                >
                    {config.createType}
                </button>
            ))}
            </div>
        )}

        <FormComponent 
            onSuccess={() => { setRelations([]); onSuccess(); }}
            onCancel={onCancel}
            extraRelations={getApiRelations()}
            initialValues={initialData}
            editId={editId}
        />
        
        <div className="mt-8 mb-20">
            <h3 className="text-sm font-bold uppercase mb-4 text-[var(--c-muted)]">Relations (Add New)</h3>
            <RelationManager relations={relations} onChange={setRelations} />
        </div>
      </div>
    </div>
  );
}
