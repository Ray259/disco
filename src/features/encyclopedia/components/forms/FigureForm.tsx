import React, { useState, useEffect } from "react";
import { createFigure, updateFigure, createGeo, SearchResult, ContentSegment } from "../../api";
import { FormLayout, FormInput, TemporalCoordinates, GeoPickerField } from "./SharedFormComponents";
import { RichContentEditor } from "../RichContentEditor";
import { extractSegments, segmentsToPlainText } from "../RichContentEditorTypes";
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
              : segmentsToPlainText(extractSegments(initialValues.primary_location)))
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
        location: { segments: form.location ? [{ EntityRef: { entity_type: "Geo", display_text: form.location } }] : [] },
        start_year: form.startYear, end_year: form.endYear,
        quote: form.quote.length > 0 ? { segments: form.quote } : undefined,
      };
      if (editName) { await updateFigure(editName, payload); }
      else { await createFigure(payload); }
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
      theme="Figure" 
      submitLabel={editName ? "[ RE-INTERNALIZE ]" : "[ INTERNALIZE ]"}
    >
      <FormInput 
        className="uppercase" 
        label="1. Subject Designation" 
        value={form.name} 
        onChange={(e) => setField("name", e.target.value)} 
        placeholder="WHO IS THIS MAN?..." 
        themed
      />
      <div className="grid grid-cols-1 md:grid-cols-2 gap-12">
        <RichContentEditor label="2. Primary Archetype" value={form.role} onChange={(segs) => setField("role", segs)} placeholder="Philosopher, Painter, Lunatic..." />
        <GeoPickerField label="3. Geopolitical Origin" value={form.location} onPick={(name) => setField("location", name)} onClear={() => setField("location", "")} />
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
