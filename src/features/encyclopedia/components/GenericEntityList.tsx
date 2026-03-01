import { useEffect, useState } from "react";

interface Props<T> {
  title: string;
  fetcher: () => Promise<T[]>;
  renderItem: (item: T) => React.ReactNode;
  onSelect: (item: T) => void;
  onCreate: () => void;
  onEdit: (item: T) => void;
  onDelete: (name: string) => void;
}

export function GenericEntityList<T extends { name: string } | { title: string }>({
  title, fetcher, renderItem, onSelect, onCreate, onEdit, onDelete
}: Props<T>) {
  const [items, setItems] = useState<T[]>([]);
  const [loading, setLoading] = useState(true);
  const [error, setError] = useState<string | null>(null);
  const [deleteName, setDeleteName] = useState<string | null>(null);

  const getName = (item: T) => "name" in item ? item.name : (item as any).title;

  const loadData = async () => {
    setLoading(true);
    try { setItems(await fetcher()); setError(null); }
    catch (err: any) { setError(err.toString()); }
    finally { setLoading(false); }
  };

  useEffect(() => { loadData(); }, [fetcher]);

  const confirmDelete = async () => {
    if (!deleteName) return;
    try { await onDelete(deleteName); loadData(); }
    catch (err) { console.error(err); }
    finally { setDeleteName(null); }
  };

  return (
    <div className="flex flex-col h-full bg-[var(--c-dark)]/70 backdrop-blur-sm relative">
      <div className="p-6 border-b border-[var(--c-border)] bg-[var(--c-panel)]/80 flex justify-between items-end">
        <div>
          <h1 className="text-3xl uppercase tracking-tighter text-[var(--disco-text-primary)] mb-1 font-header">{title}</h1>
          <span className="text-mono-sm text-[var(--disco-text-secondary)]">{items.length} ENTITIES ARCHIVED</span>
        </div>
        <button onClick={onCreate} className="text-xs bg-[var(--disco-accent-orange)] text-black px-4 py-2 font-bold uppercase hover:bg-white transition-colors tracking-widest font-mono">+ Add New</button>
      </div>

      {loading && <div className="p-8 text-[var(--c-muted)] font-mono uppercase">Loading {title}...</div>}
      {error && <div className="p-8 text-red-500 font-mono uppercase">Error: {error}</div>}

      <div className="flex-1 overflow-y-auto p-4 space-y-2">
        {items.map((item) => (
          <div key={getName(item)} onClick={() => onSelect(item)}
            className="group cursor-pointer border border-[var(--c-border)] bg-[var(--c-dark)]/60 hover:border-white transition-all p-4 flex justify-between items-center gap-4">
            <div className="flex-1 overflow-hidden">{renderItem(item)}</div>
            <div className="flex gap-2 shrink-0">
              <button onClick={(e) => { e.stopPropagation(); onEdit(item); }} className="btn-action">Edit</button>
              <button onClick={(e) => { e.stopPropagation(); setDeleteName(getName(item)); }} className="btn-action btn-action--danger">Delete</button>
            </div>
          </div>
        ))}
        {!loading && items.length === 0 && (
          <div className="p-12 text-center opacity-40">
            <div className="w-16 h-1 bg-[var(--c-border)] mx-auto mb-4" />
            <p className="text-sm font-body italic">No records found in this category.</p>
          </div>
        )}
      </div>

      {deleteName && (
        <div className="absolute inset-0 bg-black/80 flex items-center justify-center z-[100] backdrop-blur-sm">
          <div className="bg-[var(--c-panel)] border border-[var(--c-border)] p-8 max-w-sm w-full shadow-2xl">
            <h3 className="text-xl text-white font-header uppercase mb-4 text-center">Confirm Deletion</h3>
            <p className="text-[var(--c-dim)] text-sm font-mono text-center mb-8">This action cannot be undone.</p>
            <div className="flex gap-4">
              <button onClick={() => setDeleteName(null)} className="flex-1 border border-[var(--c-border)] py-3 text-white font-mono hover:bg-[var(--c-border)] uppercase text-xs">Cancel</button>
              <button onClick={confirmDelete} className="flex-1 bg-red-600 text-white py-3 font-mono hover:bg-red-700 uppercase text-xs">Delete</button>
            </div>
          </div>
        </div>
      )}
    </div>
  );
}
