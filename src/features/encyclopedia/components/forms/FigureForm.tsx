import React, { useState, useEffect } from "react";
import { createFigure, updateFigure, createGeo, SearchResult, RelationDto } from "../../api";
import { FormLayout, FormInput, FormTextArea, TemporalCoordinates } from "./SharedFormComponents";
import { RelationSearch } from "../RelationManager/RelationSearch";
import { useFormState } from "../../../../hooks/useFormState";
import {
  ZeitgeistField, ZeitgeistState,
  CoreIdeologyField, CoreIdeologyState,
  LineageField, LineageState,
  LegacyField, LegacyState,
  TerminologyField, TerminologyState,
  ContributionsField, ContributionsState,
  InstitutionalField, InstitutionalState,
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
  // special fields
  zeitgeist: ZeitgeistState;
  coreIdeology: CoreIdeologyState;
  lineage: LineageState;
  legacy: LegacyState;
  terminology: TerminologyState;
  contributions: ContributionsState;
  institutional: InstitutionalState;
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
  terminology: { entries: [] },
  contributions: { entries: [] },
  institutional: { fundingModel: "", institutionalProduct: "", successionPlan: "" },
};

/* GeoPickerField — owns the create-Geo flow; RelationSearch is purely search/select */
function GeoPickerField({ value, onPick, onClear }: {
  value: string;
  onPick: (name: string) => void;
  onClear: () => void;
}) {
  const [creating, setCreating] = useState(false);
  const [query, setQuery] = useState("");

  const handleCreate = async () => {
    if (!query.trim()) return;
    setCreating(true);
    try {
      const newId = await createGeo({ name: query.trim() });
      onPick(query.trim(), newId);
      setQuery("");
    } finally {
      setCreating(false);
    }
  };

  if (value) {
    return (
      <div>
        <label className="block text-[10px] font-mono uppercase tracking-[0.2em] mb-1 text-[var(--disco-accent-purple)]">3. Origin</label>
        <div className="flex items-center gap-2 border-b border-[var(--c-border)] py-2">
          <span className="text-sm font-body text-[var(--disco-text-primary)] flex-1">{value}</span>
          <button type="button" onClick={onClear}
            className="text-[9px] font-mono text-[var(--c-ghost)] hover:text-white uppercase tracking-wider">change</button>
        </div>
      </div>
    );
  }

  return (
    <div className="space-y-2">
      <RelationSearch
        label="3. Origin"
        placeholder="Search existing locations..."
        entityType="Geo"
        onSelect={(r: SearchResult) => onPick(r.name, r.id)}
      />
      <div className="flex gap-2 items-center">
        <input
          type="text"
          value={query}
          onChange={e => setQuery(e.target.value)}
          onKeyDown={e => { if (e.key === "Enter") { e.preventDefault(); handleCreate(); } }}
          placeholder="Or type a new location name..."
          className="flex-1 bg-transparent border-b border-dashed border-[var(--c-border)] py-1 text-xs font-mono text-[var(--disco-text-secondary)] focus:outline-none placeholder-[var(--c-ghost)]"
        />
        {query.trim() && (
          <button type="button" onClick={handleCreate}
            className="text-[9px] font-mono text-[var(--disco-accent-yellow)] hover:text-white uppercase tracking-wider whitespace-nowrap">
            {creating ? "Creating..." : `+ Create`}
          </button>
        )}
      </div>
    </div>
  );
}

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
        className="uppercase"        
        label="1. Designation"
        value={form.name}
        onChange={(e) => setField("name", e.target.value)}
        placeholder="ENTER NAME..."
        color={currentColor}
      />

      <div className="grid grid-cols-1 md:grid-cols-2 gap-12">
        <FormInput label="2. Role" value={form.role} onChange={(e) => setField("role", e.target.value)} />

        {/* Origin - search or create */}
        <GeoPickerField
          value={form.location}
          onPick={(name) => setField("location", name)}
          onClear={() => setField("location", "")}
        />
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

      {/* ── SPECIAL FIELDS ── */}

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

      <TerminologyField
        value={form.terminology}
        onChange={(v) => setField("terminology", v)}
      />

      <ContributionsField
        value={form.contributions}
        onChange={(v) => setField("contributions", v)}
      />

      <InstitutionalField
        value={form.institutional}
        onChange={(v) => setField("institutional", v)}
      />
    </FormLayout>
  );
}
