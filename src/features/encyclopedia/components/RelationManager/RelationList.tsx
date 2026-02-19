interface RelationItem {
  targetId: string;
  targetName: string;
  targetType: string;
  role: string;
}

interface RelationListProps {
  relations: RelationItem[];
  onRemove: (index: number) => void;
  onUpdateRole: (index: number, role: string) => void;
}

export function RelationList({ relations, onRemove, onUpdateRole }: RelationListProps) {
  if (relations.length === 0) return null;

  return (
    <div className="mt-4 border-t-2 border-[var(--c-border)] pt-4">
      <h4 className="text-sm font-bold uppercase mb-2">Pending Relations</h4>
      <ul className="space-y-2">
        {relations.map((rel, idx) => (
          <li key={idx} className="flex items-center gap-2 p-2 bg-[var(--c-dark)]/50 border border-[var(--c-border)]">
            <div className="flex-1">
              <div className="flex items-center gap-2">
                <span className="font-bold">{rel.targetName}</span>
                <span className="text-xs uppercase bg-[var(--c-deep)] px-1">{rel.targetType}</span>
              </div>
              <input
                type="text"
                placeholder="Role (e.g. Founder, Father)"
                className="w-full mt-1 p-1 text-sm border-b border-[var(--c-border)] focus:outline-none focus:border-[var(--disco-accent-orange)] bg-transparent"
                value={rel.role}
                onChange={(e) => onUpdateRole(idx, e.target.value)}
              />
            </div>
            <button 
              onClick={() => onRemove(idx)}
              className="text-red-600 hover:text-red-800 font-bold px-2"
            >
              X
            </button>
          </li>
        ))}
      </ul>
    </div>
  );
}
