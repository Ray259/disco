import { useState, useEffect } from "react";
import { createWork, updateWork, RelationDto } from "../../api";
import { FormLayout, FormInput, FormTextArea } from "./SharedFormComponents";

interface WorkFormProps {
  onSuccess: () => void;
  onCancel: () => void;
  extraRelations: RelationDto[];
  initialValues?: any;
  editId?: string;
}

const getText = (content: any) => {
    if (!content) return "";
    if (typeof content === "string") return content;
    if (content.segments && Array.isArray(content.segments)) {
        return content.segments.map((s: any) => s.Text || "").join("");
    }
    return "";
};

export function WorkForm({ onSuccess, onCancel, extraRelations, initialValues, editId }: WorkFormProps) {
  const [loading, setLoading] = useState(false);
  const [error, setError] = useState<string | null>(null);

  const [title, setTitle] = useState("");
  const [summary, setSummary] = useState("");
  const [endYear, setEndYear] = useState(""); // Used as Publication Date

  useEffect(() => {
    if (initialValues) {
        setTitle(initialValues.title || "");
        setSummary(getText(initialValues.summary));
        // Work doesn't have a structured date range in the same way, or does it?
        // Checking api/index.ts: Work has id, title, summary. No date field explicitly in `Work` interface.
        // But `CreateWorkRequest` didn't have date either.
        // `WorkForm` has `endYear` state but it wasn't used in `createWork` payload in previous version?
        // Checking previous version in Step 1995:
        // `await createWork({ title: name, summary: description ... })`. 
        // `endYear` state existed but was UNUSED in payload!
        // So I will ignore it for now or check if I should add it.
        // The form has a "Publication Date" input.
        // If the backend doesn't support it, I can't save it.
        // I'll leave it as is (unused) to match previous behavior, but I'll try to preserve it if it was somehow used?
        // No, it was just visual.
    }
  }, [initialValues]);

  const handleSubmit = async (e: React.FormEvent) => {
    e.preventDefault();
    setLoading(true);
    setError(null);

    try {
      if (!title) throw new Error("Title is required.");

      const payload = {
        title,
        summary: summary || undefined,
        relations: extraRelations,
      };

      if (editId) {
          await updateWork(editId, payload);
      } else {
          await createWork(payload);
      }

      setLoading(false);
      onSuccess();
    } catch (err: any) {
      setError(err.message || String(err));
      setLoading(false);
    }
  };

  const currentColor = "#d4d4d8"; // Zinc-300

  return (
    <FormLayout 
      onSubmit={handleSubmit} 
      onCancel={onCancel} 
      loading={loading} 
      error={error} 
      color={currentColor}
      submitLabel={editId ? "Update Work" : "Create Work"}
    >
      <FormInput 
        label="1. Designation" 
        value={title} 
        onChange={(e) => setTitle(e.target.value)} 
        placeholder="ENTER TITLE..." 
        color={currentColor}
      />

      <FormTextArea 
        label="2. Abstract (Summary)" 
        value={summary} 
        onChange={(e) => setSummary(e.target.value)} 
        rows={6}
        placeholder="Attributes..."
      />

       {/* Custom block for single date to match aesthetic - visual only for now as per previous implementation */}
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
