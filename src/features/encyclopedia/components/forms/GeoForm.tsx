import { useState, useEffect } from "react";
import { createGeo, updateGeo, ContentSegment } from "../../api";
import { FormLayout, FormInput, GeoPickerField } from "./SharedFormComponents";
import { RichContentEditor } from "../RichContentEditor";
import { extractSegments, segmentsToPlainText } from "../RichContentEditorTypes";

interface Props { onSuccess: () => void; onCancel: () => void; initialValues?: any; editName?: string; }

export function GeoForm({ onSuccess, onCancel, initialValues, editName }: Props) {
  const [loading, setLoading] = useState(false);
  const [error, setError] = useState<string | null>(null);
  const [name, setName] = useState("");
  const [region, setRegion] = useState("");
  const [description, setDescription] = useState<ContentSegment[]>([]);

  useEffect(() => {
    if (initialValues) { 
      setName(initialValues.name || ""); 
      setRegion(initialValues.region ? segmentsToPlainText(extractSegments(initialValues.region)) : ""); 
      setDescription(extractSegments(initialValues.description)); 
    }
  }, [initialValues]);

  const handleSubmit = async (e: React.FormEvent) => {
    e.preventDefault(); setLoading(true); setError(null);
    try {
      if (!name) throw new Error("Name is required.");
      const payload = { 
        name, 
        region: region ? { segments: [{ EntityRef: { entity_type: "Geo", display_text: region } }] } : undefined, 
        description: description.length > 0 ? { segments: description } : undefined 
      };
      if (editName) { await updateGeo(editName, payload); } else { await createGeo(payload); }
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
      theme="Geo" 
      submitLabel={editName ? "Update Location" : "Create Location"}
    >
      <FormInput 
        label="1. Designation" 
        value={name} 
        onChange={(e) => setName(e.target.value)} 
        placeholder="ENTER NAME..." 
        themed 
      />
      <GeoPickerField 
        label="2. Region / Macro-Area" 
        value={region} 
        onPick={setRegion} 
        onClear={() => setRegion("")} 
        themeColor="var(--disco-accent-teal)"
      />
      <RichContentEditor label="Description / Manifest" value={description} onChange={setDescription} placeholder="Details..." multiline />
    </FormLayout>
  );
}
