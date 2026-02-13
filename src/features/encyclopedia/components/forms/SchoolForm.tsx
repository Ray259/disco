import { useState } from "react";
import { createSchoolOfThought, RelationDto } from "../../api";

interface SchoolFormProps {
  onSuccess: () => void;
  onCancel: () => void;
  extraRelations: RelationDto[];
}

export function SchoolForm({ onSuccess, onCancel, extraRelations }: SchoolFormProps) {
  const [loading, setLoading] = useState(false);
  const [error, setError] = useState<string | null>(null);

  const [name, setName] = useState("");
  const [description, setDescription] = useState("");

  const handleSubmit = async (e: React.FormEvent) => {
    e.preventDefault();
    setLoading(true);
    setError(null);

    try {
      if (!name) throw new Error("Designation/Name is required.");

      await createSchoolOfThought({
        name,
        description: description || undefined,
        relations: extraRelations,
      });

      setLoading(false);
      onSuccess();
    } catch (err: any) {
      setError(err.message || String(err));
      setLoading(false);
    }
  };

  const currentColor = "#ef4444"; // Red

  return (
    <form onSubmit={handleSubmit} className="space-y-12">
      {error && (
        <div className="mb-8 p-4 bg-[#2a1a1a] border-l-4 border-red-800 text-red-400 font-[var(--font-mono)] text-xs">
          [FAILURE] {error}
        </div>
      )}

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
          placeholder="ENTER IDEOLOGY..."
        />
      </div>

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
