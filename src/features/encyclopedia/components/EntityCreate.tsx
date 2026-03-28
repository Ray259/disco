import { useState, useEffect } from "react";
import { ENTITY_CONFIG } from "./EntityConfig";

interface Props {
  onSuccess: () => void;
  onCancel: () => void;
  initialType?: string;
  editName?: string;
}

export function EntityCreate({ onSuccess, onCancel, initialType, editName }: Props) {
  const initialKey = initialType
    ? Object.entries(ENTITY_CONFIG).find(([_, c]) => c.createType === initialType)?.[0] || "figures"
    : "figures";
  const [activeKey, setActiveKey] = useState(initialKey);
  const [loading, setLoading] = useState(false);
  const [initialData, setInitialData] = useState<any>(null);

  const cfg = ENTITY_CONFIG[activeKey];
  const Form = cfg.formComponent;

  useEffect(() => {
    if (editName && cfg.getById) {
      setLoading(true);
      cfg.getById(editName).then((d: any) => { setInitialData(d); setLoading(false); }).catch(() => setLoading(false));
    }
  }, [editName, activeKey]);

  if (loading) return <div className="p-12 text-center italic text-[#efac55] opacity-60">"Your mind is a database of facts. It enables you to draw on these facts innately..."</div>;

  return (
    <div className="w-full h-full p-12 flex justify-center items-start overflow-y-auto bg-[var(--c-dark)]/70 backdrop-blur-sm">
      <div className="w-full max-w-2xl bg-[var(--c-dark)]/60 border border-[var(--c-border)] p-10">
        <div className="mb-12 text-center">
          <h2 className="text-3xl font-header uppercase tracking-wide mb-2" style={{ color: cfg.color }}>
            {editName ? "Edit Entry" : "New Entry"}
          </h2>
          <p className="font-body italic text-[var(--c-dim)]">{editName ? "This is the ledger you found in the trash. It's full of notes written in a man's dense cursive. Have a closer look -- maybe it can be salvaged to start keeping notes on the case?" : "Your mangled brain would like you to know that there is a boxer called Contact Mike."}</p>
        </div>

        {!editName && (
          <div className="flex flex-wrap gap-2 border-b border-[var(--c-border)] pb-4 justify-center mb-12">
            {Object.entries(ENTITY_CONFIG).map(([key, c]) => (
              <button key={key} type="button" onClick={() => setActiveKey(key)}
                className={`px-3 py-1 text-xs font-header uppercase tracking-wider transition-all border border-transparent ${activeKey === key ? "text-black font-bold" : "text-[var(--c-muted)] hover:text-white hover:border-[var(--c-border-light)]"}`}
                style={activeKey === key ? { backgroundColor: c.color } : {}}>
                {c.createType}
              </button>
            ))}
          </div>
        )}

        <Form onSuccess={onSuccess} onCancel={onCancel} initialValues={initialData} editName={editName} />
      </div>
    </div>
  );
}
