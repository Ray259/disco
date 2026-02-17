import { useState, useEffect } from "react";
import { createEvent, updateEvent, RelationDto } from "../../api";
import { FormLayout, FormInput, FormTextArea, TemporalCoordinates } from "./SharedFormComponents";

interface EventFormProps {
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

export function EventForm({ onSuccess, onCancel, extraRelations, initialValues, editId }: EventFormProps) {
  const [loading, setLoading] = useState(false);
  const [error, setError] = useState<string | null>(null);

  const [name, setName] = useState("");
  const [description, setDescription] = useState("");
  const [startYear, setStartYear] = useState("");
  const [endYear, setEndYear] = useState("");

  useEffect(() => {
    if (initialValues) {
        setName(initialValues.name || "");
        setDescription(getText(initialValues.description));
        if (initialValues.date_range) {
             setStartYear(initialValues.date_range.start?.split("-")[0] || "");
             setEndYear(initialValues.date_range.end?.split("-")[0] || "");
        }
    }
  }, [initialValues]);

  const handleSubmit = async (e: React.FormEvent) => {
    e.preventDefault();
    setLoading(true);
    setError(null);

    try {
      if (!name) throw new Error("Designation/Name is required.");
      if (!startYear || !endYear) throw new Error("Event requires temporal bounds.");

      const payload = {
        name,
        start_date: startYear,
        end_date: endYear,
        description: description || undefined,
        relations: extraRelations,
      };

      if (editId) {
          await updateEvent(editId, payload);
      } else {
          await createEvent(payload);
      }

      setLoading(false);
      onSuccess();
    } catch (err: any) {
      setError(err.message || String(err));
      setLoading(false);
    }
  };

  const currentColor = "var(--disco-accent-purple)";

  return (
    <FormLayout 
      onSubmit={handleSubmit} 
      onCancel={onCancel} 
      loading={loading} 
      error={error} 
      color={currentColor}
      submitLabel={editId ? "Update Event" : "Create Event"}
    >
      <FormInput 
        label="1. Designation" 
        value={name} 
        onChange={(e) => setName(e.target.value)} 
        placeholder="ENTER NAME..." 
        color={currentColor}
      />

      <FormTextArea 
        label="Description / Manifest" 
        value={description} 
        onChange={(e) => setDescription(e.target.value)} 
        placeholder="Details..."
      />

      <TemporalCoordinates 
        startYear={startYear} 
        endYear={endYear} 
        onStartChange={setStartYear} 
        onEndChange={setEndYear} 
        color={currentColor}
      />
    </FormLayout>
  );
}
