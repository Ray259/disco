import { RichContent } from "../api";

interface RichContentDisplayProps {
  content: RichContent;
  className?: string;
}

export function RichContentDisplay({ content, className = "" }: RichContentDisplayProps) {
  if (!content || !content.segments) return null;

  return (
    <span className={className}>
      {content.segments.map((segment, index) => {
        if ("Text" in segment) {
          return <span key={index}>{segment.Text}</span>;
        }
        if ("EntityRef" in segment) {
          return (
            <span 
              key={index} 
              className="text-indigo-400"
            >
              {segment.EntityRef.display_text}
            </span>
          );
        }
        if ("DateRef" in segment) {
          return (
            <span key={index} className="text-gray-500 text-xs mx-1">
              {segment.DateRef.start}—{segment.DateRef.end}
            </span>
          );
        }
        return null;
      })}
    </span>
  );
}
