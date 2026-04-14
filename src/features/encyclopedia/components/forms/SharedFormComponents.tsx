import React, { useState } from "react";
import { DatePicker } from "../../../../components/DatePicker";
import { SearchResult, createGeo } from "../../api";
import { RelationSearch } from "../RelationManager/RelationSearch";

interface FormLayoutProps {
  children: React.ReactNode;
  onSubmit: (e: React.FormEvent) => void;
  onCancel: () => void;
  loading: boolean;
  error: string | null;
  submitLabel?: string;
  theme?: string;
}

interface FormInputProps extends React.InputHTMLAttributes<HTMLInputElement> {
  label: string;
  themed?: boolean;
  className?: string;
}

interface FormTextAreaProps extends React.TextareaHTMLAttributes<HTMLTextAreaElement> {
  label: string;
  className?: string;
}

interface TemporalCoordinatesProps {
  startYear: string;
  endYear: string;
  onStartChange: (val: string) => void;
  onEndChange: (val: string) => void;
}

export function FormLayout({ 
  children, onSubmit, onCancel, loading, error, 
  submitLabel = "INTERNALIZE", theme = "Figure" 
}: FormLayoutProps) {
  return (
    <form onSubmit={onSubmit} className="space-y-12" data-form-theme={theme}>
      {error && (
        <div className="mb-8 p-4 bg-[#2a1a1a] border-l-4 border-red-800 text-red-400 font-mono text-xs">
          [FAILURE] {error}
        </div>
      )}
      
      {children}

      <div className="sticky bottom-0 z-10 bg-[var(--c-dark)]/95 backdrop-blur-sm pt-6 pb-2 -mx-10 px-10 border-t border-[var(--c-deep)]">
        <div className="flex justify-between items-center">
         <button
           type="button"
           onClick={onCancel}
           className="text-xs font-mono text-[var(--c-muted)] hover:text-white uppercase tracking-widest hover:underline"
         >
           [ Discard ]
         </button>

         <button
           type="submit"
           disabled={loading}
           className="submit-button--themed text-black px-8 py-3 font-header text-xl uppercase tracking-widest hover:bg-white transition-all disabled:opacity-50"
         >
            {loading ? "PROCESSING..." : submitLabel}
         </button>
        </div>
      </div>
    </form>
  );
}

export function FormInput({ label, themed, className = "", ...props }: FormInputProps) {
  return (
    <div className={`group ${className}`}>
      <label className="label-mono transition-colors group-focus-within:text-[var(--theme-color)]">
        {label}
      </label>
      <input
        className={`w-full bg-transparent border-b border-[var(--c-border)] py-2 text-lg font-body text-[#ccc] focus:border-[var(--theme-color)] focus:outline-none transition-colors ${themed ? "form-header-input" : ""}`}
        {...props}
      />
    </div>
  );
}

export function FormTextArea({ label, className = "", ...props }: FormTextAreaProps) {
  return (
    <div className={`group ${className}`}>
      <label className="label-mono">{label}</label>
      <textarea
        rows={4}
        className="w-full bg-[#181818] border border-[var(--c-border)] p-4 text-lg font-body text-white focus:outline-none placeholder-[var(--c-ghost)] focus:border-[var(--disco-accent-teal)]"
        {...props}
      />
    </div>
  );
}

export function TemporalCoordinates({ startYear, endYear, onStartChange, onEndChange }: TemporalCoordinatesProps) {
  return (
    <div className="p-6 bg-[var(--c-surface)] border border-[var(--c-deep)]">
       <h3 className="mb-6 text-center">Temporal Coordinates</h3>
       <div className="flex items-center gap-6">
         <div className="flex-1">
           <label className="block text-[10px] text-[var(--c-ghost)] mb-1 font-mono text-center">START</label>
           <DatePicker value={startYear} onChange={onStartChange} />
         </div>
         <span className="text-[var(--c-border)] text-xl">&mdash;</span>
         <div className="flex-1">
           <label className="block text-[10px] text-[var(--c-ghost)] mb-1 font-mono text-center">END</label>
           <DatePicker value={endYear} onChange={onEndChange} />
         </div>
       </div>
    </div>
  );
}

export function GeoPickerField({ 
  label = "Geopolitical Origin", 
  value, 
  onPick, 
  onClear,
  themeColor = "var(--disco-accent-purple)"
}: { 
  label?: string;
  value: string; 
  onPick: (name: string) => void; 
  onClear: () => void;
  themeColor?: string;
}) {
  const [creating, setCreating] = useState(false);
  const [error, setError] = useState<string | null>(null);

  if (value) {
    return (
      <div className="group">
        <label className="label-mono transition-colors group-focus-within:text-[var(--theme-color)]" style={{ color: themeColor }}>
          {label}
        </label>
        <div className="flex items-center gap-2 border-b border-[var(--c-border)] py-2">
          <span className="text-sm font-body text-[var(--disco-text-primary)] flex-1">{value}</span>
          <button 
            type="button" 
            onClick={onClear} 
            className="text-[9px] font-mono text-[var(--c-ghost)] hover:text-white uppercase tracking-wider"
          >
            change
          </button>
        </div>
      </div>
    );
  }

  const handleCreate = async (name: string) => {
    setCreating(true); 
    setError(null);
    try { 
      await createGeo({ name }); 
      onPick(name); 
    } catch (err: any) { 
      setError(err?.message || String(err)); 
    } finally { 
      setCreating(false); 
    }
  };

  return (
    <div>
      <RelationSearch 
        label={label} 
        placeholder="Search the ruinous past..." 
        entityType="Geo"
        onSelect={(r: SearchResult) => onPick(r.name)} 
        allowCreate 
        onCreateLabel={creating ? "Analyzing facts..." : undefined} 
        onCreate={handleCreate} 
      />
      {error && <div className="text-[10px] font-mono text-red-400 mt-1">[ERROR] {error}</div>}
    </div>
  );
}
