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
    <div className="mt-4 border-t-2 border-gray-300 pt-4">
      <h4 className="text-sm font-bold uppercase mb-2">Pending Relations</h4>
      <ul className="space-y-2">
        {relations.map((rel, idx) => (
          <li key={idx} className="flex items-center gap-2 p-2 bg-gray-50 border border-gray-200">
            <div className="flex-1">
              <div className="flex items-center gap-2">
                <span className="font-bold">{rel.targetName}</span>
                <span className="text-xs uppercase bg-gray-200 px-1">{rel.targetType}</span>
              </div>
              <input
                type="text"
                placeholder="Role (e.g. Founder, Father)"
                className="w-full mt-1 p-1 text-sm border-b border-gray-300 focus:outline-none focus:border-orange-500 bg-transparent"
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
