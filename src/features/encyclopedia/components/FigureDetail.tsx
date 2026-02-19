import { useEffect, useState } from "react";
import { getFigure, Figure } from "../api";
import { RichContentDisplay } from "./RichContentDisplay";

interface FigureDetailProps {
  id: string;
  onBack: () => void;
}

export function FigureDetail({ id, onBack }: FigureDetailProps) {
  const [figure, setFigure] = useState<Figure | null>(null);
  const [loading, setLoading] = useState(true);
  const [error, setError] = useState<string | null>(null);

  useEffect(() => {
    getFigure(id)
      .then((data) => { setFigure(data); setLoading(false); })
      .catch((err) => { setError(String(err)); setLoading(false); });
  }, [id]);

  if (loading) return <div className="p-4 text-[var(--c-ghost)]">Loading...</div>;
  if (error) return <div className="p-4 text-red-500">Error: {error}</div>;
  if (!figure) return <div className="p-4 text-[var(--c-ghost)]">Figure not found</div>;

  return (
    <div className="w-full min-h-full p-12 bg-[var(--c-dark)]/70 backdrop-blur-sm">
      <div className="flex justify-between items-center mb-12 border-b-2 border-[var(--disco-accent-teal)] pb-2">
         <div className="flex gap-4 text-mono-sm text-[var(--disco-accent-teal)]">
           <span>Encyclopedia</span>
           <span>/</span>
           <span>Figure</span>
           <span>/</span>
           <span className="text-white">{figure.id}</span>
         </div>
         <button onClick={onBack} className="md:hidden text-xs uppercase font-bold text-[var(--c-muted)]">
           [ Close ]
         </button>
      </div>

      <div className="max-w-4xl mx-auto">
        <div className="mb-12">
           <h1 className="text-7xl font-header text-white uppercase leading-[0.85] mb-6 tracking-tight">
             {figure.name}
           </h1>
           <div className="flex items-center gap-4">
              <div className="h-px bg-[var(--c-border-light)] flex-1" />
              <div className="text-lg font-body italic text-[var(--disco-accent-orange)]">
                {figure.life.start} &mdash; {figure.life.end}
              </div>
              <div className="h-px bg-[var(--c-border-light)] w-12" />
           </div>
        </div>

        <div className="grid grid-cols-1 lg:grid-cols-12 gap-16">
           <div className="lg:col-span-4 space-y-8">
              <div className="p-6 bg-[var(--c-surface)] border-l-4 border-[var(--disco-accent-yellow)]">
                 <h3 className="text-sm font-header text-[var(--c-dim)] mb-2">Primary Role</h3>
                 <div className="text-xl font-header text-white leading-none uppercase">
                   <RichContentDisplay content={figure.primary_role} />
                 </div>
              </div>

              <div className="p-6 bg-[var(--c-surface)] border-l-4 border-[var(--disco-accent-purple)]">
                 <h3 className="text-sm font-header text-[var(--c-dim)] mb-2">Origin</h3>
                 <div className="text-xl font-header text-white leading-none uppercase">
                   <RichContentDisplay content={figure.primary_location} />
                 </div>
              </div>
           </div>

           <div className="lg:col-span-8">
              {figure.defining_quote ? (
                <div className="relative mb-12">
                   <div className="absolute -top-8 -left-8 text-8xl font-serif text-[var(--c-deep)] z-0">"</div>
                   <blockquote className="relative z-10 text-2xl font-body text-[var(--disco-text-primary)] leading-relaxed italic">
                     <RichContentDisplay content={figure.defining_quote} />
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
      </div>
    </div>
  );
}
