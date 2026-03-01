import React from "react";
import { DatePicker } from "../../../../components/DatePicker";

interface FormLayoutProps {
  children: React.ReactNode;
  onSubmit: (e: React.FormEvent) => void;
  onCancel: () => void;
  loading: boolean;
  error: string | null;
  submitLabel?: string;
  color?: string;
}

interface FormInputProps extends React.InputHTMLAttributes<HTMLInputElement> {
  label: string;
  color?: string;
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
  color?: string;
}

export function FormLayout({ 
  children, onSubmit, onCancel, loading, error, 
  submitLabel = "INTERNALIZE", color = "white" 
}: FormLayoutProps) {
  return (
    <form onSubmit={onSubmit} className="space-y-12">
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
           className="bg-[var(--disco-text-primary)] text-black px-8 py-3 font-header text-xl uppercase tracking-widest hover:bg-white transition-all disabled:opacity-50"
           style={{ backgroundColor: color }}
         >
            {loading ? "PROCESSING..." : submitLabel}
         </button>
        </div>
      </div>
    </form>
  );
}

export function FormInput({ label, color, className = "", ...props }: FormInputProps) {
  return (
    <div className={`group ${className}`}>
      <label 
        className="label-mono transition-colors" 
        style={{ color: color || undefined }}
      >
        {label}
      </label>
      <input
        className="w-full bg-transparent border-b border-[var(--c-border)] py-2 text-lg font-body text-[#ccc] focus:border-[var(--disco-accent-teal)] focus:outline-none transition-colors"
        style={color ? { fontSize: "1.875rem", lineHeight: "2.25rem", fontFamily: "var(--font-header)", color: "white", borderBottomWidth: "2px" } : {}}
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

export function TemporalCoordinates({ startYear, endYear, onStartChange, onEndChange, color = "var(--disco-accent-yellow)" }: TemporalCoordinatesProps) {
  return (
    <div className="p-6 bg-[var(--c-surface)] border border-[var(--c-deep)]">
       <h3 className="mb-6 text-center">Temporal Coordinates</h3>
       <div className="flex items-center gap-6">
         <div className="flex-1">
           <label className="block text-[10px] text-[var(--c-ghost)] mb-1 font-mono text-center">START</label>
           <DatePicker value={startYear} onChange={onStartChange} color={color} />
         </div>
         <span className="text-[var(--c-border)] text-xl">&mdash;</span>
         <div className="flex-1">
           <label className="block text-[10px] text-[var(--c-ghost)] mb-1 font-mono text-center">END</label>
           <DatePicker value={endYear} onChange={onEndChange} color={color} />
         </div>
       </div>
    </div>
  );
}
