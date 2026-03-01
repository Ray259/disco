import { RichContent } from "../api";

const ENTITY_TYPE_META: Record<string, { configKey: string; color: string }> = {
  Figure:          { configKey: "figures",       color: "var(--disco-accent-orange)" },
  Institution:     { configKey: "institutions",  color: "var(--disco-accent-yellow)" },
  Event:           { configKey: "events",        color: "var(--disco-accent-purple)" },
  Geo:             { configKey: "geos",          color: "var(--disco-accent-teal)" },
  Work:            { configKey: "works",         color: "#d4d4d8" },
  SchoolOfThought: { configKey: "schools",       color: "#ef4444" },
};

interface Props {
  content: RichContent;
  className?: string;
  onEntityClick?: (entityType: string, name: string) => void;
}

export function RichContentDisplay({ content, className = "", onEntityClick }: Props) {
  if (!content?.segments) return null;
  return (
    <span className={className}>
      {content.segments.map((seg, i) => {
        if ("Text" in seg) return <span key={i}>{seg.Text}</span>;
        if ("EntityRef" in seg) {
          const ref = seg.EntityRef;
          const meta = ENTITY_TYPE_META[ref.entity_type] || { configKey: "", color: "var(--c-muted)" };
          const label = ref.display_text.replace(/^@/, "");
          return (
            <span key={i} className="rich-entity-ref" style={{ color: meta.color }} title={`${ref.entity_type}: ${label}`}
              onClick={(e) => { e.stopPropagation(); onEntityClick?.(meta.configKey, label); }}>
              {label}
            </span>
          );
        }
        if ("DateRef" in seg) return <span key={i} className="text-gray-500 text-xs mx-1">{seg.DateRef.start}—{seg.DateRef.end}</span>;
        return null;
      })}
    </span>
  );
}

