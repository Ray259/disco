import { ContentSegment } from "../api";

export interface RichContentEditorProps {
  /** Current value as an array of content segments */
  value: ContentSegment[];
  /** Called whenever the editor content changes */
  onChange: (segments: ContentSegment[]) => void;
  /** Field label displayed above the editor */
  label: string;
  /** Placeholder text when editor is empty */
  placeholder?: string;
  /** Allow multi-line input (default: false → single-line) */
  multiline?: boolean;
  /** Accent color for focus states */
  color?: string;
  /** Additional CSS class */
  className?: string;
}

export interface EntityChipData {
  entity_type: string;
  display_text: string;
}

/** Convert ContentSegment[] to plain text (for backwards compat) */
export function segmentsToPlainText(segments: ContentSegment[]): string {
  return segments
    .map((s) => {
      if ("Text" in s) return s.Text;
      if ("EntityRef" in s) return s.EntityRef.display_text;
      if ("DateRef" in s) return `${s.DateRef.start}—${s.DateRef.end}`;
      return "";
    })
    .join("");
}

/** Wrap a plain string as a single Text segment */
export function textToSegments(text: string): ContentSegment[] {
  return text ? [{ Text: text }] : [];
}

/** Extract segments from a RichContent object (handles string | RichContent | undefined) */
export function extractSegments(content: any): ContentSegment[] {
  if (!content) return [];
  if (typeof content === "string") return content ? [{ Text: content }] : [];
  if (content.segments && Array.isArray(content.segments)) {
    return content.segments;
  }
  return [];
}
