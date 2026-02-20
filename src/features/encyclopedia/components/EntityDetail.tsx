import { useEffect, useState } from "react";
import { RichContentDisplay } from "./RichContentDisplay";
import { ENTITY_CONFIG } from "./EntityConfig";

interface EntityDetailProps {
  entityType: string;
  id: string;
  onBack: () => void;
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

export function EntityDetail({ entityType, id, onBack }: EntityDetailProps) {
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
    config.getById(id)
      .then((d: any) => { setData(d); setLoading(false); })
      .catch((err: any) => { setError(String(err)); setLoading(false); });
  }, [id, entityType]);

  if (loading) return <div className="p-8 text-[var(--c-ghost)] h-full bg-[var(--c-dark)]/70">Loading...</div>;
  if (error) return <div className="p-8 text-red-500 h-full bg-[var(--c-dark)]/70">Error: {error}</div>;
  if (!data) return <div className="p-8 text-[var(--c-ghost)] h-full bg-[var(--c-dark)]/70">Not found</div>;

  const name = data.name || data.title || "Untitled";

  return (
    <div className="w-full h-full p-12 overflow-y-auto bg-[var(--c-dark)]/70 backdrop-blur-sm">
      <div className="max-w-4xl mx-auto bg-[var(--c-dark)]/60 border border-[var(--c-border)] p-10">
        <div className="flex justify-between items-center mb-10 border-b-2 pb-2" style={{ borderColor: accentColor }}>
          <div className="flex gap-4 text-mono-sm" style={{ color: accentColor }}>
            <span>Encyclopedia</span>
            <span>/</span>
            <span>{config?.title || entityType}</span>
            <span>/</span>
            <span className="text-white">{id.split('-')[0].toUpperCase()}</span>
          </div>
          <button onClick={onBack} className="text-xs uppercase font-bold text-[var(--c-muted)] hover:text-white transition-colors">
            [ Back ]
          </button>
        </div>

        <h1 className="text-5xl font-header text-white uppercase leading-[0.85] mb-8 tracking-tight">
          {name}
        </h1>

        {renderEntityBody(entityType, data, accentColor)}
      </div>
    </div>
  );
}

function renderEntityBody(entityType: string, data: any, color: string) {
  switch (entityType) {
    case "figures":
      return <FigureBody data={data} />;
    case "institutions":
      return <InstitutionBody data={data} />;
    case "events":
      return <EventBody data={data} />;
    case "geos":
      return <GeoBody data={data} />;
    case "works":
      return <WorkBody data={data} />;
    case "schools":
      return <SchoolBody data={data} />;
    default:
      return <div className="text-[var(--c-dim)] italic">No detail layout for this type.</div>;
  }
}

function FigureBody({ data }: { data: any }) {
  return (
    <>
      <div className="flex items-center gap-4 mb-10">
        <div className="h-px bg-[var(--c-border-light)] flex-1" />
        <div className="text-lg font-body italic text-[var(--disco-accent-orange)]">
          {data.life?.start} &mdash; {data.life?.end}
        </div>
        <div className="h-px bg-[var(--c-border-light)] w-12" />
      </div>

      <div className="grid grid-cols-1 lg:grid-cols-12 gap-12">
        <div className="lg:col-span-4 space-y-6">
          <InfoCard label="Primary Role" color="var(--disco-accent-yellow)">
            <RichContentDisplay content={data.primary_role} />
          </InfoCard>
          <InfoCard label="Origin" color="var(--disco-accent-purple)">
            <RichContentDisplay content={data.primary_location} />
          </InfoCard>
        </div>

        <div className="lg:col-span-8">
          {data.defining_quote ? (
            <div className="relative">
              <div className="absolute -top-8 -left-8 text-8xl font-serif text-[var(--c-deep)] z-0">"</div>
              <blockquote className="relative z-10 text-2xl font-body text-[var(--disco-text-primary)] leading-relaxed italic">
                <RichContentDisplay content={data.defining_quote} />
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
    </>
  );
}

function InstitutionBody({ data }: { data: any }) {
  return (
    <div className="space-y-8">
      {data.founded && (
        <div className="flex items-center gap-4">
          <div className="h-px bg-[var(--c-border-light)] flex-1" />
          <div className="text-lg font-body italic text-[var(--disco-accent-yellow)]">
            Founded {data.founded.start} &mdash; {data.founded.end}
          </div>
          <div className="h-px bg-[var(--c-border-light)] w-12" />
        </div>
      )}
      {data.description && (
        <div className="text-lg font-body text-[var(--disco-text-primary)] leading-relaxed">
          <RichContentDisplay content={data.description} />
        </div>
      )}
      {!data.description && <div className="text-zinc-600 italic font-body">No records available.</div>}
    </div>
  );
}

function EventBody({ data }: { data: any }) {
  return (
    <div className="space-y-8">
      {data.date_range && (
        <div className="flex items-center gap-4">
          <div className="h-px bg-[var(--c-border-light)] flex-1" />
          <div className="text-lg font-body italic text-[var(--disco-accent-purple)]">
            {data.date_range.start} &mdash; {data.date_range.end}
          </div>
          <div className="h-px bg-[var(--c-border-light)] w-12" />
        </div>
      )}
      {data.description && (
        <div className="text-lg font-body text-[var(--disco-text-primary)] leading-relaxed">
          <RichContentDisplay content={data.description} />
        </div>
      )}
      {!data.description && <div className="text-zinc-600 italic font-body">No account of this event survives.</div>}
    </div>
  );
}

function GeoBody({ data }: { data: any }) {
  return (
    <div className="space-y-8">
      {data.region && (
        <InfoCard label="Region" color="var(--disco-accent-teal)">
          <RichContentDisplay content={data.region} />
        </InfoCard>
      )}
      {data.description && (
        <div className="text-lg font-body text-[var(--disco-text-primary)] leading-relaxed">
          <RichContentDisplay content={data.description} />
        </div>
      )}
      {!data.description && <div className="text-zinc-600 italic font-body">This territory remains unmapped.</div>}
    </div>
  );
}

function WorkBody({ data }: { data: any }) {
  return (
    <div className="space-y-8">
      {data.summary ? (
        <div className="relative">
          <div className="absolute -top-8 -left-8 text-8xl font-serif text-[var(--c-deep)] z-0">"</div>
          <blockquote className="relative z-10 text-xl font-body text-[var(--disco-text-primary)] leading-relaxed italic">
            <RichContentDisplay content={data.summary} />
          </blockquote>
        </div>
      ) : (
        <div className="text-zinc-600 italic font-body">No summary has been recorded for this work.</div>
      )}
    </div>
  );
}

function SchoolBody({ data }: { data: any }) {
  return (
    <div className="space-y-8">
      {data.description ? (
        <div className="text-lg font-body text-[var(--disco-text-primary)] leading-relaxed border-l-2 border-[var(--disco-accent-orange)] pl-6">
          <RichContentDisplay content={data.description} />
        </div>
      ) : (
        <div className="text-zinc-600 italic font-body">The tenets of this school remain unrecorded.</div>
      )}
    </div>
  );
}
