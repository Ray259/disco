import { useState } from "react";
import { createInstitution, RelationDto } from "../../api";
import { FormLayout, FormInput, FormTextArea, TemporalCoordinates } from "./SharedFormComponents";

interface InstitutionFormProps {
  onSuccess: () => void;
  onCancel: () => void;
  extraRelations: RelationDto[];
}

export function InstitutionForm({ onSuccess, onCancel, extraRelations }: InstitutionFormProps) {
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

      await createInstitution({
        name,
        founded_start: startYear || undefined,
        founded_end: endYear || undefined,
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

  const currentColor = "var(--disco-accent-yellow)";

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
      />
    </FormLayout>
  );
}
