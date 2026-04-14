import { RichContent } from "../api";

const CONFIG_KEYS: Record<string, string> = {
  Figure: "figures",
  Institution: "institutions",
  Event: "events",
  Geo: "geos",
  Work: "works",
  SchoolOfThought: "schools",
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
          const configKey = CONFIG_KEYS[ref.entity_type] || "";
          const label = ref.display_text.replace(/^@/, "");
          return (
            <span 
              key={i} 
              className="rich-entity-ref" 
              data-entity-type={ref.entity_type}
              title={`${ref.entity_type}: ${label}`}
              onClick={(e) => { e.stopPropagation(); onEntityClick?.(configKey, label); }}
            >
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

