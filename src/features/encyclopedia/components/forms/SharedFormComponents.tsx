import React from "react";

// --- Types ---
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

// --- Components ---

export function FormLayout({ 
  children, 
  onSubmit, 
  onCancel, 
  loading, 
  error, 
  submitLabel = "INTERNALIZE", 
  color = "white" 
}: FormLayoutProps) {
  return (
    <form onSubmit={onSubmit} className="space-y-12">
      {error && (
        <div className="mb-8 p-4 bg-[#2a1a1a] border-l-4 border-red-800 text-red-400 font-[var(--font-mono)] text-xs">
          [FAILURE] {error}
        </div>
      )}
      
      {children}

      <div className="flex justify-between items-center pt-8 border-t border-[#222]">
         <button
           type="button"
           onClick={onCancel}
           className="text-xs font-[var(--font-mono)] text-[#666] hover:text-white uppercase tracking-widest hover:underline"
         >
           [ Discard ]
         </button>

         <button
           type="submit"
           disabled={loading}
           className="bg-[#d4d4d8] text-black px-8 py-3 font-[var(--font-header)] text-xl uppercase tracking-widest hover:bg-white transition-all disabled:opacity-50"
           style={{ backgroundColor: color }}
         >
            {loading ? "PROCESSING..." : submitLabel}
         </button>
      </div>
    </form>
  );
}

export function FormInput({ label, color, className = "", ...props }: FormInputProps) {
  return (
    <div className={`group ${className}`}>
      <label 
        className="block text-xs font-[var(--font-mono)] uppercase tracking-widest mb-2 transition-colors" 
        style={{ color: color || "#666" }}
      >
        {label}
      </label>
      <input
        className="w-full bg-transparent border-b border-[#333] py-2 text-lg font-[var(--font-body)] text-[#ccc] focus:border-[var(--disco-accent-teal)] focus:outline-none transition-colors"
        style={color ? { fontSize: "1.875rem", lineHeight: "2.25rem", fontFamily: "var(--font-header)", color: "white", borderBottomWidth: "2px" } : {}}
        {...props}
      />
    </div>
  );
}

export function FormTextArea({ label, className = "", ...props }: FormTextAreaProps) {
  return (
    <div className={`group ${className}`}>
      <label className="block text-xs font-[var(--font-mono)] text-[#666] uppercase tracking-widest mb-2">
        {label}
      </label>
      <textarea
        rows={4}
        className="w-full bg-[#181818] border border-[#333] p-4 text-lg font-[var(--font-body)] text-white focus:outline-none placeholder-[#444] focus:border-[var(--disco-accent-teal)]"
        {...props}
      />
    </div>
  );
}

export function TemporalCoordinates({ startYear, endYear, onStartChange, onEndChange, color = "var(--disco-accent-yellow)" }: TemporalCoordinatesProps) {
  const inputStyle = `w-full bg-[#0a0a0a] border border-[#333] p-3 text-center text-xl font-[var(--font-mono)] outline-none focus:border-[var(--disco-accent-orange)]`;
  
  return (
    <div className="p-6 bg-[#151515] border border-[#222] relative overflow-hidden">
       <div className="absolute top-0 right-0 w-4 h-4 border-t border-r border-[#444]" />
       <h3 className="text-xs font-[var(--font-mono)] text-[#666] uppercase mb-6 text-center tracking-widest">
         Temporal Coordinates
       </h3>
       <div className="flex items-center gap-6">
         <div className="flex-1">
           <label className="block text-[10px] text-[#444] mb-1 font-[var(--font-mono)] text-center">START</label>
           <input
             type="text"
             value={startYear}
             onChange={(e) => onStartChange(e.target.value)}
             className={inputStyle}
             style={{ color }}
             placeholder="YYYY-MM-DD"
           />
         </div>
         <span className="text-[#333] text-xl">&mdash;</span>
         <div className="flex-1">
           <label className="block text-[10px] text-[#444] mb-1 font-[var(--font-mono)] text-center">END</label>
           <input
             type="text"
             value={endYear} 
             onChange={(e) => onEndChange(e.target.value)}
             className={inputStyle}
             style={{ color }}
             placeholder="YYYY-MM-DD"
           />
         </div>
       </div>
    </div>
  );
}
