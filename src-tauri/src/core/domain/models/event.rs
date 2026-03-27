use serde::{Deserialize, Serialize};

use crate::core::domain::values::date_range::DateRange;
use crate::core::domain::values::entity_ref::EntityRef;
use crate::core::domain::values::rich_content::RichContent;
use crate::core::domain::values::relation::Relation;

/// A historical event with a date range.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Event {
    pub name: String,
    pub date_range: DateRange,
    pub description: Option<RichContent>,
    pub location_ref: Option<EntityRef>,
    pub participants: Vec<EntityRef>,
    pub causes: Vec<RichContent>,
    pub consequences: Vec<RichContent>,
    pub relations: Vec<Relation>,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

impl Event {
    pub fn new(name: String, date_range: DateRange) -> Self {
        let now = chrono::Utc::now();
        Self {
            name, date_range, description: None, location_ref: None,
            participants: Vec::new(), causes: Vec::new(), consequences: Vec::new(),
            relations: Vec::new(), created_at: now, updated_at: now,
        }
    }
}

use crate::core::domain::values::entity_ref::EntityType;
use crate::core::domain::traits::DomainEntity;

impl DomainEntity for Event {
    fn entity_type(&self) -> EntityType { EntityType::Event }
    fn name(&self) -> String { self.name.clone() }
    fn set_updated_at(&mut self, date: chrono::DateTime<chrono::Utc>) { self.updated_at = date; }
}
