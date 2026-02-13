import { useState, useEffect } from 'react';
import { searchEntities, SearchResult } from '../../api';

interface RelationSearchProps {
  onSelect: (entity: SearchResult) => void;
}

export function RelationSearch({ onSelect }: RelationSearchProps) {
  const [query, setQuery] = useState('');
  const [results, setResults] = useState<SearchResult[]>([]);
  const [loading, setLoading] = useState(false);

  useEffect(() => {
    if (query.length < 2) {
      setResults([]);
      return;
    }

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
    }, 300); // Debounce

    return () => clearTimeout(timer);
  }, [query]);

  return (
    <div className="relative">
      <label className="block text-sm font-bold uppercase mb-1">Add Relation</label>
      <input
        type="text"
        className="w-full p-2 border-2 border-gray-800 bg-white focus:outline-none focus:border-orange-500 font-mono"
        placeholder="Search for an entity..."
        value={query}
        onChange={(e) => setQuery(e.target.value)}
      />
      
      {loading && <div className="absolute top-10 right-2 text-xs">Scanning...</div>}

      {results.length > 0 && (
        <ul className="absolute z-10 w-full bg-white border-2 border-gray-800 mt-1 max-h-48 overflow-y-auto shadow-lg">
          {results.map((r) => (
            <li 
              key={r.id}
              className="p-2 hover:bg-orange-100 cursor-pointer border-b border-gray-200 last:border-0 flex justify-between items-center"
              onClick={() => {
                onSelect(r);
                setQuery('');
                setResults([]);
              }}
            >
              <span className="font-bold">{r.name}</span>
              <span className="text-xs uppercase bg-gray-200 px-1">{r.entity_type}</span>
            </li>
          ))}
        </ul>
      )}
    </div>
  );
}
