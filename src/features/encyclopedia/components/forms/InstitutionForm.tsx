import { useState, useEffect } from "react";
import { createInstitution, updateInstitution, ContentSegment } from "../../api";
import { FormLayout, FormInput, TemporalCoordinates } from "./SharedFormComponents";
import { RichContentEditor } from "../RichContentEditor";
import { extractSegments } from "../RichContentEditorTypes";

function extractYear(s: string | undefined | null): string { return s?.match(/^(-?\d+)/)?.[1] || ""; }

interface Props { onSuccess: () => void; onCancel: () => void; initialValues?: any; editName?: string; }

export function InstitutionForm({ onSuccess, onCancel, initialValues, editName }: Props) {
  const [loading, setLoading] = useState(false);
  const [error, setError] = useState<string | null>(null);
  const [name, setName] = useState("");
  const [description, setDescription] = useState<ContentSegment[]>([]);
  const [startYear, setStartYear] = useState("");
  const [endYear, setEndYear] = useState("");

  useEffect(() => {
    if (initialValues) {
      setName(initialValues.name || "");
      setDescription(extractSegments(initialValues.description));
      if (initialValues.founded) { setStartYear(extractYear(initialValues.founded.start)); setEndYear(extractYear(initialValues.founded.end)); }
    }
  }, [initialValues]);

  const handleSubmit = async (e: React.FormEvent) => {
    e.preventDefault(); setLoading(true); setError(null);
    try {
      if (!name) throw new Error("Name is required.");
      const payload = { name, founded_start: startYear || undefined, founded_end: endYear || undefined, description: description.length > 0 ? { segments: description } : undefined };
      if (editName) { await updateInstitution(editName, payload); } else { await createInstitution(payload); }
      onSuccess();
    } catch (err: any) { setError(err.message || String(err)); }
    finally { setLoading(false); }
  };

  const c = "var(--disco-accent-yellow)";
  return (
    <FormLayout onSubmit={handleSubmit} onCancel={onCancel} loading={loading} error={error} color={c} submitLabel={editName ? "Update Institution" : "Create Institution"}>
      <FormInput label="1. Designation" value={name} onChange={(e) => setName(e.target.value)} placeholder="ENTER NAME..." color={c} />
      <RichContentEditor label="Description / Manifest" value={description} onChange={setDescription} placeholder="Details..." multiline />
      <TemporalCoordinates startYear={startYear} endYear={endYear} onStartChange={setStartYear} onEndChange={setEndYear} />
    </FormLayout>
  );
}
