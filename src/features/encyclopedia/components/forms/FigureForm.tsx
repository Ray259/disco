import React, { useState, useEffect } from "react";
import { createFigure, updateFigure, createGeo, SearchResult, ContentSegment } from "../../api";
import { FormLayout, FormInput, TemporalCoordinates } from "./SharedFormComponents";
import { RichContentEditor } from "../RichContentEditor";
import { extractSegments } from "../RichContentEditorTypes";
import { RelationSearch } from "../RelationManager/RelationSearch";
import { useFormState } from "../../../../hooks/useFormState";
import {
  ZeitgeistField, ZeitgeistState, CoreIdeologyField, CoreIdeologyState,
  LineageField, LineageState, LegacyField, LegacyState,
  TerminologyField, TerminologyState, ContributionsField, ContributionsState,
  InstitutionalField, InstitutionalState,
} from "./SpecialFields";

function extractYear(dateStr: string | undefined | null): string {
  if (!dateStr) return "";
  const m = dateStr.match(/^(-?\d+)/);
  return m ? m[1] : "";
}

interface FigureFormState {
  name: string;
  role: ContentSegment[];
  location: string;
  quote: ContentSegment[];
  startYear: string;
  endYear: string;
  zeitgeist: ZeitgeistState;
  coreIdeology: CoreIdeologyState;
  lineage: LineageState;
  legacy: LegacyState;
  terminology: TerminologyState;
  contributions: ContributionsState;
  institutional: InstitutionalState;
}

const INITIAL: FigureFormState = {
  name: "", role: [], location: "", quote: [], startYear: "", endYear: "",
  zeitgeist: { era: "", catalyst: "", opposition: "" },
  coreIdeology: { axiom: "", argumentFlow: "" },
  lineage: { predecessors: [], rivals: [], successors: [] },
  legacy: { shortTermSuccess: "", modernRelevance: "", criticalFlaw: "", personalSynthesis: "" },
  terminology: { entries: [] },
  contributions: { entries: [] },
  institutional: { fundingModel: "", institutionalProduct: "", successionPlan: "" },
};

function GeoPickerField({ value, onPick, onClear }: { value: string; onPick: (name: string) => void; onClear: () => void }) {
  const [creating, setCreating] = useState(false);
  const [error, setError] = useState<string | null>(null);

  if (value) {
    return (
      <div>
        <label className="block text-[10px] font-mono uppercase tracking-[0.2em] mb-1 text-[var(--disco-accent-purple)]">3. Geopolitical Origin</label>
        <div className="flex items-center gap-2 border-b border-[var(--c-border)] py-2">
          <span className="text-sm font-body text-[var(--disco-text-primary)] flex-1">{value}</span>
          <button type="button" onClick={onClear} className="text-[9px] font-mono text-[var(--c-ghost)] hover:text-white uppercase tracking-wider">change</button>
        </div>
      </div>
    );
  }

  const handleCreate = async (name: string) => {
    setCreating(true); setError(null);
    try { await createGeo({ name }); onPick(name); }
    catch (err: any) { setError(err?.message || String(err)); }
    finally { setCreating(false); }
  };

  return (
    <div>
      <RelationSearch label="3. Geopolitical Origin" placeholder="Search the ruinous past..." entityType="Geo"
        onSelect={(r: SearchResult) => onPick(r.name)} allowCreate onCreateLabel={creating ? "Analyzing facts..." : undefined} onCreate={handleCreate} />
      {error && <div className="text-[10px] font-mono text-red-400 mt-1">[ERROR] {error}</div>}
    </div>
  );
}

interface Props { onSuccess: () => void; onCancel: () => void; initialValues?: any; editName?: string; }

export function FigureForm({ onSuccess, onCancel, initialValues, editName }: Props) {
  const [form, setField, resetForm] = useFormState<FigureFormState>(INITIAL);
  const [loading, setLoading] = useState(false);
  const [error, setError] = useState<string | null>(null);

  useEffect(() => {
    if (initialValues) {
      resetForm({
        ...INITIAL,
        name: initialValues.name || "",
        role: extractSegments(initialValues.primary_role),
        location: initialValues.primary_location
          ? (typeof initialValues.primary_location === "string" ? initialValues.primary_location
              : initialValues.primary_location.segments?.map((s: any) => s.Text || "").join("") || "")
          : "",
        quote: extractSegments(initialValues.defining_quote),
        startYear: extractYear(initialValues.life?.start),
        endYear: extractYear(initialValues.life?.end),
      });
    }
  }, [initialValues]);

  const handleSubmit = async (e: React.FormEvent) => {
    e.preventDefault();
    setLoading(true); setError(null);
    try {
      if (!form.name) throw new Error("Name is required.");
      if (form.role.length === 0 || !form.location) throw new Error("Role and Origin required.");
      const payload = {
        name: form.name,
        role: { segments: form.role },
        location: { segments: form.location ? [{ Text: form.location }] : [] },
        start_year: form.startYear, end_year: form.endYear,
        quote: form.quote.length > 0 ? { segments: form.quote } : undefined,
      };
      if (editName) { await updateFigure(editName, payload); }
      else { await createFigure(payload); }
      onSuccess();
    } catch (err: any) { setError(err.message || String(err)); }
    finally { setLoading(false); }
  };

  const c = "var(--disco-accent-orange)";
  return (
    <FormLayout onSubmit={handleSubmit} onCancel={onCancel} loading={loading} error={error} color={c} submitLabel={editName ? "[ RE-INTERNALIZE ]" : "[ INTERNALIZE ]"}>
      <FormInput className="uppercase" label="1. Subject Designation" value={form.name} onChange={(e) => setField("name", e.target.value)} placeholder="WHO IS THIS MAN?..." color={c} />
      <div className="grid grid-cols-1 md:grid-cols-2 gap-12">
        <RichContentEditor label="2. Primary Archetype" value={form.role} onChange={(segs) => setField("role", segs)} placeholder="Philosopher, Painter, Lunatic..." />
        <GeoPickerField value={form.location} onPick={(name) => setField("location", name)} onClear={() => setField("location", "")} />
      </div>
      <RichContentEditor label="4. Significant Utterance" value={form.quote} onChange={(segs) => setField("quote", segs)} placeholder="&ldquo;...&rdquo;" multiline />
      <TemporalCoordinates startYear={form.startYear} endYear={form.endYear} onStartChange={(v) => setField("startYear", v)} onEndChange={(v) => setField("endYear", v)} />
      <ZeitgeistField value={form.zeitgeist} onChange={(v) => setField("zeitgeist", v)} />
      <CoreIdeologyField value={form.coreIdeology} onChange={(v) => setField("coreIdeology", v)} />
      <LineageField value={form.lineage} onChange={(v) => setField("lineage", v)} />
      <LegacyField value={form.legacy} onChange={(v) => setField("legacy", v)} />
      <TerminologyField value={form.terminology} onChange={(v) => setField("terminology", v)} />
      <ContributionsField value={form.contributions} onChange={(v) => setField("contributions", v)} />
      <InstitutionalField value={form.institutional} onChange={(v) => setField("institutional", v)} />
    </FormLayout>
  );
}
