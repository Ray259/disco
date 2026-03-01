import { useEffect, useState } from "react";
import { RichContentDisplay } from "./RichContentDisplay";
import { ENTITY_CONFIG } from "./EntityConfig";
import { formatHistoricalDate } from "../utils/dateUtils";

interface EntityDetailProps {
  entityType: string;
  name: string;
  onBack: () => void;
  onEntityClick?: (entityType: string, name: string) => void;
}

interface InfoCardProps {
  label: string;
  children: React.ReactNode;
  color: string;
}

function InfoCard({ label, color, children }: InfoCardProps) {
  return (
    <div className="p-6 bg-[var(--c-surface)] border-l-4" style={{ borderColor: color }}>
      <h3 className="text-sm font-header text-[var(--c-dim)] mb-2">{label}</h3>
      <div className="text-xl font-header text-white leading-none uppercase">{children}</div>
    </div>
  );
}

/** Small utility — render a RichContent field or a plain text as a section card */
function DetailSection({ label, color, children }: { label: string; color: string; children: React.ReactNode }) {
  return (
    <div className="special-section__card">
      <div className="special-section__card-marker" style={{ backgroundColor: color }} />
      <div>
        <h4 className="text-xs font-header uppercase tracking-wider mb-2" style={{ color }}>{label}</h4>
        <div className="text-sm font-body text-[var(--disco-text-secondary)] leading-relaxed">{children}</div>
      </div>
    </div>
  );
}

function SectionHeading({ label, color }: { label: string; color: string }) {
  return (
    <div className="flex items-center gap-4 mt-12 mb-6">
      <div className="h-px bg-[var(--c-border)] flex-1" />
      <span className="font-header text-sm tracking-[0.3em] uppercase" style={{ color }}>⟐ {label}</span>
      <div className="h-px bg-[var(--c-border)] flex-1" />
    </div>
  );
}

export function EntityDetail({ entityType, name, onBack, onEntityClick }: EntityDetailProps) {
  const [data, setData] = useState<any>(null);
  const [loading, setLoading] = useState(true);
  const [error, setError] = useState<string | null>(null);

  const config = ENTITY_CONFIG[entityType];
  const accentColor = config?.color || "var(--disco-accent-orange)";

  useEffect(() => {
    if (!config?.getById) {
      setError("No detail fetcher configured");
      setLoading(false);
      return;
    }
    config.getById(name)
      .then((d: any) => { setData(d); setLoading(false); })
      .catch((err: any) => { setError(String(err)); setLoading(false); });
  }, [name, entityType]);

  if (loading) return <div className="p-8 text-[var(--c-ghost)] h-full bg-[var(--c-dark)]/70">Loading...</div>;
  if (error) return <div className="p-8 text-red-500 h-full bg-[var(--c-dark)]/70">Error: {error}</div>;
  if (!data) return <div className="p-8 text-[var(--c-ghost)] h-full bg-[var(--c-dark)]/70">Not found</div>;

  const headerName = data.name || data.title || "Untitled";

  return (
    <div className="w-full h-full p-12 overflow-y-auto bg-[var(--c-dark)]/70 backdrop-blur-sm">
      <div className="max-w-4xl mx-auto bg-[var(--c-dark)]/60 border border-[var(--c-border)] p-10">
        <div className="flex justify-between items-center mb-10 border-b-2 pb-2" style={{ borderColor: accentColor }}>
          <div className="flex gap-4 text-mono-sm" style={{ color: accentColor }}>
            <span>Encyclopedia</span>
            <span>/</span>
            <span>{config?.title || entityType}</span>
            <span>/</span>
            <span className="text-white">{(data.name || data.title || '').toUpperCase()}</span>
          </div>
          <button onClick={onBack} className="text-xs uppercase font-bold text-[var(--c-muted)] hover:text-white transition-colors">
            [ Back ]
          </button>
        </div>

        <h1 className="text-5xl font-header text-white uppercase leading-[0.85] mb-8 tracking-tight">
          {headerName}
        </h1>

        {renderEntityBody(entityType, data, accentColor, onEntityClick)}
      </div>
    </div>
  );
}

function renderEntityBody(entityType: string, data: any, _color: string, onEntityClick?: (t: string, n: string) => void) {
  switch (entityType) {
    case "figures":
      return <FigureBody data={data} onEntityClick={onEntityClick} />;
    case "institutions":
      return <InstitutionBody data={data} onEntityClick={onEntityClick} />;
    case "events":
      return <EventBody data={data} onEntityClick={onEntityClick} />;
    case "geos":
      return <GeoBody data={data} onEntityClick={onEntityClick} />;
    case "works":
      return <WorkBody data={data} onEntityClick={onEntityClick} />;
    case "schools":
      return <SchoolBody data={data} onEntityClick={onEntityClick} />;
    default:
      return <div className="text-[var(--c-dim)] italic">No detail layout for this type.</div>;
  }
}

/* ══════════════════════ FIGURE ══════════════════════ */

function FigureBody({ data, onEntityClick }: { data: any; onEntityClick?: (t: string, n: string) => void }) {
  return (
    <>
      {/* ── Life Dates ── */}
      <div className="flex items-center gap-4 mb-10">
        <div className="h-px bg-[var(--c-border-light)] flex-1" />
        <div className="text-lg font-body italic text-[var(--disco-accent-orange)]">
          {formatHistoricalDate(data.life?.start)} &mdash; {formatHistoricalDate(data.life?.end)}
        </div>
        <div className="h-px bg-[var(--c-border-light)] w-12" />
      </div>

      {/* ── Primary Info Cards ── */}
      <div className="grid grid-cols-1 lg:grid-cols-12 gap-12">
        <div className="lg:col-span-4 space-y-6">
          <InfoCard label="Primary Role" color="var(--disco-accent-yellow)">
            <RichContentDisplay content={data.primary_role} onEntityClick={onEntityClick} />
          </InfoCard>
          <InfoCard label="Origin" color="var(--disco-accent-purple)">
            <RichContentDisplay content={data.primary_location} onEntityClick={onEntityClick} />
          </InfoCard>
        </div>

        <div className="lg:col-span-8">
          {data.defining_quote ? (
            <div className="relative">
              <div className="absolute -top-8 -left-8 text-8xl font-serif text-[var(--c-deep)] z-0">"</div>
              <blockquote className="relative z-10 text-2xl font-body text-[var(--disco-text-primary)] leading-relaxed italic">
                <RichContentDisplay content={data.defining_quote} onEntityClick={onEntityClick} />
              </blockquote>
              <div className="mt-4 text-right">
                <span className="text-mono-sm text-[var(--c-muted)]">&mdash; RÉTROSPECTIVE</span>
              </div>
            </div>
          ) : (
            <div className="text-zinc-600 italic font-body">No recording of speech available.</div>
          )}
        </div>
      </div>

      {/* ── Zeitgeist ── */}
      {data.zeitgeist && (data.zeitgeist.era || data.zeitgeist.catalyst || data.zeitgeist.opposition) && (
        <>
          <SectionHeading label="Zeitgeist" color="var(--disco-accent-teal)" />
          <div className="special-section" style={{ "--section-accent": "var(--disco-accent-teal)" } as React.CSSProperties}>
            {data.zeitgeist.era && (
              <div className="mb-4">
                <span className="text-[10px] font-mono uppercase tracking-[0.2em] text-[var(--disco-accent-teal)]">Era</span>
                <div className="text-lg font-header uppercase text-white mt-1">{data.zeitgeist.era}</div>
              </div>
            )}
            <div className="grid grid-cols-1 md:grid-cols-2 gap-6">
              {data.zeitgeist.catalyst && (
                <DetailSection label="Catalyst" color="var(--disco-accent-teal)">
                  {data.zeitgeist.catalyst}
                </DetailSection>
              )}
              {data.zeitgeist.opposition && (
                <DetailSection label="Opposition" color="var(--disco-accent-purple)">
                  {data.zeitgeist.opposition}
                </DetailSection>
              )}
            </div>
          </div>
        </>
      )}

      {/* ── Core Ideology ── */}
      {(data.axiom || data.argument_flow) && (
        <>
          <SectionHeading label="Core Ideology" color="var(--disco-accent-yellow)" />
          <div className="special-section" style={{ "--section-accent": "var(--disco-accent-yellow)" } as React.CSSProperties}>
            {data.axiom && (
              <div className="axiom-field mb-6">
                <div className="axiom-field__quote-mark">"</div>
                <div className="axiom-field__content">
                  <span className="block text-[10px] font-mono uppercase tracking-[0.2em] mb-2 text-[var(--disco-accent-yellow)]">
                    Axiom — The First Principle
                  </span>
                  <div className="text-lg font-body italic text-[var(--disco-text-primary)] leading-relaxed">
                    <RichContentDisplay content={data.axiom} onEntityClick={onEntityClick} />
                  </div>
                </div>
              </div>
            )}
            {data.argument_flow && (
              <div className="argument-flow">
                <div className="argument-flow__line" />
                <span className="block text-[10px] font-mono uppercase tracking-[0.2em] mb-2 text-[var(--disco-accent-yellow)]">
                  Argument Flow
                </span>
                <div className="text-sm font-body text-[var(--disco-text-secondary)] leading-relaxed whitespace-pre-wrap">
                  <RichContentDisplay content={data.argument_flow} onEntityClick={onEntityClick} />
                </div>
              </div>
            )}
          </div>
        </>
      )}

      {/* ── Key Terminology ── */}
      {data.key_terminology && Object.keys(data.key_terminology).length > 0 && (
        <>
          <SectionHeading label="Key Terminology" color="#06b6d4" />
          <div className="special-section" style={{ "--section-accent": "#06b6d4" } as React.CSSProperties}>
            <div className="space-y-0">
              {Object.entries(data.key_terminology).map(([term, definition]: [string, any]) => (
                <div key={term} className="terminology-entry">
                  <div className="terminology-entry__term">{term}</div>
                  <div className="terminology-entry__def">
                    {typeof definition === "object" && definition?.segments ? (
                      <RichContentDisplay content={definition} onEntityClick={onEntityClick} />
                    ) : (
                      String(definition || "—")
                    )}
                  </div>
                </div>
              ))}
            </div>
          </div>
        </>
      )}

      {/* ── Intellectual Lineage ── */}
      {(data.predecessors?.length > 0 || data.contemporary_rivals?.length > 0 || data.successors?.length > 0) && (
        <>
          <SectionHeading label="Intellectual Lineage" color="#a78bfa" />
          <div className="special-section" style={{ "--section-accent": "#a78bfa" } as React.CSSProperties}>
            <div className="grid grid-cols-1 md:grid-cols-3 gap-6">
              {data.predecessors?.length > 0 && (
                <div>
                  <h4 className="text-[10px] font-mono uppercase tracking-[0.25em] mb-3 text-[var(--disco-accent-teal)]">Predecessors</h4>
                  <div className="space-y-1.5">
                    {data.predecessors.map((ref: any, i: number) => (
                      <div key={i} className="lineage-chip" style={{ borderColor: "var(--disco-accent-teal)" }}>
                        <span
                          className="text-xs font-body text-[var(--disco-text-primary)] truncate cursor-pointer hover:underline"
                          style={{ color: "var(--disco-accent-teal)" }}
                          onClick={() => onEntityClick?.("figures", ref.display_text)}
                        >
                          {ref.display_text}
                        </span>
                      </div>
                    ))}
                  </div>
                </div>
              )}
              {data.contemporary_rivals?.length > 0 && (
                <div>
                  <h4 className="text-[10px] font-mono uppercase tracking-[0.25em] mb-3 text-[#ef4444]">⚔ Rivals</h4>
                  <div className="space-y-1.5">
                    {data.contemporary_rivals.map((ref: any, i: number) => (
                      <div key={i} className="lineage-chip" style={{ borderColor: "#ef4444" }}>
                        <span
                          className="text-xs font-body text-[var(--disco-text-primary)] truncate cursor-pointer hover:underline"
                          style={{ color: "#ef4444" }}
                          onClick={() => onEntityClick?.("figures", ref.display_text)}
                        >
                          {ref.display_text}
                        </span>
                      </div>
                    ))}
                  </div>
                </div>
              )}
              {data.successors?.length > 0 && (
                <div>
                  <h4 className="text-[10px] font-mono uppercase tracking-[0.25em] mb-3 text-[var(--disco-accent-yellow)]">Successors</h4>
                  <div className="space-y-1.5">
                    {data.successors.map((ref: any, i: number) => (
                      <div key={i} className="lineage-chip" style={{ borderColor: "var(--disco-accent-yellow)" }}>
                        <span
                          className="text-xs font-body text-[var(--disco-text-primary)] truncate cursor-pointer hover:underline"
                          style={{ color: "var(--disco-accent-yellow)" }}
                          onClick={() => onEntityClick?.("figures", ref.display_text)}
                        >
                          {ref.display_text}
                        </span>
                      </div>
                    ))}
                  </div>
                </div>
              )}
            </div>
          </div>
        </>
      )}

      {/* ── Legacy ── */}
      {(data.short_term_success || data.modern_relevance || data.critical_flaw || data.personal_synthesis) && (
        <>
          <SectionHeading label="Legacy" color="#f59e0b" />
          <div className="special-section" style={{ "--section-accent": "#f59e0b" } as React.CSSProperties}>
            <div className="space-y-4">
              {data.short_term_success && (
                <div className="legacy-card">
                  <div className="legacy-card__border" style={{ backgroundColor: "#22c55e" }} />
                  <div className="legacy-card__content">
                    <h4 className="text-xs font-header uppercase tracking-wider mb-2" style={{ color: "#22c55e" }}>Short-term Success</h4>
                    <div className="text-sm font-body text-[var(--disco-text-secondary)]">
                      <RichContentDisplay content={data.short_term_success} onEntityClick={onEntityClick} />
                    </div>
                  </div>
                </div>
              )}
              {data.modern_relevance && (
                <div className="legacy-card">
                  <div className="legacy-card__border" style={{ backgroundColor: "var(--disco-accent-teal)" }} />
                  <div className="legacy-card__content">
                    <h4 className="text-xs font-header uppercase tracking-wider mb-2" style={{ color: "var(--disco-accent-teal)" }}>Modern Relevance</h4>
                    <div className="text-sm font-body text-[var(--disco-text-secondary)]">
                      <RichContentDisplay content={data.modern_relevance} onEntityClick={onEntityClick} />
                    </div>
                  </div>
                </div>
              )}
              {data.critical_flaw && (
                <div className="legacy-card">
                  <div className="legacy-card__border" style={{ backgroundColor: "#ef4444" }} />
                  <div className="legacy-card__content">
                    <h4 className="text-xs font-header uppercase tracking-wider mb-2" style={{ color: "#ef4444" }}>Critical Flaw</h4>
                    <div className="text-sm font-body text-[var(--disco-text-secondary)]">
                      <RichContentDisplay content={data.critical_flaw} onEntityClick={onEntityClick} />
                    </div>
                  </div>
                </div>
              )}
              {data.personal_synthesis && (
                <div className="legacy-card">
                  <div className="legacy-card__border" style={{ backgroundColor: "var(--disco-accent-yellow)" }} />
                  <div className="legacy-card__content">
                    <h4 className="text-xs font-header uppercase tracking-wider mb-2" style={{ color: "var(--disco-accent-yellow)" }}>Personal Synthesis</h4>
                    <div className="text-sm font-body text-[var(--disco-text-secondary)]">
                      <RichContentDisplay content={data.personal_synthesis} onEntityClick={onEntityClick} />
                    </div>
                  </div>
                </div>
              )}
            </div>
          </div>
        </>
      )}

      {/* ── Major Contributions ── */}
      {data.major_contributions?.length > 0 && (
        <>
          <SectionHeading label="Major Contributions" color="var(--disco-accent-orange)" />
          <div className="special-section" style={{ "--section-accent": "var(--disco-accent-orange)" } as React.CSSProperties}>
            <div className="contributions-timeline">
              {data.major_contributions.map((c: any, i: number) => (
                <div key={i} className="contributions-entry">
                  <div className="contributions-entry__marker" />
                  <div className="contributions-entry__content">
                    <div className="flex items-baseline gap-3">
                      <span className="text-xs font-header uppercase tracking-wider text-[var(--disco-accent-orange)]">{c.title}</span>
                      {c.date && (
                        <span className="text-[9px] font-mono text-[var(--c-ghost)]">
                          {formatHistoricalDate(c.date.start)} — {formatHistoricalDate(c.date.end)}
                        </span>
                      )}
                    </div>
                    {c.impact && (
                      <div className="text-xs font-body text-[var(--disco-text-secondary)] mt-1">
                        <RichContentDisplay content={c.impact} onEntityClick={onEntityClick} />
                      </div>
                    )}
                  </div>
                </div>
              ))}
            </div>
          </div>
        </>
      )}

      {/* ── Institutional Power Base ── */}
      {(data.primary_institution || data.funding_model || data.institutional_product || data.succession_plan) && (
        <>
          <SectionHeading label="Institutional Power Base" color="var(--disco-accent-yellow)" />
          <div className="special-section" style={{ "--section-accent": "var(--disco-accent-yellow)" } as React.CSSProperties}>
            <div className="space-y-6">
              {data.primary_institution && (
                <InfoCard label="Primary Institution" color="var(--disco-accent-yellow)">
                  <span
                    className="cursor-pointer hover:underline"
                    style={{ color: "var(--disco-accent-yellow)" }}
                    onClick={() => onEntityClick?.("institutions", data.primary_institution.display_text)}
                  >
                    {data.primary_institution.display_text}
                  </span>
                </InfoCard>
              )}
              {data.funding_model && (
                <DetailSection label="Funding Model" color="var(--disco-accent-yellow)">
                  <RichContentDisplay content={data.funding_model} onEntityClick={onEntityClick} />
                </DetailSection>
              )}
              {data.institutional_product && (
                <DetailSection label="Institutional Product" color="var(--disco-accent-teal)">
                  <RichContentDisplay content={data.institutional_product} onEntityClick={onEntityClick} />
                </DetailSection>
              )}
              {data.succession_plan && (
                <DetailSection label="Succession Plan" color="var(--disco-accent-purple)">
                  <RichContentDisplay content={data.succession_plan} onEntityClick={onEntityClick} />
                </DetailSection>
              )}
            </div>
          </div>
        </>
      )}
    </>
  );
}

/* ══════════════════════ INSTITUTION ══════════════════════ */

function InstitutionBody({ data, onEntityClick }: { data: any; onEntityClick?: (t: string, n: string) => void }) {
  return (
    <div className="space-y-8">
      {data.founded && (
        <div className="flex items-center gap-4">
          <div className="h-px bg-[var(--c-border-light)] flex-1" />
          <div className="text-lg font-body italic text-[var(--disco-accent-yellow)]">
            Founded {formatHistoricalDate(data.founded.start)} &mdash; {formatHistoricalDate(data.founded.end)}
          </div>
          <div className="h-px bg-[var(--c-border-light)] w-12" />
        </div>
      )}
      {data.description && (
        <div className="text-lg font-body text-[var(--disco-text-primary)] leading-relaxed">
          <RichContentDisplay content={data.description} onEntityClick={onEntityClick} />
        </div>
      )}
      {!data.description && <div className="text-zinc-600 italic font-body">No records available.</div>}
    </div>
  );
}

/* ══════════════════════ EVENT ══════════════════════ */

function EventBody({ data, onEntityClick }: { data: any; onEntityClick?: (t: string, n: string) => void }) {
  return (
    <div className="space-y-8">
      {data.date_range && (
        <div className="flex items-center gap-4">
          <div className="h-px bg-[var(--c-border-light)] flex-1" />
          <div className="text-lg font-body italic text-[var(--disco-accent-purple)]">
            {formatHistoricalDate(data.date_range.start)} &mdash; {formatHistoricalDate(data.date_range.end)}
          </div>
          <div className="h-px bg-[var(--c-border-light)] w-12" />
        </div>
      )}
      {data.description && (
        <div className="text-lg font-body text-[var(--disco-text-primary)] leading-relaxed">
          <RichContentDisplay content={data.description} onEntityClick={onEntityClick} />
        </div>
      )}
      {!data.description && <div className="text-zinc-600 italic font-body">No account of this event survives.</div>}
    </div>
  );
}

/* ══════════════════════ GEO ══════════════════════ */

function GeoBody({ data, onEntityClick }: { data: any; onEntityClick?: (t: string, n: string) => void }) {
  return (
    <div className="space-y-8">
      {data.region && (
        <InfoCard label="Region" color="var(--disco-accent-teal)">
          <RichContentDisplay content={data.region} onEntityClick={onEntityClick} />
        </InfoCard>
      )}
      {data.description && (
        <div className="text-lg font-body text-[var(--disco-text-primary)] leading-relaxed">
          <RichContentDisplay content={data.description} onEntityClick={onEntityClick} />
        </div>
      )}
      {!data.description && <div className="text-zinc-600 italic font-body">This territory remains unmapped.</div>}
    </div>
  );
}

/* ══════════════════════ WORK ══════════════════════ */

function WorkBody({ data, onEntityClick }: { data: any; onEntityClick?: (t: string, n: string) => void }) {
  return (
    <div className="space-y-8">
      {data.summary ? (
        <div className="relative">
          <div className="absolute -top-8 -left-8 text-8xl font-serif text-[var(--c-deep)] z-0">"</div>
          <blockquote className="relative z-10 text-xl font-body text-[var(--disco-text-primary)] leading-relaxed italic">
            <RichContentDisplay content={data.summary} onEntityClick={onEntityClick} />
          </blockquote>
        </div>
      ) : (
        <div className="text-zinc-600 italic font-body">No summary has been recorded for this work.</div>
      )}
    </div>
  );
}

/* ══════════════════════ SCHOOL ══════════════════════ */

function SchoolBody({ data, onEntityClick }: { data: any; onEntityClick?: (t: string, n: string) => void }) {
  return (
    <div className="space-y-8">
      {data.description ? (
        <div className="text-lg font-body text-[var(--disco-text-primary)] leading-relaxed border-l-2 border-[var(--disco-accent-orange)] pl-6">
          <RichContentDisplay content={data.description} onEntityClick={onEntityClick} />
        </div>
      ) : (
        <div className="text-zinc-600 italic font-body">The tenets of this school remain unrecorded.</div>
      )}
    </div>
  );
}
