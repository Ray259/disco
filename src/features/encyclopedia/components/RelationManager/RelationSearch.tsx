import { useState, useEffect } from 'react';
import { searchEntities, SearchResult } from '../../api';

interface RelationSearchProps {
  onSelect: (entity: SearchResult) => void;
  /** Filter results to a specific entity_type */
  entityType?: string;
  /** Show "+ Create" option when no exact match. Caller handles creation via onCreate. */
  allowCreate?: boolean;
  /** Caller-owned creation callback — receives the typed name, must handle entity creation */
  onCreate?: (name: string) => void;
  /** Custom label for the create button */
  onCreateLabel?: string;
  label?: string;
  placeholder?: string;
}

export function RelationSearch({
  onSelect, entityType, allowCreate, onCreate, onCreateLabel,
  label = "Add Relation", placeholder = "Search for an entity...",
}: RelationSearchProps) {
  const [query, setQuery] = useState('');
  const [results, setResults] = useState<SearchResult[]>([]);
  const [loading, setLoading] = useState(false);

  const filtered = entityType
    ? results.filter(r => r.entity_type === entityType)
    : results;

  useEffect(() => {
    if (query.length < 2) { setResults([]); return; }

    const timer = setTimeout(async () => {
      setLoading(true);
      try {
        const data = await searchEntities(query);
        setResults(data);
      } catch (e) {
        console.error("Search failed", e);
      } finally {
        setLoading(false);
      }
    }, 300);

    return () => clearTimeout(timer);
  }, [query]);

  const exactMatch = filtered.some(r => r.name.toLowerCase() === query.trim().toLowerCase());
  const showCreate = allowCreate && onCreate && query.trim().length >= 2 && !loading && !exactMatch;

  return (
    <div className="relative">
      <label className="block text-sm font-bold uppercase mb-1">{label}</label>
      <input
        type="text"
        className="w-full p-2 border-2 border-[var(--c-border)] bg-[var(--c-dark)] focus:outline-none focus:border-[var(--disco-accent-orange)] font-mono text-[var(--disco-text-primary)]"
        placeholder={placeholder}
        value={query}
        onChange={(e) => setQuery(e.target.value)}
      />

      {loading && <div className="absolute top-10 right-2 text-xs text-[var(--c-ghost)]">Scanning...</div>}

      {(filtered.length > 0 || showCreate) && (
        <ul className="absolute z-10 w-full bg-[var(--c-panel)] border-2 border-[var(--c-border)] mt-1 max-h-48 overflow-y-auto shadow-lg">
          {filtered.map((r) => (
            <li
              key={`${r.entity_type}-${r.name}`}
              className="p-2 hover:bg-[var(--c-deep)] cursor-pointer border-b border-[var(--c-border)] last:border-0 flex justify-between items-center"
              onClick={() => { onSelect(r); setQuery(''); setResults([]); }}
            >
              <span className="font-bold">{r.name}</span>
              <span className="text-xs uppercase bg-[var(--c-deep)] px-1">{r.entity_type}</span>
            </li>
          ))}
          {showCreate && (
            <li
              className="p-2 hover:bg-[var(--c-deep)] cursor-pointer flex justify-between items-center text-[var(--disco-accent-yellow)]"
              onClick={() => { onCreate!(query.trim()); setQuery(''); setResults([]); }}
            >
              <span className="text-sm font-body italic">
                {onCreateLabel || `+ Create "${query.trim()}"`}
              </span>
              <span className="text-xs uppercase bg-[var(--c-deep)] px-1">new</span>
            </li>
          )}
        </ul>
      )}
    </div>
  );
}
