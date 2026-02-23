import { useState, useEffect } from "react";
import { createFigure, updateFigure, RelationDto } from "../../api";
import { FormLayout, FormInput, FormTextArea, TemporalCoordinates } from "./SharedFormComponents";
import { useFormState } from "../../../../hooks/useFormState";
import {
  ZeitgeistField, ZeitgeistState,
  CoreIdeologyField, CoreIdeologyState,
  LineageField, LineageState,
  LegacyField, LegacyState,
} from "./SpecialFields";

interface FigureFormProps {
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

interface FigureFormState {
  name: string;
  role: string;
  location: string;
  quote: string;
  startYear: string;
  endYear: string;
  // Special fields (UI-only for now)
  zeitgeist: ZeitgeistState;
  coreIdeology: CoreIdeologyState;
  lineage: LineageState;
  legacy: LegacyState;
}

const INITIAL_STATE: FigureFormState = {
  name: "",
  role: "",
  location: "",
  quote: "",
  startYear: "",
  endYear: "",
  zeitgeist: { era: "", catalyst: "", opposition: "" },
  coreIdeology: { axiom: "", argumentFlow: "" },
  lineage: { predecessors: [], rivals: [], successors: [] },
  legacy: { shortTermSuccess: "", modernRelevance: "", criticalFlaw: "", personalSynthesis: "" },
};

export function FigureForm({ onSuccess, onCancel, extraRelations, initialValues, editId }: FigureFormProps) {
  const [form, setField, resetForm] = useFormState<FigureFormState>(INITIAL_STATE);
  const [loading, setLoading] = useState(false);
  const [error, setError] = useState<string | null>(null);

  useEffect(() => {
    if (initialValues) {
      resetForm({
        ...INITIAL_STATE,
        name: initialValues.name || "",
        role: getText(initialValues.primary_role),
        location: getText(initialValues.primary_location),
        quote: getText(initialValues.defining_quote),
        startYear: initialValues.life?.start?.split("-")[0] || "",
        endYear: initialValues.life?.end?.split("-")[0] || "",
        // Special fields would be hydrated here once backend sends them
      });
    }
  }, [initialValues]);

  const handleSubmit = async (e: React.FormEvent) => {
    e.preventDefault();
    setLoading(true);
    setError(null);

    try {
      if (!form.name) throw new Error("Designation/Name is required.");
      if (!form.role || !form.location) throw new Error("Role and Origin required.");

      const payload = {
        name: form.name,
        role: form.role,
        location: form.location,
        start_year: form.startYear,
        end_year: form.endYear,
        quote: form.quote || undefined,
        relations: extraRelations,
      };

      if (editId) {
        await updateFigure(editId, payload);
      } else {
        await createFigure(payload);
      }

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
      submitLabel={editId ? "Update Figure" : "INTERNALIZE"}
    >
      {/* ── BASIC FIELDS ── */}
      <FormInput
        label="1. Designation"
        value={form.name}
        onChange={(e) => setField("name", e.target.value)}
        placeholder="ENTER NAME..."
        color={currentColor}
      />

      <div className="grid grid-cols-1 md:grid-cols-2 gap-12">
        <FormInput label="2. Role" value={form.role} onChange={(e) => setField("role", e.target.value)} />
        <FormInput label="3. Origin" value={form.location} onChange={(e) => setField("location", e.target.value)} />
      </div>

      <FormTextArea
        label="4. Defining Utterance"
        value={form.quote}
        onChange={(e) => setField("quote", e.target.value)}
        rows={2}
        className="font-italic"
        placeholder="&ldquo;...&rdquo;"
      />

      <TemporalCoordinates
        startYear={form.startYear}
        endYear={form.endYear}
        onStartChange={(v) => setField("startYear", v)}
        onEndChange={(v) => setField("endYear", v)}
      />

      {/* ── SPECIAL FIELDS (UI-only, not wired to backend yet) ── */}

      <ZeitgeistField
        value={form.zeitgeist}
        onChange={(v) => setField("zeitgeist", v)}
      />

      <CoreIdeologyField
        value={form.coreIdeology}
        onChange={(v) => setField("coreIdeology", v)}
      />

      <LineageField
        value={form.lineage}
        onChange={(v) => setField("lineage", v)}
      />

      <LegacyField
        value={form.legacy}
        onChange={(v) => setField("legacy", v)}
      />
    </FormLayout>
  );
}
