import { useState, useEffect } from "react";
import { createSchoolOfThought, updateSchoolOfThought, ContentSegment } from "../../api";
import { FormLayout, FormInput } from "./SharedFormComponents";
import { RichContentEditor } from "../RichContentEditor";
import { extractSegments } from "../RichContentEditorTypes";

interface Props { onSuccess: () => void; onCancel: () => void; initialValues?: any; editName?: string; }

export function SchoolForm({ onSuccess, onCancel, initialValues, editName }: Props) {
  const [loading, setLoading] = useState(false);
  const [error, setError] = useState<string | null>(null);
  const [name, setName] = useState("");
  const [description, setDescription] = useState<ContentSegment[]>([]);

  useEffect(() => {
    if (initialValues) { setName(initialValues.name || ""); setDescription(extractSegments(initialValues.description)); }
  }, [initialValues]);

  const handleSubmit = async (e: React.FormEvent) => {
    e.preventDefault(); setLoading(true); setError(null);
    try {
      if (!name) throw new Error("Name is required.");
      const payload = { name, description: description.length > 0 ? { segments: description } : undefined };
      if (editName) { await updateSchoolOfThought(editName, payload); } else { await createSchoolOfThought(payload); }
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
      theme="SchoolOfThought" 
      submitLabel={editName ? "Update School" : "Create School"}
    >
      <FormInput 
        label="1. Designation" 
        value={name} 
        onChange={(e) => setName(e.target.value)} 
        placeholder="ENTER IDEOLOGY..." 
        themed 
      />
      <RichContentEditor label="Description / Manifest" value={description} onChange={setDescription} placeholder="Attributes..." multiline />
    </FormLayout>
  );
}
