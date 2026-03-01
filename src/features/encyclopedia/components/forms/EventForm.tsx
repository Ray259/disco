import { useState, useEffect } from "react";
import { createEvent, updateEvent, ContentSegment } from "../../api";
import { FormLayout, FormInput, TemporalCoordinates } from "./SharedFormComponents";
import { RichContentEditor } from "../RichContentEditor";
import { extractSegments } from "../RichContentEditorTypes";

function extractYear(s: string | undefined | null): string { return s?.match(/^(-?\d+)/)?.[1] || ""; }

interface Props { onSuccess: () => void; onCancel: () => void; initialValues?: any; editName?: string; }

export function EventForm({ onSuccess, onCancel, initialValues, editName }: Props) {
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
      if (initialValues.date_range) { setStartYear(extractYear(initialValues.date_range.start)); setEndYear(extractYear(initialValues.date_range.end)); }
    }
  }, [initialValues]);

  const handleSubmit = async (e: React.FormEvent) => {
    e.preventDefault(); setLoading(true); setError(null);
    try {
      if (!name) throw new Error("Name is required.");
      if (!startYear || !endYear) throw new Error("Event requires temporal bounds.");
      const payload = { name, start_date: startYear, end_date: endYear, description: description.length > 0 ? { segments: description } : undefined };
      if (editName) { await updateEvent(editName, payload); } else { await createEvent(payload); }
      onSuccess();
    } catch (err: any) { setError(err.message || String(err)); }
    finally { setLoading(false); }
  };

  const c = "var(--disco-accent-purple)";
  return (
    <FormLayout onSubmit={handleSubmit} onCancel={onCancel} loading={loading} error={error} color={c} submitLabel={editName ? "Update Event" : "Create Event"}>
      <FormInput label="1. Designation" value={name} onChange={(e) => setName(e.target.value)} placeholder="ENTER NAME..." color={c} />
      <RichContentEditor label="Description / Manifest" value={description} onChange={setDescription} placeholder="Details..." multiline />
      <TemporalCoordinates startYear={startYear} endYear={endYear} onStartChange={setStartYear} onEndChange={setEndYear} color={c} />
    </FormLayout>
  );
}
