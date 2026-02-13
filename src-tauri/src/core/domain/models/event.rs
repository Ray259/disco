use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::core::domain::values::date_range::DateRange;
use crate::core::domain::values::entity_ref::EntityRef;

use crate::core::domain::values::rich_content::RichContent;
use crate::core::domain::values::relation::Relation;

/// Represents a historical occurrence, be it a battle, a discovery, or a social movement.
///
/// Events are anchored in time (`date_range`) and space (`location_ref`). They are
/// causal nodes in the graph, linking `causes` to `consequences` and involving `participants`.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Event {
    pub id: Uuid,
    pub name: String,
    /// The temporal bounds of the event.
    pub date_range: DateRange,
    pub description: Option<RichContent>,
    /// Where the event took place.
    pub location_ref: Option<EntityRef>,
    /// Figures or Institutions involved.
    pub participants: Vec<EntityRef>,
    /// What led to this event (RichText or EntityRefs).
    pub causes: Vec<RichContent>,
    /// What resulted from this event.
    pub consequences: Vec<RichContent>,
    pub relations: Vec<Relation>,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

impl Event {
    pub fn new(id: Uuid, name: String, date_range: DateRange) -> Self {
        let now = chrono::Utc::now();
        Self {
            id,
            name,
            date_range,
            description: None,
            location_ref: None,
            participants: Vec::new(),
            causes: Vec::new(),
            consequences: Vec::new(),
            relations: Vec::new(),
            created_at: now,
            updated_at: now,
        }
    }

    pub fn with_description(mut self, description: RichContent) -> Self {
        self.description = Some(description);
        self.updated_at = chrono::Utc::now();
        self
    }

    pub fn with_location(mut self, location: EntityRef) -> Self {
        self.location_ref = Some(location);
        self.updated_at = chrono::Utc::now();
        self
    }

    pub fn add_participant(mut self, participant: EntityRef) -> Self {
        self.participants.push(participant);
        self.updated_at = chrono::Utc::now();
        self
    }
}
