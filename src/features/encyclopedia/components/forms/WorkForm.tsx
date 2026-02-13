import { useState } from "react";
import { createWork, RelationDto } from "../../api";
import { FormLayout, FormInput, FormTextArea } from "./SharedFormComponents";

interface WorkFormProps {
  onSuccess: () => void;
  onCancel: () => void;
  extraRelations: RelationDto[];
}

export function WorkForm({ onSuccess, onCancel, extraRelations }: WorkFormProps) {
  const [loading, setLoading] = useState(false);
  const [error, setError] = useState<string | null>(null);

  const [name, setName] = useState("");
  const [description, setDescription] = useState("");
  const [endYear, setEndYear] = useState(""); // Used as Publication Date

  const handleSubmit = async (e: React.FormEvent) => {
    e.preventDefault();
    setLoading(true);
    setError(null);

    try {
      if (!name) throw new Error("Designation/Name is required.");

      await createWork({
        title: name,
        summary: description || undefined,
        relations: extraRelations,
      });

      setLoading(false);
      onSuccess();
    } catch (err: any) {
      setError(err.message || String(err));
      setLoading(false);
    }
  };

  const currentColor = "#d4d4d8";

  return (
    <FormLayout 
      onSubmit={handleSubmit} 
      onCancel={onCancel} 
      loading={loading} 
      error={error} 
      color={currentColor}
    >
      <FormInput 
        label="1. Designation" 
        value={name} 
        onChange={(e) => setName(e.target.value)} 
        placeholder="ENTER TITLE..." 
        color={currentColor}
      />

      <FormTextArea 
        label="2. Abstract (Summary)" 
        value={description} 
        onChange={(e) => setDescription(e.target.value)} 
        placeholder="Details..."
      />

       {/* Custom block for single date to match aesthetic */}
      <div className="p-6 bg-[#151515] border border-[#222] relative overflow-hidden">
         <div className="absolute top-0 right-0 w-4 h-4 border-t border-r border-[#444]" />
         <h3 className="text-xs font-[var(--font-mono)] text-[#666] uppercase mb-6 text-center tracking-widest">
           Publication Date
         </h3>
         <div className="flex justify-center">
           <div className="w-1/2">
             <label className="block text-[10px] text-[#444] mb-1 font-[var(--font-mono)] text-center">
               PUBLISHED
             </label>
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
    </FormLayout>
  );
}
