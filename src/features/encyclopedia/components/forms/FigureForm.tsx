import { useState } from "react";
import { createFigure, RelationDto } from "../../api";
import { FormLayout, FormInput, FormTextArea, TemporalCoordinates } from "./SharedFormComponents";

interface FigureFormProps {
  onSuccess: () => void;
  onCancel: () => void;
  extraRelations: RelationDto[];
}

export function FigureForm({ onSuccess, onCancel, extraRelations }: FigureFormProps) {
  const [loading, setLoading] = useState(false);
  const [error, setError] = useState<string | null>(null);

  const [name, setName] = useState("");
  const [role, setRole] = useState("");
  const [location, setLocation] = useState("");
  const [quote, setQuote] = useState("");
  const [startYear, setStartYear] = useState("");
  const [endYear, setEndYear] = useState("");

  const handleSubmit = async (e: React.FormEvent) => {
    e.preventDefault();
    setLoading(true);
    setError(null);

    try {
      if (!name) throw new Error("Designation/Name is required.");
      if (!role || !location) throw new Error("Role and Origin required.");

      await createFigure({
        name,
        role,
        location,
        start_year: startYear,
        end_year: endYear,
        quote: quote || undefined,
        relations: extraRelations,
      });

      setLoading(false);
      onSuccess();
    } catch (err: any) {
      setError(err.message || String(err));
      setLoading(false);
    }
  };

  const currentColor = "var(--disco-accent-orange)";

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

      <div className="grid grid-cols-1 md:grid-cols-2 gap-12">
         <FormInput label="2. Role" value={role} onChange={(e) => setRole(e.target.value)} />
         <FormInput label="3. Origin" value={location} onChange={(e) => setLocation(e.target.value)} />
      </div>

      <FormTextArea 
        label="4. Defining Utterance" 
        value={quote} 
        onChange={(e) => setQuote(e.target.value)} 
        rows={2} 
        className="font-italic"
        placeholder="&ldquo;...&rdquo;"
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
