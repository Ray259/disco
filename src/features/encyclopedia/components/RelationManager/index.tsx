import { RelationSearch } from './RelationSearch';
import { RelationList } from './RelationList';

export interface PendingRelation {
  targetId: string;
  targetName: string;
  targetType: string;
  role: string;
}

interface RelationManagerProps {
  relations: PendingRelation[];
  onChange: (relations: PendingRelation[]) => void;
}

export function RelationManager({ relations, onChange }: RelationManagerProps) {
  const handleSelect = (entity: any) => {
    if (relations.some(r => r.targetId === entity.id)) return;
    onChange([...relations, {
      targetId: entity.id,
      targetName: entity.name,
      targetType: entity.entity_type,
      role: '',
    }]);
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
    <div className="border-2 border-[var(--c-border)] p-4 bg-[var(--c-dark)]/50 mt-6">
      <RelationSearch onSelect={handleSelect} />
      <RelationList 
        relations={relations} 
        onRemove={handleRemove} 
        onUpdateRole={handleUpdateRole} 
      />
    </div>
  );
}
