import { useEffect, useState } from "react";
import { Figure, getAllFigures } from "../api";
import { RichContentDisplay } from "./RichContentDisplay";

interface FigureListProps {
  onSelect: (id: string) => void;
  onCreate: () => void;
}

export function FigureList({ onSelect, onCreate }: FigureListProps) {
  const [figures, setFigures] = useState<Figure[]>([]);
  const [loading, setLoading] = useState(true);
  const [error, setError] = useState<string | null>(null);

  useEffect(() => {
    getAllFigures()
      .then(setFigures)
      .catch((err) => setError(String(err)))
      .finally(() => setLoading(false));
  }, []);

  if (loading) return <div className="p-4 text-gray-400">Loading figures...</div>;
  if (error) return <div className="p-4 text-red-400">Error: {error}</div>;

  return (
    <div className="flex flex-col h-full">
      {/* Header */}
      <div className="p-6 border-b border-[#333] bg-[#1a1a1b]">
        <h1 className="text-3xl uppercase tracking-tighter text-[var(--disco-text-primary)] mb-1">
          Encyclopedia
        </h1>
        <div className="flex justify-between items-end">
           <span className="text-xs font-[var(--font-mono)] text-[var(--disco-text-secondary)]">
             {figures.length} ENTITIES ARCHIVED
           </span>
           <button 
             onClick={onCreate}
             className="text-xs bg-[var(--disco-accent-orange)] text-black px-3 py-1 font-bold uppercase hover:bg-white transition-colors"
           >
             + Add Thought
           </button>
        </div>
      </div>

      {/* List Area */}
      <div className="flex-1 overflow-y-auto p-4 space-y-2">
        {figures.map((figure) => (
          <div 
            key={figure.id} 
            onClick={() => onSelect(figure.id)}
            className="group relative cursor-pointer border border-transparent hover:border-[#444] hover:bg-[#222] transition-all p-3 pl-4"
          >
            {/* Hover Indicator Bar */}
            <div className="absolute left-0 top-0 bottom-0 w-1 bg-[var(--disco-accent-teal)] opacity-0 group-hover:opacity-100 transition-opacity" />
            
            <div className="flex justify-between items-baseline mb-1">
               <h3 className="text-lg font-[var(--font-header)] text-[var(--disco-text-primary)] group-hover:text-white leading-none">
                 {figure.name}
               </h3>
               {/* Use the first part of ID as a "skill check" value style */}
               <span className="text-[10px] font-[var(--font-mono)] text-[#555] group-hover:text-[var(--disco-accent-orange)]">
                 {figure.id.split('-')[0].toUpperCase()}
               </span>
            </div>
            
            <div className="text-sm font-[var(--font-body)] text-[var(--disco-text-secondary)] italic leading-tight line-clamp-2">
               <RichContentDisplay content={figure.primary_role} /> — <RichContentDisplay content={figure.primary_location} />
            </div>
          </div>
        ))}
        
        {figures.length === 0 && (
          <div className="p-8 text-center opacity-50">
             <div className="w-12 h-1 bg-[#333] mx-auto mb-4" />
             <p className="text-sm font-[var(--font-body)] italic">The cabinet is empty.</p>
          </div>
        )}
      </div>
    </div>
  );
}
