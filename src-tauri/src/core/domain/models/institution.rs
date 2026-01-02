use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::core::domain::values::date_range::DateRange;
use crate::core::domain::values::entity_ref::EntityRef;
use crate::core::domain::values::rich_content::RichContent;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Institution {
    pub id: Uuid,
    pub name: String,
    pub location_ref: Option<EntityRef>,
    pub founded: Option<DateRange>,
    pub description: Option<RichContent>,
    pub founders: Vec<EntityRef>,
    pub products: Vec<RichContent>,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

impl Institution {
    pub fn new(id: Uuid, name: String) -> Self {
        let now = chrono::Utc::now();
        Self {
            id,
            name,
            location_ref: None,
            founded: None,
            description: None,
            founders: Vec::new(),
            products: Vec::new(),
            created_at: now,
            updated_at: now,
        }
    }

    pub fn with_location(mut self, location: EntityRef) -> Self {
        self.location_ref = Some(location);
        self.updated_at = chrono::Utc::now();
        self
    }

    pub fn with_description(mut self, description: RichContent) -> Self {
        self.description = Some(description);
        self.updated_at = chrono::Utc::now();
        self
    }

    pub fn add_founder(mut self, founder: EntityRef) -> Self {
        self.founders.push(founder);
        self.updated_at = chrono::Utc::now();
        self
    }
}
