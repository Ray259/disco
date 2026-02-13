import { useState } from "react";
import { createEvent, RelationDto } from "../../api";
import { FormLayout, FormInput, FormTextArea, TemporalCoordinates } from "./SharedFormComponents";

interface EventFormProps {
  onSuccess: () => void;
  onCancel: () => void;
  extraRelations: RelationDto[];
}

export function EventForm({ onSuccess, onCancel, extraRelations }: EventFormProps) {
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
      if (!startYear || !endYear) throw new Error("Event requires temporal bounds.");

      await createEvent({
        name,
        start_date: startYear,
        end_date: endYear,
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

  const currentColor = "var(--disco-accent-purple)";

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
