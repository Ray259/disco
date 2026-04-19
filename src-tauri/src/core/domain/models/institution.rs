use serde::{Deserialize, Serialize};

use crate::core::domain::values::date_range::DateRange;
use crate::core::domain::values::entity_ref::EntityRef;
use crate::core::domain::values::relation::Relation;
use crate::core::domain::values::rich_content::RichContent;

/// An institution or organization.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Institution {
    pub name: String,
    pub location_ref: Option<EntityRef>,
    pub founded: Option<DateRange>,
    pub description: Option<RichContent>,
    pub founders: Vec<EntityRef>,
    pub products: Vec<RichContent>,
    pub relations: Vec<Relation>,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

impl Institution {
    pub fn new(name: String) -> Self {
        let now = chrono::Utc::now();
        Self {
            name,
            location_ref: None,
            founded: None,
            description: None,
            founders: Vec::new(),
            products: Vec::new(),
            relations: Vec::new(),
            created_at: now,
            updated_at: now,
        }
    }
}

use crate::core::domain::traits::DomainEntity;
use crate::core::domain::values::entity_ref::EntityType;

impl DomainEntity for Institution {
    fn entity_type(&self) -> EntityType {
        EntityType::Institution
    }
    fn name(&self) -> String {
        self.name.clone()
    }
    fn set_updated_at(&mut self, date: chrono::DateTime<chrono::Utc>) {
        self.updated_at = date;
    }
}
