use serde::{Deserialize, Serialize};
use crate::core::domain::values::entity_ref::EntityRef;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum FixedRelation {
    // Figure <-> Institution
    MemberOf,
    FounderOf,
    HeadOf,
    EnemyOf,
    
    // Figure <-> Work
    AuthorOf,
    SubjectOf,
    CritiqueOf,
    
    // Figure <-> Event
    ParticipantIn,
    WitnessOf,
    Caused,
    
    // Event <-> Geo
    HappenedAt,
    
    // Institution <-> Geo
    HeadquarteredAt,
    
    // SchoolOfThought relations
    AdherentOf,
    CriticalOf,
    BranchOf,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum RelationKind {
    Custom(String), // For same-type relations (e.g. Figure <-> Figure "Mentor")
    Fixed(FixedRelation), // For cross-type relations
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Relation {
    pub target: EntityRef,
    pub kind: RelationKind,
}
