import { useState } from "react";
import { createInstitution } from "../../api";

interface InstitutionFormProps {
  onSuccess: () => void;
  onCancel: () => void;
}

export function InstitutionForm({ onSuccess, onCancel }: InstitutionFormProps) {
  const [loading, setLoading] = useState(false);
  const [error, setError] = useState<string | null>(null);

  const [name, setName] = useState("");
  const [description, setDescription] = useState("");
  const [startYear, setStartYear] = useState("");
  const [endYear, setEndYear] = useState("");

  const handleSubmit = async (e: React.FormEvent) => {
    e.preventDefault();
    setLoading(true);
    setError(null);

    try {
      if (!name) throw new Error("Designation/Name is required.");

      await createInstitution({
        name,
        founded_start: startYear || undefined,
        founded_end: endYear || undefined,
        description: description || undefined,
      });

      setLoading(false);
      onSuccess();
    } catch (err: any) {
      setError(err.message || String(err));
      setLoading(false);
    }
  };

  const currentColor = "var(--disco-accent-yellow)"; // Motorics

  return (
    <form onSubmit={handleSubmit} className="space-y-12">
      {error && (
        <div className="mb-8 p-4 bg-[#2a1a1a] border-l-4 border-red-800 text-red-400 font-[var(--font-mono)] text-xs">
          [FAILURE] {error}
        </div>
      )}

      {/* Designation */}
      <div className="group">
        <label className="block text-xs font-[var(--font-mono)] uppercase tracking-widest mb-2 transition-colors" style={{ color: currentColor }}>
          1. Designation
        </label>
        <input
          type="text"
          value={name}
          onChange={(e) => setName(e.target.value)}
          className="w-full bg-transparent border-b-2 border-[#333] py-2 text-3xl font-[var(--font-header)] text-white focus:outline-none transition-colors placeholder-[#333] uppercase"
          style={{ borderColor: "#333" }} 
          placeholder="ENTER NAME..."
        />
      </div>

      {/* Description */}
      <div className="group">
        <label className="block text-xs font-[var(--font-mono)] text-[#666] uppercase tracking-widest mb-2">
          Description / Manifest
        </label>
        <textarea
          value={description}
          onChange={(e) => setDescription(e.target.value)}
          rows={4}
          className="w-full bg-[#181818] border border-[#333] p-4 text-lg font-[var(--font-body)] text-white focus:outline-none placeholder-[#444]"
          style={{ borderColor: "#333" }}
          placeholder="Details..."
        />
      </div>

      {/* Temporal Coordinates */}
      <div className="p-6 bg-[#151515] border border-[#222] relative overflow-hidden">
         <div className="absolute top-0 right-0 w-4 h-4 border-t border-r border-[#444]" />
         <h3 className="text-xs font-[var(--font-mono)] text-[#666] uppercase mb-6 text-center tracking-widest">
           Temporal Coordinates
         </h3>
         <div className="flex items-center gap-6">
           <div className="flex-1">
             <label className="block text-[10px] text-[#444] mb-1 font-[var(--font-mono)] text-center">START</label>
             <input
               type="text"
               value={startYear}
               onChange={(e) => setStartYear(e.target.value)}
               className="w-full bg-[#0a0a0a] border border-[#333] p-3 text-center text-xl font-[var(--font-mono)] text-[var(--disco-accent-yellow)] focus:border-[var(--disco-accent-orange)] outline-none"
               placeholder="YYYY-MM-DD"
             />
           </div>
           <span className="text-[#333] text-xl">&mdash;</span>
           <div className="flex-1">
             <label className="block text-[10px] text-[#444] mb-1 font-[var(--font-mono)] text-center">END</label>
             <input
               type="text"
               value={endYear} 
               onChange={(e) => setEndYear(e.target.value)}
               className="w-full bg-[#0a0a0a] border border-[#333] p-3 text-center text-xl font-[var(--font-mono)] text-[var(--disco-accent-yellow)] focus:border-[var(--disco-accent-orange)] outline-none"
               placeholder="YYYY-MM-DD"
             />
           </div>
         </div>
      </div>

      {/* Actions */}
      <div className="flex justify-between items-center pt-8 border-t border-[#222]">
         <button
           type="button"
           onClick={onCancel}
           className="text-xs font-[var(--font-mono)] text-[#666] hover:text-white uppercase tracking-widest hover:underline"
         >
           [ Discard ]
         </button>

         <button
           type="submit"
           disabled={loading}
           className="bg-[#d4d4d8] text-black px-8 py-3 font-[var(--font-header)] text-xl uppercase tracking-widest hover:bg-white transition-all disabled:opacity-50"
           style={{ backgroundColor: currentColor }}
         >
            {loading ? "PROCESSING..." : "INTERNALIZE"}
         </button>
      </div>
    </form>
  );
}
