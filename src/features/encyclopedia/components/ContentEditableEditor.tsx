/**
 * ContentEditableEditor — contenteditable-based implementation
 *
 * Implements the RichContentEditorProps contract using a native
 * contenteditable div. Can be swapped for Slate.js / ProseMirror
 * by changing the import in RichContentEditor.tsx.
 */

import { useState, useRef, useCallback, useEffect, KeyboardEvent } from "react";
import { ContentSegment, searchEntities, SearchResult } from "../api";
import type { RichContentEditorProps } from "./RichContentEditorTypes";

// ─── Constants ──────────────────────────────────────────────────────────

const CHIP_ATTR = "data-entity-chip";
const CHIP_ID_ATTR = "data-entity-id";
const CHIP_TYPE_ATTR = "data-entity-type";

// ─── Serializer: DOM → ContentSegment[] ─────────────────────────────────

function serializeEditor(container: HTMLElement): ContentSegment[] {
  const segments: ContentSegment[] = [];
  let currentText = "";

  const flush = () => {
    if (currentText) {
      segments.push({ Text: currentText });
      currentText = "";
    }
  };

  const walk = (node: Node) => {
    if (node.nodeType === Node.TEXT_NODE) {
      currentText += node.textContent || "";
      return;
    }

    if (node.nodeType === Node.ELEMENT_NODE) {
      const el = node as HTMLElement;

      // Entity chip
      if (el.hasAttribute(CHIP_ATTR)) {
        flush();
        // Strip leading '@' — segmentsToHTML re-adds it for display
        const rawText = el.textContent || "";
        const cleanText = rawText.startsWith("@") ? rawText.slice(1) : rawText;
        segments.push({
          EntityRef: {
            entity_type: el.getAttribute(CHIP_TYPE_ATTR) || "",
            display_text: cleanText,
          },
        });
        return; // don't walk children of chip
      }

      // <br> → newline
      if (el.tagName === "BR") {
        currentText += "\n";
        return;
      }

      // Block elements (div, p) that act as line breaks
      if (el.tagName === "DIV" || el.tagName === "P") {
        // Only add newline if we already have text (avoid leading newline)
        if (currentText.length > 0 || segments.length > 0) {
          currentText += "\n";
        }
      }

      // Recurse into children
      for (const child of Array.from(el.childNodes)) {
        walk(child);
      }
    }
  };

  for (const child of Array.from(container.childNodes)) {
    walk(child);
  }
  flush();

  return segments;
}

// ─── Deserializer: ContentSegment[] → HTML string ───────────────────────

function segmentsToHTML(segments: ContentSegment[]): string {
  if (!segments || segments.length === 0) return "";

  return segments
    .map((seg) => {
      if ("Text" in seg) {
        return escapeHTML(seg.Text).replace(/\n/g, "<br>");
      }
      if ("EntityRef" in seg) {
        const r = seg.EntityRef;
        // Strip leading '@' if already present to avoid double-@@
        const label = r.display_text.startsWith("@") ? r.display_text : `@${r.display_text}`;
        return `<span class="rich-editor__chip" ${CHIP_ATTR}="true" ${CHIP_TYPE_ATTR}="${escapeAttr(r.entity_type)}" contenteditable="false">${escapeHTML(label)}</span>`;
      }
      if ("DateRef" in seg) {
        return `<span class="rich-editor__date">${escapeHTML(seg.DateRef.start)}—${escapeHTML(seg.DateRef.end)}</span>`;
      }
      return "";
    })
    .join("");
}

function escapeHTML(s: string): string {
  return s.replace(/&/g, "&amp;").replace(/</g, "&lt;").replace(/>/g, "&gt;");
}

function escapeAttr(s: string): string {
  return s.replace(/"/g, "&quot;").replace(/'/g, "&#39;");
}

// ─── Mention Popup ──────────────────────────────────────────────────────

interface MentionState {
  active: boolean;
  query: string;
  /** Pixel position for the popup */
  top: number;
  left: number;
  /** The text node & offset where the @ was typed */
  anchorNode: Node | null;
  anchorOffset: number;
}

const INITIAL_MENTION: MentionState = {
  active: false,
  query: "",
  top: 0,
  left: 0,
  anchorNode: null,
  anchorOffset: 0,
};

// ─── Component ──────────────────────────────────────────────────────────

export function ContentEditableEditor({
  value,
  onChange,
  label,
  placeholder = "",
  multiline = false,
  color,
  className = "",
}: RichContentEditorProps) {
  const editorRef = useRef<HTMLDivElement>(null);
  const [mention, setMention] = useState<MentionState>(INITIAL_MENTION);
  const [results, setResults] = useState<SearchResult[]>([]);
  const [loading, setLoading] = useState(false);
  const [selectedIdx, setSelectedIdx] = useState(0);
  const [isEmpty, setIsEmpty] = useState(true);
  const isInternalChange = useRef(false);

  // ── Deserialize on mount / external value change ───────────────────

  useEffect(() => {
    if (isInternalChange.current) {
      isInternalChange.current = false;
      return;
    }
    const el = editorRef.current;
    if (!el) return;
    const html = segmentsToHTML(value);
    if (el.innerHTML !== html) {
      el.innerHTML = html;
    }
    setIsEmpty(!value || value.length === 0 || (value.length === 1 && "Text" in value[0] && !value[0].Text));
  }, [value]);

  // ── Mention search debounce ────────────────────────────────────────

  useEffect(() => {
    if (!mention.active || mention.query.length < 2) {
      setResults([]);
      return;
    }
    const timer = setTimeout(async () => {
      setLoading(true);
      try {
        const data = await searchEntities(mention.query);
        setResults(data);
        setSelectedIdx(0);
      } catch {
        setResults([]);
      } finally {
        setLoading(false);
      }
    }, 250);
    return () => clearTimeout(timer);
  }, [mention.query, mention.active]);

  // ── Emit changes ───────────────────────────────────────────────────

  const emitChange = useCallback(() => {
    const el = editorRef.current;
    if (!el) return;
    isInternalChange.current = true;
    const segs = serializeEditor(el);
    setIsEmpty(segs.length === 0 || (segs.length === 1 && "Text" in segs[0] && !segs[0].Text));
    onChange(segs);
  }, [onChange]);

  // ── Detect @mention trigger ────────────────────────────────────────

  const handleInput = useCallback(() => {
    const sel = window.getSelection();
    if (!sel || sel.rangeCount === 0) {
      emitChange();
      return;
    }

    const range = sel.getRangeAt(0);
    const node = range.startContainer;

    if (node.nodeType === Node.TEXT_NODE) {
      const text = node.textContent || "";
      const cursor = range.startOffset;

      // Find the last @ before cursor that isn't preceded by a word char
      const before = text.slice(0, cursor);
      const atIdx = before.lastIndexOf("@");

      if (atIdx !== -1) {
        const charBefore = atIdx > 0 ? before[atIdx - 1] : " ";
        if (charBefore === " " || charBefore === "\n" || atIdx === 0 || charBefore === "\u00A0") {
          const query = before.slice(atIdx + 1);
          // If query contains a space, close mention
          if (query.includes(" ")) {
            if (mention.active) setMention(INITIAL_MENTION);
          } else {
            // Calculate popup position
            const rect = getCaretRect();
            setMention({
              active: true,
              query,
              top: rect ? rect.bottom + 4 : 0,
              left: rect ? rect.left : 0,
              anchorNode: node,
              anchorOffset: atIdx,
            });
          }
          emitChange();
          return;
        }
      }
    }

    if (mention.active) setMention(INITIAL_MENTION);
    emitChange();
  }, [emitChange, mention.active]);

  // ── Insert entity chip ─────────────────────────────────────────────

  const insertChip = useCallback(
    (entity: SearchResult) => {
      const el = editorRef.current;
      if (!el || !mention.anchorNode) return;

      const textNode = mention.anchorNode as Text;
      const text = textNode.textContent || "";
      const sel = window.getSelection();
      const cursorPos = sel?.getRangeAt(0).startOffset ?? text.length;

      // Text before the @
      const beforeAt = text.slice(0, mention.anchorOffset);
      // Text after the query
      const afterQuery = text.slice(cursorPos);

      // Create: [beforeText] [chip] [afterText]
      const beforeNode = document.createTextNode(beforeAt);
      const chip = document.createElement("span");
      chip.className = "rich-editor__chip";
      chip.setAttribute(CHIP_ATTR, "true");
      chip.setAttribute(CHIP_ID_ATTR, entity.id);
      chip.setAttribute(CHIP_TYPE_ATTR, entity.entity_type);
      chip.contentEditable = "false";
      chip.textContent = `@${entity.name}`;

      const space = document.createTextNode("\u00A0"); // non-breaking space after chip
      const afterNode = document.createTextNode(afterQuery || "");

      const parent = textNode.parentNode!;
      parent.replaceChild(afterNode, textNode);
      parent.insertBefore(space, afterNode);
      parent.insertBefore(chip, space);
      parent.insertBefore(beforeNode, chip);

      // Place cursor after the space
      const newRange = document.createRange();
      newRange.setStartAfter(space);
      newRange.collapse(true);
      sel?.removeAllRanges();
      sel?.addRange(newRange);

      setMention(INITIAL_MENTION);
      setResults([]);
      emitChange();
    },
    [mention, emitChange]
  );

  // ── Keyboard handling ──────────────────────────────────────────────

  const handleKeyDown = useCallback(
    (e: KeyboardEvent<HTMLDivElement>) => {
      // Mention popup navigation
      if (mention.active && results.length > 0) {
        if (e.key === "ArrowDown") {
          e.preventDefault();
          setSelectedIdx((i) => Math.min(i + 1, results.length - 1));
          return;
        }
        if (e.key === "ArrowUp") {
          e.preventDefault();
          setSelectedIdx((i) => Math.max(i - 1, 0));
          return;
        }
        if (e.key === "Enter" || e.key === "Tab") {
          e.preventDefault();
          insertChip(results[selectedIdx]);
          return;
        }
        if (e.key === "Escape") {
          e.preventDefault();
          setMention(INITIAL_MENTION);
          return;
        }
      }

      // Prevent Enter in single-line mode
      if (!multiline && e.key === "Enter") {
        e.preventDefault();
        return;
      }
    },
    [mention.active, results, selectedIdx, insertChip, multiline]
  );

  // ── Close mention on blur ──────────────────────────────────────────

  const handleBlur = useCallback(() => {
    // Delay to allow click on popup items
    setTimeout(() => {
      setMention(INITIAL_MENTION);
    }, 200);
  }, []);

  // ── Render ─────────────────────────────────────────────────────────

  const editorContainerRef = useRef<HTMLDivElement>(null);

  return (
    <div className={`group rich-editor-wrapper ${className}`} ref={editorContainerRef}>
      <label
        className="label-mono transition-colors"
        style={{ color: color || undefined }}
      >
        {label}
      </label>

      <div className="relative">
        <div
          ref={editorRef}
          className={`rich-editor ${multiline ? "rich-editor--multiline" : ""}`}
          contentEditable
          suppressContentEditableWarning
          onInput={handleInput}
          onKeyDown={handleKeyDown}
          onBlur={handleBlur}
          style={
            color
              ? { borderBottomColor: undefined }
              : {}
          }
          data-placeholder={placeholder}
        />

        {/* Placeholder overlay */}
        {isEmpty && (
          <div className="rich-editor__placeholder">{placeholder}</div>
        )}

        {/* @mention popup */}
        {mention.active && (results.length > 0 || loading) && (
          <MentionPopup
            results={results}
            loading={loading}
            selectedIdx={selectedIdx}
            onSelect={insertChip}
            containerRef={editorContainerRef}
            mention={mention}
          />
        )}
      </div>
    </div>
  );
}

// ─── Mention Popup Sub-component ─────────────────────────────────────────

function MentionPopup({
  results,
  loading,
  selectedIdx,
  onSelect,
  containerRef,
  mention,
}: {
  results: SearchResult[];
  loading: boolean;
  selectedIdx: number;
  onSelect: (r: SearchResult) => void;
  containerRef: React.RefObject<HTMLDivElement | null>;
  mention: MentionState;
}) {
  // Position relative to the container
  const style: React.CSSProperties = {};
  if (containerRef.current && mention.top > 0) {
    const contRect = containerRef.current.getBoundingClientRect();
    style.top = `${mention.top - contRect.top}px`;
    style.left = `${Math.max(0, mention.left - contRect.left)}px`;
  }

  return (
    <div className="rich-editor__popup" style={style}>
      {loading && (
        <div className="rich-editor__popup-loading">Scanning...</div>
      )}
      {results.map((r, i) => (
        <div
          key={r.id}
          className={`rich-editor__popup-item ${i === selectedIdx ? "rich-editor__popup-item--selected" : ""}`}
          onMouseDown={(e) => {
            e.preventDefault(); // prevent blur
            onSelect(r);
          }}
        >
          <span className="rich-editor__popup-name">{r.name}</span>
          <span className="rich-editor__popup-type">{r.entity_type}</span>
        </div>
      ))}
    </div>
  );
}

// ─── Utility ─────────────────────────────────────────────────────────────

function getCaretRect(): DOMRect | null {
  const sel = window.getSelection();
  if (!sel || sel.rangeCount === 0) return null;
  const range = sel.getRangeAt(0).cloneRange();
  range.collapse(true);

  // Insert a zero-width space to get a rect
  const span = document.createElement("span");
  span.textContent = "\u200b";
  range.insertNode(span);
  const rect = span.getBoundingClientRect();
  span.parentNode?.removeChild(span);

  // Normalize to merge adjacent text nodes
  sel.getRangeAt(0).startContainer.parentNode?.normalize();

  return rect;
}
