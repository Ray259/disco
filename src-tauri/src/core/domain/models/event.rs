use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::core::domain::values::date_range::DateRange;
use crate::core::domain::values::entity_ref::EntityRef;
use crate::core::domain::values::rich_content::RichContent;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Event {
    pub id: Uuid,
    pub name: String,
    pub date_range: DateRange,
    pub description: Option<RichContent>,
    pub location_ref: Option<EntityRef>,
    pub participants: Vec<EntityRef>,
    pub causes: Vec<RichContent>,
    pub consequences: Vec<RichContent>,
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
