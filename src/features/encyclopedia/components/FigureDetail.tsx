import { useEffect, useState } from "react";
import { getFigure, Figure } from "../api"; // Corrected import
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
    getFigure(id) // Corrected function call
      .then((data) => {
        setFigure(data);
        setLoading(false);
      })
      .catch((err) => {
        setError(String(err)); // Handle unknown error type
        setLoading(false);
      });
  }, [id]);

  if (loading) return <div className="p-4 text-gray-400">Loading...</div>;
  if (error) return <div className="p-4 text-red-500">Error: {error}</div>;
  if (!figure) return <div className="p-4 text-gray-400">Figure not found</div>;

  return (
    <div className="w-full min-h-full p-12">
      {/* Top Navigation / Breadcrumb style */}
      <div className="flex justify-between items-center mb-12 border-b-2 border-[var(--disco-accent-teal)] pb-2">
         <div className="flex gap-4 text-xs font-[var(--font-mono)] uppercase tracking-widest text-[var(--disco-accent-teal)]">
           <span>Encyclopedia</span>
           <span>/</span>
           <span>Figure</span>
           <span>/</span>
           <span className="text-white">{figure.id}</span>
         </div>
         <button 
           onClick={onBack}
           className="md:hidden text-xs uppercase font-bold text-[#666]"
         >
           [ Close ]
         </button>
      </div>

      <div className="max-w-4xl mx-auto">
        {/* Header Section */}
        <div className="mb-12">
           <h1 className="text-7xl font-[var(--font-header)] text-white uppercase leading-[0.85] mb-6 tracking-tight">
             {figure.name}
           </h1>
           <div className="flex items-center gap-4">
              <div className="h-px bg-[#444] flex-1" />
              <div className="text-lg font-[var(--font-body)] italic text-[var(--disco-accent-orange)]">
                {figure.life.start} &mdash; {figure.life.end}
              </div>
              <div className="h-px bg-[#444] w-12" />
           </div>
        </div>

        {/* Content Section */}
        <div className="grid grid-cols-1 lg:grid-cols-12 gap-16">
           {/* Left: Attributes/Stats */}
           <div className="lg:col-span-4 space-y-8">
              <div className="p-6 bg-[#151515] border-l-4 border-[var(--disco-accent-yellow)]">
                 <h3 className="text-sm font-[var(--font-header)] text-[#888] mb-2">Primary Role</h3>
                 <div className="text-xl font-[var(--font-header)] text-white leading-none uppercase">
                   <RichContentDisplay content={figure.primary_role} />
                 </div>
              </div>

              <div className="p-6 bg-[#151515] border-l-4 border-[var(--disco-accent-purple)]">
                 <h3 className="text-sm font-[var(--font-header)] text-[#888] mb-2">Origin</h3>
                 <div className="text-xl font-[var(--font-header)] text-white leading-none uppercase">
                   <RichContentDisplay content={figure.primary_location} />
                 </div>
              </div>
           </div>

           {/* Right: Body Text/Quote */}
           <div className="lg:col-span-8">
              {figure.defining_quote ? (
                <div className="relative mb-12">
                   {/* Large quotation mark decoration */}
                   <div className="absolute -top-8 -left-8 text-8xl font-serif text-[#222] z-0">"</div>
                   <blockquote className="relative z-10 text-2xl font-[var(--font-body)] text-[var(--disco-text-primary)] leading-relaxed italic">
                     <RichContentDisplay content={figure.defining_quote} />
                   </blockquote>
                   <div className="mt-4 text-right">
                      <span className="text-xs font-[var(--font-mono)] text-[#666] tracking-widest uppercase">&mdash; RÉTROSPECTIVE</span>
                   </div>
                </div>
              ) : (
                <div className="text-zinc-600 italic font-[var(--font-body)]">No recording of speech available.</div>
              )}
           </div>
        </div>
      </div>
    </div>
  );
}
