import { useState, useRef, useEffect } from "react";

interface DatePickerProps {
  value: string;
  onChange: (value: string) => void;
  placeholder?: string;
  color?: string;
}

const MONTHS = ["JAN", "FEB", "MAR", "APR", "MAY", "JUN", "JUL", "AUG", "SEP", "OCT", "NOV", "DEC"];

function daysInMonth(year: number, month: number): number {
  return new Date(year, month + 1, 0).getDate();
}

function parseDate(value: string): { year: number; month: number; day: number } | null {
  const m = value.match(/^(\d{4})-(\d{2})-(\d{2})$/);
  if (!m) return null;
  return { year: parseInt(m[1]), month: parseInt(m[2]) - 1, day: parseInt(m[3]) };
}

function formatDate(year: number, month: number, day: number): string {
  return `${year}-${String(month + 1).padStart(2, "0")}-${String(day).padStart(2, "0")}`;
}

export function DatePicker({ value, onChange, placeholder = "YYYY-MM-DD", color = "var(--disco-accent-yellow)" }: DatePickerProps) {
  const [open, setOpen] = useState(false);
  const ref = useRef<HTMLDivElement>(null);

  const parsed = parseDate(value);
  const today = new Date();
  const [viewYear, setViewYear] = useState(parsed?.year ?? today.getFullYear());
  const [viewMonth, setViewMonth] = useState(parsed?.month ?? today.getMonth());

  useEffect(() => {
    const p = parseDate(value);
    if (p) { setViewYear(p.year); setViewMonth(p.month); }
  }, [value]);

  useEffect(() => {
    const handler = (e: MouseEvent) => {
      if (ref.current && !ref.current.contains(e.target as Node)) setOpen(false);
    };
    document.addEventListener("mousedown", handler);
    return () => document.removeEventListener("mousedown", handler);
  }, []);

  const days = daysInMonth(viewYear, viewMonth);
  const firstDay = new Date(viewYear, viewMonth, 1).getDay();

  const prevMonth = () => {
    if (viewMonth === 0) { setViewMonth(11); setViewYear(viewYear - 1); }
    else setViewMonth(viewMonth - 1);
  };

  const nextMonth = () => {
    if (viewMonth === 11) { setViewMonth(0); setViewYear(viewYear + 1); }
    else setViewMonth(viewMonth + 1);
  };

  const selectDay = (day: number) => {
    onChange(formatDate(viewYear, viewMonth, day));
    setOpen(false);
  };

  const isSelected = (day: number) =>
    parsed?.year === viewYear && parsed?.month === viewMonth && parsed?.day === day;

  return (
    <div className="relative" ref={ref}>
      <input
        type="text"
        value={value}
        onChange={(e) => {
          onChange(e.target.value);
          setOpen(true);
        }}
        onFocus={() => setOpen(true)}
        onClick={() => setOpen(true)}
        placeholder={placeholder}
        className="w-full bg-[var(--c-dark)] border border-[var(--c-border)] p-3 text-center text-xl font-mono outline-none hover:border-[var(--c-border-light)] focus:border-[var(--disco-accent-orange)] transition-colors"
        style={{ color: value ? color : "var(--c-ghost)" }}
      />

      {open && (
        <div className="absolute z-50 mt-1 w-[280px] left-1/2 -translate-x-1/2 bg-[var(--c-dark)] border border-[var(--c-border)] shadow-2xl">
          {/* Header */}
          <div className="flex items-center justify-between px-3 py-2 border-b border-[var(--c-border)] bg-[var(--c-surface)]">
            <button type="button" onClick={() => setViewYear(viewYear - 1)}
              className="text-[var(--c-dim)] hover:text-white text-xs font-mono px-1 transition-colors">‹‹</button>
            <button type="button" onClick={prevMonth}
              className="text-[var(--c-dim)] hover:text-white text-sm font-mono px-1 transition-colors">‹</button>
            <span className="text-sm font-mono tracking-widest text-white uppercase">
              {MONTHS[viewMonth]} {viewYear}
            </span>
            <button type="button" onClick={nextMonth}
              className="text-[var(--c-dim)] hover:text-white text-sm font-mono px-1 transition-colors">›</button>
            <button type="button" onClick={() => setViewYear(viewYear + 1)}
              className="text-[var(--c-dim)] hover:text-white text-xs font-mono px-1 transition-colors">››</button>
          </div>

          {/* Day labels */}
          <div className="grid grid-cols-7 px-2 pt-2">
            {["SU", "MO", "TU", "WE", "TH", "FR", "SA"].map(d => (
              <div key={d} className="text-center text-[9px] font-mono text-[var(--c-faint)] pb-1">{d}</div>
            ))}
          </div>

          {/* Days grid */}
          <div className="grid grid-cols-7 px-2 pb-2 gap-[1px]">
            {Array.from({ length: firstDay }).map((_, i) => <div key={`e-${i}`} />)}
            {Array.from({ length: days }, (_, i) => i + 1).map(day => (
              <button
                key={day}
                type="button"
                onClick={() => selectDay(day)}
                className={`h-8 text-xs font-mono transition-all ${
                  isSelected(day)
                    ? "text-black font-bold"
                    : "text-[var(--disco-text-secondary)] hover:text-white hover:bg-[var(--c-deep)]"
                }`}
                style={isSelected(day) ? { backgroundColor: color } : {}}
              >
                {day}
              </button>
            ))}
          </div>

          {/* Quick actions */}
          <div className="flex border-t border-[var(--c-border)] text-[10px] font-mono uppercase">
            <button type="button"
              onClick={() => { onChange(formatDate(today.getFullYear(), today.getMonth(), today.getDate())); setOpen(false); }}
              className="flex-1 py-2 text-[var(--c-dim)] hover:text-white hover:bg-[var(--c-deep)] transition-colors text-center">
              Today
            </button>
            <button type="button"
              onClick={() => { onChange(""); setOpen(false); }}
              className="flex-1 py-2 text-[var(--c-dim)] hover:text-red-400 hover:bg-[var(--c-deep)] transition-colors text-center border-l border-[var(--c-border)]">
              Clear
            </button>
          </div>
        </div>
      )}
    </div>
  );
}
