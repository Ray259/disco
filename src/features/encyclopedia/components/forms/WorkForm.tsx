import { useState, useEffect } from "react";
import { createWork, updateWork, RelationDto } from "../../api";
import { FormLayout, FormInput, FormTextArea } from "./SharedFormComponents";
import { DatePicker } from "../../../../components/DatePicker";

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
  const [endYear, setEndYear] = useState("");

  useEffect(() => {
    if (initialValues) {
        setTitle(initialValues.title || "");
        setSummary(getText(initialValues.summary));
    }
  }, [initialValues]);

  const handleSubmit = async (e: React.FormEvent) => {
    e.preventDefault();
    setLoading(true);
    setError(null);

    try {
      if (!title) throw new Error("Title is required.");

      const payload = { title, summary: summary || undefined, relations: extraRelations };

      if (editId) { await updateWork(editId, payload); }
      else { await createWork(payload); }

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
      onSubmit={handleSubmit} onCancel={onCancel} loading={loading} error={error} 
      color={currentColor} submitLabel={editId ? "Update Work" : "Create Work"}
    >
      <FormInput 
        label="1. Designation" value={title} 
        onChange={(e) => setTitle(e.target.value)} 
        placeholder="ENTER TITLE..." color={currentColor}
      />

      <FormTextArea 
        label="2. Abstract (Summary)" value={summary} 
        onChange={(e) => setSummary(e.target.value)} 
        rows={6} placeholder="Attributes..."
      />

      <div className="p-6 bg-[var(--c-surface)] border border-[var(--c-deep)] relative overflow-hidden">
         <div className="absolute top-0 right-0 w-4 h-4 border-t border-r border-[var(--c-border-light)]" />
         <h3 className="label-mono mb-6 text-center">Publication Date</h3>
         <div className="flex justify-center">
           <div className="w-1/2">
             <label className="block text-[10px] text-[var(--c-ghost)] mb-1 font-mono text-center">PUBLISHED</label>
             <DatePicker value={endYear} onChange={setEndYear} color="var(--disco-accent-yellow)" />
           </div>
         </div>
      </div>
    </FormLayout>
  );
}
