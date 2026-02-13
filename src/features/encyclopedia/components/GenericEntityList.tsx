import { useEffect, useState } from "react";

interface GenericEntityListProps<T> {
  title: string;
  fetcher: () => Promise<T[]>;
  renderItem: (item: T) => React.ReactNode;
  onSelect: (item: T) => void;
  onCreate: () => void;
}

export function GenericEntityList<T extends { id: string }>({ 
  title, 
  fetcher, 
  renderItem, 
  onSelect, 
  onCreate 
}: GenericEntityListProps<T>) {
  const [items, setItems] = useState<T[]>([]);
  const [loading, setLoading] = useState(true);
  const [error, setError] = useState<string | null>(null);

  useEffect(() => {
    fetcher()
      .then(setItems)
      .catch((err) => setError(String(err)))
      .finally(() => setLoading(false));
  }, [fetcher]);

  if (loading) return <div className="p-8 text-gray-500 font-[var(--font-mono)] uppercase">Loading {title}...</div>;
  if (error) return <div className="p-8 text-red-500 font-[var(--font-mono)] uppercase">Error: {error}</div>;

  return (
    <div className="flex flex-col h-full bg-[#0a0a0a]">
      {/* Header */}
      <div className="p-6 border-b border-[#333] bg-[#1a1a1b] flex justify-between items-end">
        <div>
           <h1 className="text-3xl uppercase tracking-tighter text-[var(--disco-text-primary)] mb-1 font-[var(--font-header)]">
            {title}
          </h1>
          <span className="text-xs font-[var(--font-mono)] text-[var(--disco-text-secondary)]">
             {items.length} ENTITIES ARCHIVED
           </span>
        </div>
        <button 
          onClick={onCreate}
          className="text-xs bg-[var(--disco-accent-orange)] text-black px-4 py-2 font-bold uppercase hover:bg-white transition-colors tracking-widest font-[var(--font-mono)]"
        >
          + Add New
        </button>
      </div>

      {/* List Area */}
      <div className="flex-1 overflow-y-auto p-4 space-y-2">
        {items.map((item) => (
          <div 
            key={item.id} 
            onClick={() => onSelect(item)}
            className="cursor-pointer"
          >
            {renderItem(item)}
          </div>
        ))}
        
        {items.length === 0 && (
          <div className="p-12 text-center opacity-40">
             <div className="w-16 h-1 bg-[#333] mx-auto mb-4" />
             <p className="text-sm font-[var(--font-body)] italic">No records found in this category.</p>
          </div>
        )}
      </div>
    </div>
  );
}
