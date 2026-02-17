import { useState, useEffect } from "react";
import { createSchoolOfThought, updateSchoolOfThought, RelationDto } from "../../api";
import { FormLayout, FormInput, FormTextArea } from "./SharedFormComponents";

interface SchoolFormProps {
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

export function SchoolForm({ onSuccess, onCancel, extraRelations, initialValues, editId }: SchoolFormProps) {
  const [loading, setLoading] = useState(false);
  const [error, setError] = useState<string | null>(null);

  const [name, setName] = useState("");
  const [description, setDescription] = useState("");

  useEffect(() => {
    if (initialValues) {
        setName(initialValues.name || "");
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
        description: description || undefined,
        relations: extraRelations,
      };

      if (editId) {
          await updateSchoolOfThought(editId, payload);
      } else {
          await createSchoolOfThought(payload);
      }

      setLoading(false);
      onSuccess();
    } catch (err: any) {
      setError(err.message || String(err));
      setLoading(false);
    }
  };

  const currentColor = "#ef4444"; // Red-500

  return (
    <FormLayout 
      onSubmit={handleSubmit} 
      onCancel={onCancel} 
      loading={loading} 
      error={error} 
      color={currentColor}
      submitLabel={editId ? "Update School" : "Create School"}
    >
      <FormInput 
        label="1. Designation" 
        value={name} 
        onChange={(e) => setName(e.target.value)} 
        placeholder="ENTER IDEOLOGY..." 
        color={currentColor}
      />

      <FormTextArea 
        label="Description / Manifest" 
        value={description} 
        onChange={(e) => setDescription(e.target.value)} 
        rows={6}
        placeholder="Attributes..."
      />
    </FormLayout>
  );
}
