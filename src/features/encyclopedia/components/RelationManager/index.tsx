import { RelationSearch } from './RelationSearch';
import { RelationList } from './RelationList';

export interface PendingRelation {
  targetId: string;
  targetName: string;
  targetType: string;
  role: string; // The type of relation (e.g. "FOUNDER", "STUDENT")
}

interface RelationManagerProps {
  relations: PendingRelation[];
  onChange: (relations: PendingRelation[]) => void;
}

export function RelationManager({ relations, onChange }: RelationManagerProps) {
  const handleSelect = (entity: any) => {
    // Prevent adding duplicates
    if (relations.some(r => r.targetId === entity.id)) return;

    const newRelation: PendingRelation = {
      targetId: entity.id,
      targetName: entity.name,
      targetType: entity.entity_type,
      role: '', // User must specify
    };

    onChange([...relations, newRelation]);
  };

  const handleRemove = (index: number) => {
    const next = [...relations];
    next.splice(index, 1);
    onChange(next);
  };

  const handleUpdateRole = (index: number, role: string) => {
    const next = [...relations];
    next[index].role = role;
    onChange(next);
  };

  return (
    <div className="border-2 border-gray-200 p-4 bg-gray-50/50 mt-6">
      <RelationSearch onSelect={handleSelect} />
      <RelationList 
        relations={relations} 
        onRemove={handleRemove} 
        onUpdateRole={handleUpdateRole} 
      />
    </div>
  );
}
