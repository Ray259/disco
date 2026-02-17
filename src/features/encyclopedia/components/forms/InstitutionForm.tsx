import { useState, useEffect } from "react";
import { createInstitution, updateInstitution, RelationDto } from "../../api";
import { FormLayout, FormInput, FormTextArea, TemporalCoordinates } from "./SharedFormComponents";

interface InstitutionFormProps {
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

export function InstitutionForm({ onSuccess, onCancel, extraRelations, initialValues, editId }: InstitutionFormProps) {
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
        if (initialValues.founded) {
             setStartYear(initialValues.founded.start?.split("-")[0] || "");
             setEndYear(initialValues.founded.end?.split("-")[0] || "");
        }
    }
  }, [initialValues]);

  const handleSubmit = async (e: React.FormEvent) => {
    e.preventDefault();
    setLoading(true);
    setError(null);

    try {
      if (!name) throw new Error("Designation/Name is required.");

      const payload = {
        name,
        founded_start: startYear || undefined,
        founded_end: endYear || undefined,
        description: description || undefined,
        relations: extraRelations,
      };

      if (editId) {
          await updateInstitution(editId, payload);
      } else {
          await createInstitution(payload);
      }

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
      submitLabel={editId ? "Update Institution" : "Create Institution"}
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
