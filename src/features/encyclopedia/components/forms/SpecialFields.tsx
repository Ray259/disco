import React, { useState } from "react";

/* ─────────────────────────────────────────────
   SHARED SPECIAL-FIELD PRIMITIVES
   ───────────────────────────────────────────── */

interface SectionDividerProps {
  label: string;
  color?: string;
  collapsed?: boolean;
  onToggle?: () => void;
}

export function SectionDivider({ label, color = "var(--c-muted)", collapsed, onToggle }: SectionDividerProps) {
  return (
    <button
      type="button"
      onClick={onToggle}
      className="w-full flex items-center gap-4 py-6 group cursor-pointer select-none"
    >
      <div className="h-px flex-1 bg-[var(--c-border)]" />
      <span
        className="font-header text-sm tracking-[0.3em] uppercase transition-colors group-hover:brightness-125"
        style={{ color }}
      >
        ⟐ {label}
      </span>
      {onToggle && (
        <span className="text-[10px] font-mono text-[var(--c-ghost)] transition-transform" style={{ transform: collapsed ? "rotate(-90deg)" : "rotate(0deg)" }}>
          ▼
        </span>
      )}
      <div className="h-px flex-1 bg-[var(--c-border)]" />
    </button>
  );
}

function SpecialTextArea({ label, value, onChange, placeholder, rows = 3, accentColor }: {
  label: string;
  value: string;
  onChange: (v: string) => void;
  placeholder?: string;
  rows?: number;
  accentColor?: string;
}) {
  return (
    <div className="group">
      <label className="block text-[10px] font-mono uppercase tracking-[0.2em] mb-2 transition-colors"
        style={{ color: accentColor || "var(--c-muted)" }}
      >
        {label}
      </label>
      <textarea
        rows={rows}
        value={value}
        onChange={e => onChange(e.target.value)}
        placeholder={placeholder}
        className="w-full bg-transparent border border-[var(--c-border)] p-3 text-sm font-body text-[var(--disco-text-primary)] focus:outline-none transition-colors placeholder-[var(--c-ghost)] resize-none"
        style={{ borderColor: undefined }}
        onFocus={e => { if (accentColor) e.currentTarget.style.borderColor = accentColor; }}
        onBlur={e => { e.currentTarget.style.borderColor = ""; }}
      />
    </div>
  );
}

function SpecialInput({ label, value, onChange, placeholder, accentColor }: {
  label: string;
  value: string;
  onChange: (v: string) => void;
  placeholder?: string;
  accentColor?: string;
}) {
  return (
    <div className="group">
      <label className="block text-[10px] font-mono uppercase tracking-[0.2em] mb-1 transition-colors"
        style={{ color: accentColor || "var(--c-muted)" }}
      >
        {label}
      </label>
      <input
        type="text"
        value={value}
        onChange={e => onChange(e.target.value)}
        placeholder={placeholder}
        className="w-full bg-transparent border-b border-[var(--c-border)] py-2 text-sm font-body text-[var(--disco-text-primary)] focus:outline-none transition-colors placeholder-[var(--c-ghost)]"
        onFocus={e => { if (accentColor) e.currentTarget.style.borderColor = accentColor; }}
        onBlur={e => { e.currentTarget.style.borderColor = ""; }}
      />
    </div>
  );
}

/* ─────────────────────────────────────────────
   1. ZEITGEIST — "Spirit of the Age"
   ───────────────────────────────────────────── */

export interface ZeitgeistState {
  era: string;
  catalyst: string;
  opposition: string;
}

interface ZeitgeistFieldProps {
  value: ZeitgeistState;
  onChange: (v: ZeitgeistState) => void;
}

export function ZeitgeistField({ value, onChange }: ZeitgeistFieldProps) {
  const [collapsed, setCollapsed] = useState(false);

  const update = (field: keyof ZeitgeistState, v: string) => {
    onChange({ ...value, [field]: v });
  };

  return (
    <div>
      <SectionDivider label="Zeitgeist" color="var(--disco-accent-teal)" collapsed={collapsed} onToggle={() => setCollapsed(!collapsed)} />

      {!collapsed && (
        <div className="special-section" style={{ "--section-accent": "var(--disco-accent-teal)" } as React.CSSProperties}>
          {/* Era — full width header */}
          <div className="mb-6">
            <SpecialInput
              label="Era / Historical Period"
              value={value.era}
              onChange={v => update("era", v)}
              placeholder="e.g. The Enlightenment, Classical Antiquity..."
              accentColor="var(--disco-accent-teal)"
            />
          </div>

          {/* Catalyst & Opposition — side by side */}
          <div className="grid grid-cols-1 md:grid-cols-2 gap-6">
            <div className="special-section__card special-section__card--catalyst">
              <div className="special-section__card-marker" style={{ backgroundColor: "var(--disco-accent-teal)" }} />
              <SpecialTextArea
                label="Catalyst"
                value={value.catalyst}
                onChange={v => update("catalyst", v)}
                placeholder="What triggered their work? The crisis, event, or condition that set them in motion..."
                rows={4}
                accentColor="var(--disco-accent-teal)"
              />
            </div>

            <div className="special-section__card special-section__card--opposition">
              <div className="special-section__card-marker" style={{ backgroundColor: "var(--disco-accent-purple)" }} />
              <SpecialTextArea
                label="Opposition"
                value={value.opposition}
                onChange={v => update("opposition", v)}
                placeholder="What did they fight against? The institution, doctrine, or rival they opposed..."
                rows={4}
                accentColor="var(--disco-accent-purple)"
              />
            </div>
          </div>

          {/* Influences placeholder — will be entity refs later */}
          <div className="mt-6 p-4 border border-dashed border-[var(--c-border)] opacity-50">
            <span className="text-[10px] font-mono uppercase tracking-[0.2em] text-[var(--c-ghost)]">
              ◇ Influences — entity linking (coming soon)
            </span>
          </div>
        </div>
      )}
    </div>
  );
}

/* ─────────────────────────────────────────────
   2. AXIOM & ARGUMENT FLOW — "Core Ideology"
   ───────────────────────────────────────────── */

export interface CoreIdeologyState {
  axiom: string;
  argumentFlow: string;
}

interface CoreIdeologyFieldProps {
  value: CoreIdeologyState;
  onChange: (v: CoreIdeologyState) => void;
}

export function CoreIdeologyField({ value, onChange }: CoreIdeologyFieldProps) {
  const [collapsed, setCollapsed] = useState(false);

  return (
    <div>
      <SectionDivider label="Core Ideology" color="var(--disco-accent-yellow)" collapsed={collapsed} onToggle={() => setCollapsed(!collapsed)} />

      {!collapsed && (
        <div className="special-section" style={{ "--section-accent": "var(--disco-accent-yellow)" } as React.CSSProperties}>
          {/* Axiom — blockquote-style */}
          <div className="axiom-field">
            <div className="axiom-field__quote-mark">"</div>
            <div className="axiom-field__content">
              <label className="block text-[10px] font-mono uppercase tracking-[0.2em] mb-2 text-[var(--disco-accent-yellow)]">
                Axiom — The First Principle
              </label>
              <textarea
                value={value.axiom}
                onChange={e => onChange({ ...value, axiom: e.target.value })}
                placeholder="The fundamental truth or rule they lived by..."
                rows={3}
                className="axiom-field__textarea"
              />
            </div>
          </div>

          {/* Argument Flow */}
          <div className="mt-8">
            <label className="block text-[10px] font-mono uppercase tracking-[0.2em] mb-2 text-[var(--disco-accent-yellow)]">
              Argument Flow — Logical Progression
            </label>
            <div className="argument-flow">
              <div className="argument-flow__line" />
              <textarea
                value={value.argumentFlow}
                onChange={e => onChange({ ...value, argumentFlow: e.target.value })}
                placeholder={"1. All men are mortal\n2. Socrates is a man\n3. Therefore, Socrates is mortal"}
                rows={5}
                className="argument-flow__textarea"
              />
            </div>
          </div>
        </div>
      )}
    </div>
  );
}

/* ─────────────────────────────────────────────
   3. INTELLECTUAL LINEAGE — "Genealogy"
   ───────────────────────────────────────────── */

export interface LineageState {
  predecessors: string[];
  rivals: string[];
  successors: string[];
}

interface LineageFieldProps {
  value: LineageState;
  onChange: (v: LineageState) => void;
}

function LineageColumn({ label, items, onAdd, onRemove, accentColor, placeholder }: {
  label: string;
  items: string[];
  onAdd: (name: string) => void;
  onRemove: (index: number) => void;
  accentColor: string;
  placeholder: string;
}) {
  const [draft, setDraft] = useState("");

  const handleKeyDown = (e: React.KeyboardEvent) => {
    if (e.key === "Enter" && draft.trim()) {
      e.preventDefault();
      onAdd(draft.trim());
      setDraft("");
    }
  };

  return (
    <div className="lineage-column">
      <h4 className="text-[10px] font-mono uppercase tracking-[0.25em] mb-3 text-center" style={{ color: accentColor }}>
        {label}
      </h4>
      <div className="space-y-1.5 mb-3 min-h-[40px]">
        {items.map((name, i) => (
          <div key={i} className="lineage-chip" style={{ borderColor: accentColor }}>
            <span className="text-xs font-body text-[var(--disco-text-primary)] truncate">{name}</span>
            <button
              type="button"
              onClick={() => onRemove(i)}
              className="text-[10px] text-[var(--c-ghost)] hover:text-red-400 transition-colors ml-2"
            >
              ×
            </button>
          </div>
        ))}
      </div>
      <input
        type="text"
        value={draft}
        onChange={e => setDraft(e.target.value)}
        onKeyDown={handleKeyDown}
        placeholder={placeholder}
        className="w-full bg-transparent border-b border-[var(--c-border)] py-1 text-xs font-mono text-[var(--disco-text-secondary)] focus:outline-none placeholder-[var(--c-ghost)]"
        onFocus={e => { e.currentTarget.style.borderColor = accentColor; }}
        onBlur={e => { e.currentTarget.style.borderColor = ""; }}
      />
      <span className="text-[9px] font-mono text-[var(--c-ghost)] mt-1 block">Press Enter to add</span>
    </div>
  );
}

export function LineageField({ value, onChange }: LineageFieldProps) {
  const [collapsed, setCollapsed] = useState(false);

  const addTo = (field: keyof LineageState, name: string) => {
    onChange({ ...value, [field]: [...value[field], name] });
  };
  const removeFrom = (field: keyof LineageState, index: number) => {
    onChange({ ...value, [field]: value[field].filter((_, i) => i !== index) });
  };

  return (
    <div>
      <SectionDivider label="Intellectual Lineage" color="#a78bfa" collapsed={collapsed} onToggle={() => setCollapsed(!collapsed)} />

      {!collapsed && (
        <div className="special-section" style={{ "--section-accent": "#a78bfa" } as React.CSSProperties}>
          {/* Main flow: Predecessors → [CENTER] → Successors */}
          <div className="lineage-flow">
            <LineageColumn
              label="Predecessors"
              items={value.predecessors}
              onAdd={n => addTo("predecessors", n)}
              onRemove={i => removeFrom("predecessors", i)}
              accentColor="var(--disco-accent-teal)"
              placeholder="Who inspired them?"
            />

            <div className="lineage-flow__arrow">
              <div className="lineage-flow__arrow-line" />
              <span className="lineage-flow__arrow-head">▶</span>
              <div className="lineage-flow__arrow-line" />
            </div>

            <div className="lineage-flow__center">
              <div className="lineage-flow__center-diamond">◆</div>
              <span className="text-[9px] font-mono uppercase tracking-widest text-[var(--c-ghost)] mt-1">Subject</span>
            </div>

            <div className="lineage-flow__arrow">
              <div className="lineage-flow__arrow-line" />
              <span className="lineage-flow__arrow-head">▶</span>
              <div className="lineage-flow__arrow-line" />
            </div>

            <LineageColumn
              label="Successors"
              items={value.successors}
              onAdd={n => addTo("successors", n)}
              onRemove={i => removeFrom("successors", i)}
              accentColor="var(--disco-accent-yellow)"
              placeholder="Who carried on?"
            />
          </div>

          {/* Rivals — separate row */}
          <div className="mt-6 pt-4 border-t border-[var(--c-border)]">
            <LineageColumn
              label="⚔ Contemporary Rivals"
              items={value.rivals}
              onAdd={n => addTo("rivals", n)}
              onRemove={i => removeFrom("rivals", i)}
              accentColor="#ef4444"
              placeholder="Who opposed them?"
            />
          </div>
        </div>
      )}
    </div>
  );
}

/* ─────────────────────────────────────────────
   4. LEGACY — "The Reckoning"
   ───────────────────────────────────────────── */

export interface LegacyState {
  shortTermSuccess: string;
  modernRelevance: string;
  criticalFlaw: string;
  personalSynthesis: string;
}

interface LegacyFieldProps {
  value: LegacyState;
  onChange: (v: LegacyState) => void;
}

const LEGACY_CARDS: { key: keyof LegacyState; label: string; sublabel: string; color: string; placeholder: string }[] = [
  {
    key: "shortTermSuccess",
    label: "Short-term Success",
    sublabel: "Immediate impact during their time",
    color: "#22c55e",
    placeholder: "What did they achieve in their lifetime?",
  },
  {
    key: "modernRelevance",
    label: "Modern Relevance",
    sublabel: "How they echo through history",
    color: "var(--disco-accent-teal)",
    placeholder: "How are they viewed or used today?",
  },
  {
    key: "criticalFlaw",
    label: "Critical Flaw",
    sublabel: "The failing that undid them",
    color: "#ef4444",
    placeholder: "What was their blind spot, their hubris?",
  },
  {
    key: "personalSynthesis",
    label: "Personal Synthesis",
    sublabel: "Their own final verdict",
    color: "var(--disco-accent-yellow)",
    placeholder: "How did they ultimately view their life's work?",
  },
];

export function LegacyField({ value, onChange }: LegacyFieldProps) {
  const [collapsed, setCollapsed] = useState(false);

  return (
    <div>
      <SectionDivider label="Legacy" color="#f59e0b" collapsed={collapsed} onToggle={() => setCollapsed(!collapsed)} />

      {!collapsed && (
        <div className="special-section" style={{ "--section-accent": "#f59e0b" } as React.CSSProperties}>
          <div className="space-y-4">
            {LEGACY_CARDS.map((card, i) => (
              <div
                key={card.key}
                className="legacy-card"
                style={{
                  "--legacy-accent": card.color,
                  animationDelay: `${i * 60}ms`,
                } as React.CSSProperties}
              >
                <div className="legacy-card__border" style={{ backgroundColor: card.color }} />
                <div className="legacy-card__content">
                  <div className="flex items-baseline justify-between mb-2">
                    <h4 className="text-xs font-header uppercase tracking-wider" style={{ color: card.color }}>
                      {card.label}
                    </h4>
                    <span className="text-[9px] font-mono text-[var(--c-ghost)] italic">{card.sublabel}</span>
                  </div>
                  <textarea
                    value={value[card.key]}
                    onChange={e => onChange({ ...value, [card.key]: e.target.value })}
                    placeholder={card.placeholder}
                    rows={card.key === "personalSynthesis" ? 4 : 2}
                    className="w-full bg-transparent border-none p-0 text-sm font-body text-[var(--disco-text-primary)] focus:outline-none placeholder-[var(--c-ghost)] resize-none"
                  />
                </div>
              </div>
            ))}
          </div>
        </div>
      )}
    </div>
  );
}
