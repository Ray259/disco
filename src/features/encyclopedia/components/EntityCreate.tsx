import { useState } from "react";
import { 
  createFigure, 
  createInstitution, 
  createEvent, 
  createGeo, 
  createWork 
} from "../api";

interface EntityCreateProps {
  onSuccess: () => void;
  onCancel: () => void;
}

type EntityType = "Figure" | "Institution" | "Event" | "Geo" | "Work";

export function EntityCreate({ onSuccess, onCancel }: EntityCreateProps) {
  const [type, setType] = useState<EntityType>("Figure");
  const [loading, setLoading] = useState(false);
  const [error, setError] = useState<string | null>(null);

  // Common / Shared Fields
  const [name, setName] = useState("");
  const [description, setDescription] = useState(""); // Used for Institution, Event, Geo, Work(summary)
  const [startYear, setStartYear] = useState("");
  const [endYear, setEndYear] = useState("");

  // Figure Specific
  const [role, setRole] = useState("");
  const [location, setLocation] = useState("");
  const [quote, setQuote] = useState("");

  // Geo Specific
  const [region, setRegion] = useState("");

  const handleSubmit = async (e: React.FormEvent) => {
    e.preventDefault();
    setLoading(true);
    setError(null);

    try {
      if (!name) throw new Error("Designation/Name is required.");

      switch (type) {
        case "Figure":
          if (!role || !location) throw new Error("Role and Origin required.");
          await createFigure({
            name,
            role,
            location,
            start_year: startYear,
            end_year: endYear,
            quote: quote || undefined,
          });
          break;
        case "Institution":
          await createInstitution({
            name,
            founded_start: startYear || undefined,
            founded_end: endYear || undefined,
            description: description || undefined,
          });
          break;
        case "Event":
          if (!startYear || !endYear) throw new Error("Event requires temporal bounds.");
          await createEvent({
            name,
            start_date: startYear, // Assuming user types full date or just year which backend handles
            end_date: endYear,
            description: description || undefined
          });
          break;
        case "Geo":
          await createGeo({
            name,
            region: region || undefined,
            description: description || undefined
          });
          break;
        case "Work":
          await createWork({
            title: name,
            summary: description || undefined
          });
          break;
      }
      
      setLoading(false);
      onSuccess();
    } catch (err: any) {
      console.error(err);
      setError(err.message || String(err));
      setLoading(false);
    }
  };

  const getTypeColor = (t: EntityType) => {
    switch (t) {
      case "Figure": return "var(--disco-accent-orange)"; // Intellect
      case "Institution": return "var(--disco-accent-yellow)"; // Motorics
      case "Event": return "var(--disco-accent-purple)"; // Physique
      case "Geo": return "var(--disco-accent-teal)"; // Psyche
      case "Work": return "#d4d4d8"; // Neutral/White
      default: return "white";
    }
  };

  const currentColor = getTypeColor(type);

  return (
    <div className="w-full h-full p-12 flex justify-center items-start overflow-y-auto">
      <div className="w-full max-w-2xl">
        <div className="mb-12 text-center">
           <h2 className="text-3xl font-[var(--font-header)] uppercase tracking-wide mb-2" style={{ color: currentColor }}>
             New Entry
           </h2>
           <p className="font-[var(--font-body)] italic text-[#888]">
             "Cataloging the world, one fragment at a time..."
           </p>
        </div>

        {error && (
          <div className="mb-8 p-4 bg-[#2a1a1a] border-l-4 border-red-800 text-red-400 font-[var(--font-mono)] text-xs">
            [FAILURE] {error}
          </div>
        )}

        <form onSubmit={handleSubmit} className="space-y-12">
          {/* Classification Tabs */}
          <div className="flex flex-wrap gap-2 border-b border-[#333] pb-4 justify-center">
             {["Figure", "Institution", "Event", "Geo", "Work"].map((t) => (
                <button
                  key={t}
                  type="button"
                  onClick={() => setType(t as EntityType)}
                  className={`px-3 py-1 text-xs font-[var(--font-header)] uppercase tracking-wider transition-all border border-transparent ${
                    type === t 
                      ? `bg-[${getTypeColor(t as EntityType)}] text-black font-bold` 
                      : "text-[#666] hover:text-white hover:border-[#444]"
                  }`}
                  style={type === t ? { backgroundColor: getTypeColor(t as EntityType) } : {}}
                >
                  {t}
                </button>
             ))}
          </div>

          {/* Section 1: Designation */}
          <div className="group">
            <label className="block text-xs font-[var(--font-mono)] uppercase tracking-widest mb-2 transition-colors" style={{ color: currentColor }}>
              1. Designation
            </label>
            <input
              type="text"
              value={name}
              onChange={(e) => setName(e.target.value)}
              className="w-full bg-transparent border-b-2 border-[#333] py-2 text-3xl font-[var(--font-header)] text-white focus:outline-none transition-colors placeholder-[#333] uppercase"
              style={{ borderColor: "#333" }} /* Reset fallback */
              placeholder={type === "Work" ? "ENTER TITLE..." : "ENTER NAME..."}
            />
          </div>

          {/* Dynamic Sections based on Type */}

          {type === "Figure" && (
            <>
              <div className="grid grid-cols-1 md:grid-cols-2 gap-12">
                 <div>
                    <label className="block text-xs font-[var(--font-mono)] text-[#666] uppercase tracking-widest mb-2">2. Role</label>
                    <input
                      type="text"
                      value={role}
                      onChange={(e) => setRole(e.target.value)}
                      className="w-full bg-transparent border-b border-[#333] py-2 text-lg font-[var(--font-body)] text-[#ccc] focus:border-[var(--disco-accent-teal)] focus:outline-none"
                    />
                 </div>
                 <div>
                    <label className="block text-xs font-[var(--font-mono)] text-[#666] uppercase tracking-widest mb-2">3. Origin</label>
                    <input
                      type="text"
                      value={location}
                      onChange={(e) => setLocation(e.target.value)}
                      className="w-full bg-transparent border-b border-[#333] py-2 text-lg font-[var(--font-body)] text-[#ccc] focus:border-[var(--disco-accent-teal)] focus:outline-none"
                    />
                 </div>
              </div>
              <div className="group">
                <label className="block text-xs font-[var(--font-mono)] text-[#666] uppercase tracking-widest mb-2">4. Defining Utterance</label>
                <textarea
                  value={quote}
                  onChange={(e) => setQuote(e.target.value)}
                  rows={2}
                  className="w-full bg-[#181818] border border-[#333] p-4 text-xl font-[var(--font-body)] italic text-white focus:border-[var(--disco-accent-orange)] focus:outline-none placeholder-[#444]"
                  placeholder="&ldquo;...&rdquo;"
                />
              </div>
            </>
          )}

          {type === "Geo" && (
             <div className="group">
                <label className="block text-xs font-[var(--font-mono)] text-[#666] uppercase tracking-widest mb-2">2. Region / Macro-Area</label>
                <input
                  type="text"
                  value={region}
                  onChange={(e) => setRegion(e.target.value)}
                  className="w-full bg-transparent border-b border-[#333] py-2 text-lg font-[var(--font-body)] text-[#ccc] focus:border-[var(--disco-accent-teal)] focus:outline-none"
                  placeholder="e.g. Isola of Graad"
                />
             </div>
          )}

          {(type === "Institution" || type === "Event" || type === "Geo" || type === "Work") && (
             <div className="group">
               <label className="block text-xs font-[var(--font-mono)] text-[#666] uppercase tracking-widest mb-2">
                 {type === "Work" ? "2. Abstract (Summary)" : "Description / Manifest"}
               </label>
               <textarea
                 value={description}
                 onChange={(e) => setDescription(e.target.value)}
                 rows={4}
                 className="w-full bg-[#181818] border border-[#333] p-4 text-lg font-[var(--font-body)] text-white focus:outline-none placeholder-[#444]"
                 style={{ borderColor: "#333" }}
                 placeholder="Details..."
               />
             </div>
          )}

          {/* Temporal Coordinates (Not for Geo or Work usually, but maybe Work has pub date) */}
          {(type === "Figure" || type === "Institution" || type === "Event" || type === "Work") && (
            <div className="p-6 bg-[#151515] border border-[#222] relative overflow-hidden">
               <div className="absolute top-0 right-0 w-4 h-4 border-t border-r border-[#444]" />
               <h3 className="text-xs font-[var(--font-mono)] text-[#666] uppercase mb-6 text-center tracking-widest">
                 {type === "Work" ? "Publication Date" : "Temporal Coordinates"}
               </h3>
               <div className="flex items-center gap-6">
                 {type !== "Work" && (
                   <div className="flex-1">
                     <label className="block text-[10px] text-[#444] mb-1 font-[var(--font-mono)] text-center">START</label>
                     <input
                       type="text"
                       value={startYear}
                       onChange={(e) => setStartYear(e.target.value)}
                       className="w-full bg-[#0a0a0a] border border-[#333] p-3 text-center text-xl font-[var(--font-mono)] text-[var(--disco-accent-yellow)] focus:border-[var(--disco-accent-orange)] outline-none"
                       placeholder="YYYY-MM-DD"
                     />
                   </div>
                 )}
                 {type !== "Work" && <span className="text-[#333] text-xl">&mdash;</span>}
                 
                 <div className="flex-1">
                   {/* For Work, this is just 'Published' */}
                   <label className="block text-[10px] text-[#444] mb-1 font-[var(--font-mono)] text-center">
                     {type === "Work" ? "PUBLISHED" : "END"}
                   </label>
                   <input
                     type="text"
                     value={endYear} // Reuse endYear for Work pub date
                     onChange={(e) => setEndYear(e.target.value)}
                     className="w-full bg-[#0a0a0a] border border-[#333] p-3 text-center text-xl font-[var(--font-mono)] text-[var(--disco-accent-yellow)] focus:border-[var(--disco-accent-orange)] outline-none"
                     placeholder="YYYY-MM-DD"
                   />
                 </div>
               </div>
            </div>
          )}

          {/* Actions */}
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
               style={{ backgroundColor: currentColor }}
             >
                {loading ? "PROCESSING..." : "INTERNALIZE"}
             </button>
          </div>
        </form>
      </div>
    </div>
  );
}
