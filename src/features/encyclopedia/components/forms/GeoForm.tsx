import { useState, useEffect } from "react";
import { createGeo, updateGeo, RelationDto } from "../../api";
import { FormLayout, FormInput, FormTextArea } from "./SharedFormComponents";

interface GeoFormProps {
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

export function GeoForm({ onSuccess, onCancel, extraRelations, initialValues, editId }: GeoFormProps) {
  const [loading, setLoading] = useState(false);
  const [error, setError] = useState<string | null>(null);

  const [name, setName] = useState("");
  const [region, setRegion] = useState("");
  const [description, setDescription] = useState("");

  useEffect(() => {
    if (initialValues) {
        setName(initialValues.name || "");
        setRegion(getText(initialValues.region));
        setDescription(getText(initialValues.description));
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
        region: region || undefined,
        description: description || undefined,
        relations: extraRelations,
      };

      if (editId) {
          await updateGeo(editId, payload);
      } else {
          await createGeo(payload);
      }

      setLoading(false);
      onSuccess();
    } catch (err: any) {
      setError(err.message || String(err));
      setLoading(false);
    }
  };

  const currentColor = "var(--disco-accent-teal)";

  return (
    <FormLayout 
      onSubmit={handleSubmit} 
      onCancel={onCancel} 
      loading={loading} 
      error={error} 
      color={currentColor}
      submitLabel={editId ? "Update Location" : "Create Location"}
    >
      <FormInput 
        label="1. Designation" 
        value={name} 
        onChange={(e) => setName(e.target.value)} 
        placeholder="ENTER NAME..." 
        color={currentColor}
      />

      <FormInput 
        label="2. Region / Macro-Area" 
        value={region} 
        onChange={(e) => setRegion(e.target.value)} 
        placeholder="e.g. Isola of Graad"
      />

      <FormTextArea 
        label="Description / Manifest" 
        value={description} 
        onChange={(e) => setDescription(e.target.value)} 
        placeholder="Details..."
      />
    </FormLayout>
  );
}
