import { useState, useEffect } from "react";
import { createWork, updateWork, ContentSegment } from "../../api";
import { FormLayout, FormInput } from "./SharedFormComponents";
import { RichContentEditor } from "../RichContentEditor";
import { extractSegments } from "../RichContentEditorTypes";
import { DatePicker } from "../../../../components/DatePicker";

interface Props { onSuccess: () => void; onCancel: () => void; initialValues?: any; editName?: string; }

export function WorkForm({ onSuccess, onCancel, initialValues, editName }: Props) {
  const [loading, setLoading] = useState(false);
  const [error, setError] = useState<string | null>(null);
  const [title, setTitle] = useState("");
  const [summary, setSummary] = useState<ContentSegment[]>([]);
  const [endYear, setEndYear] = useState("");

  useEffect(() => {
    if (initialValues) { setTitle(initialValues.title || ""); setSummary(extractSegments(initialValues.summary)); }
  }, [initialValues]);

  const handleSubmit = async (e: React.FormEvent) => {
    e.preventDefault(); setLoading(true); setError(null);
    try {
      if (!title) throw new Error("Title is required.");
      const payload = { title, summary: summary.length > 0 ? { segments: summary } : undefined };
      if (editName) { await updateWork(editName, payload); } else { await createWork(payload); }
      onSuccess();
    } catch (err: any) { setError(err.message || String(err)); }
    finally { setLoading(false); }
  };

  return (
    <FormLayout 
      onSubmit={handleSubmit} 
      onCancel={onCancel} 
      loading={loading} 
      error={error} 
      theme="Work" 
      submitLabel={editName ? "Update Work" : "Create Work"}
    >
      <FormInput 
        label="1. Designation" 
        value={title} 
        onChange={(e) => setTitle(e.target.value)} 
        placeholder="ENTER TITLE..." 
        themed 
      />
      <RichContentEditor label="2. Abstract (Summary)" value={summary} onChange={setSummary} placeholder="Attributes..." multiline />
      <div className="p-6 bg-[var(--c-surface)] border border-[var(--c-deep)] relative overflow-hidden">
        <div className="absolute top-0 right-0 w-4 h-4 border-t border-r border-[var(--c-border-light)]" />
        <h3 className="label-mono mb-6 text-center">Publication Date</h3>
        <div className="flex justify-center">
          <div className="w-1/2">
            <label className="block text-[10px] text-[var(--c-ghost)] mb-1 font-mono text-center">PUBLISHED</label>
            <DatePicker value={endYear} onChange={setEndYear} />
          </div>
        </div>
      </div>
    </FormLayout>
  );
}
