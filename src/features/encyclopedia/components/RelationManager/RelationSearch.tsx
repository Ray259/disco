import { useState, useEffect } from 'react';
import { searchEntities, SearchResult } from '../../api';

interface RelationSearchProps {
  onSelect: (entity: SearchResult) => void;
  /** When set, only results of this entity_type are shown (e.g. "Geo"). Defaults to all types. */
  entityType?: string;
  label?: string;
  placeholder?: string;
}

export function RelationSearch({
  onSelect, entityType,
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

      {filtered.length > 0 && (
        <ul className="absolute z-10 w-full bg-[var(--c-panel)] border-2 border-[var(--c-border)] mt-1 max-h-48 overflow-y-auto shadow-lg">
          {filtered.map((r) => (
            <li
              key={r.id}
              className="p-2 hover:bg-[var(--c-deep)] cursor-pointer border-b border-[var(--c-border)] last:border-0 flex justify-between items-center"
              onClick={() => { onSelect(r); setQuery(''); setResults([]); }}
            >
              <span className="font-bold">{r.name}</span>
              <span className="text-xs uppercase bg-[var(--c-deep)] px-1">{r.entity_type}</span>
            </li>
          ))}
        </ul>
      )}
    </div>
  );
}
