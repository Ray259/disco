import { useEffect, useState } from "react";

interface GenericEntityListProps<T> {
  title: string;
  fetcher: () => Promise<T[]>;
  renderItem: (item: T) => React.ReactNode;
  onSelect: (item: T) => void;
  onCreate: () => void;
  onEdit: (item: T) => void;
  onDelete: (id: string) => void;
}

export function GenericEntityList<T extends { id: string }>({ 
  title, 
  fetcher, 
  renderItem, 
  onSelect,
  onCreate,
  onEdit,
  onDelete
}: GenericEntityListProps<T>) {
  const [items, setItems] = useState<T[]>([]);
  const [loading, setLoading] = useState(true);
  const [error, setError] = useState<string | null>(null);

  const loadData = async () => {
    setLoading(true);
    try {
      const data = await fetcher();
      setItems(data);
      setError(null);
    } catch (err: any) {
      setError(err.toString());
    } finally {
      setLoading(false);
    }
  };

  useEffect(() => {
    loadData();
  }, [fetcher]);

  const [deleteId, setDeleteId] = useState<string | null>(null);

  const handleDeleteClick = (e: React.MouseEvent, id: string) => {
    e.stopPropagation();
    setDeleteId(id);
  };

  const confirmDelete = async () => {
    if (!deleteId) return;
    try {
        await onDelete(deleteId);
        loadData(); 
    } catch (err) {
        console.error(err);
    } finally {
        setDeleteId(null);
    }
  };

  return (
    <div className="flex flex-col h-full bg-[#0a0a0a] relative">
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

      {loading && <div className="p-8 text-gray-500 font-[var(--font-mono)] uppercase">Loading {title}...</div>}
      {error && <div className="p-8 text-red-500 font-[var(--font-mono)] uppercase">Error: {error}</div>}

      {/* List Area */}
      <div className="flex-1 overflow-y-auto p-4 space-y-2 custom-scrollbar">
        {items.map((item) => (
          <div 
            key={item.id} 
            onClick={() => onSelect(item)}
            className="group relative cursor-pointer border border-[#333] bg-[#0a0a0a] hover:border-white transition-all p-4"
          >
            <div className="pr-20">
                {renderItem(item)}
            </div>
            
            <div className="absolute top-4 right-4 flex gap-2 z-50 opacity-100">
                <button
                  onClick={(e) => { 
                      e.stopPropagation(); 
                      onEdit(item); 
                  }}
                  className="px-2 py-1 text-[10px] border border-[#444] text-[#888] hover:border-white hover:text-white uppercase tracking-wider bg-black"
                >
                  Edit
                </button>
                <button
                  onClick={(e) => handleDeleteClick(e, item.id)}
                  className="px-2 py-1 text-[10px] border border-[#444] text-[#888] hover:border-red-500 hover:text-red-500 uppercase tracking-wider bg-black"
                >
                  Delete
                </button>
            </div>
          </div>
        ))}
        
        {!loading && items.length === 0 && (
          <div className="p-12 text-center opacity-40">
             <div className="w-16 h-1 bg-[#333] mx-auto mb-4" />
             <p className="text-sm font-[var(--font-body)] italic">No records found in this category.</p>
          </div>
        )}
      </div>

      {/* Custom Delete Modal */}
      {deleteId && (
        <div className="absolute inset-0 bg-black/80 flex items-center justify-center z-[100] backdrop-blur-sm">
            <div className="bg-[#1a1a1b] border border-[#333] p-8 max-w-sm w-full shadow-2xl">
                <h3 className="text-xl text-white font-[var(--font-header)] uppercase mb-4 text-center">Confirm Deletion</h3>
                <p className="text-[#888] text-sm font-[var(--font-mono)] text-center mb-8">
                    This action cannot be undone. The entity will be permanently removed.
                </p>
                <div className="flex gap-4">
                    <button 
                        onClick={(e) => { e.stopPropagation(); setDeleteId(null); }}
                        className="flex-1 border border-[#333] py-3 text-white font-[var(--font-mono)] hover:bg-[#333] uppercase text-xs"
                    >
                        Cancel
                    </button>
                    <button 
                        onClick={(e) => { e.stopPropagation(); confirmDelete(); }}
                        className="flex-1 bg-red-600 text-white py-3 font-[var(--font-mono)] hover:bg-red-700 uppercase text-xs"
                    >
                        Delete
                    </button>
                </div>
            </div>
        </div>
      )}
    </div>
  );
}
