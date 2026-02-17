use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::core::domain::values::date_range::DateRange;
use crate::core::domain::values::entity_ref::EntityRef;

use crate::core::domain::values::rich_content::RichContent;
use crate::core::domain::values::relation::Relation;

/// Represents an organization, government, company, or group.
///
/// Institutions provide the structural context for Figures. They have a lifespan (`founded`),
/// a physical headquarters (`location_ref`), and are defined by their `founders` and `products`.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Institution {
    pub id: Uuid,
    pub name: String,
    /// Reference to a Geo entity representing the HQ.
    pub location_ref: Option<EntityRef>,
    /// The operational period of the institution.
    pub founded: Option<DateRange>,
    /// Rich text description of the institution's purpose.
    pub description: Option<RichContent>,
    /// References to Figures who established the institution.
    pub founders: Vec<EntityRef>,
    /// What the institution creates (laws, goods, theories).
    pub products: Vec<RichContent>,
    pub relations: Vec<Relation>,
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
            relations: Vec::new(),
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

use crate::core::domain::values::entity_ref::EntityType;
use crate::core::domain::traits::DomainEntity;

impl DomainEntity for Institution {
    fn id(&self) -> Uuid {
        self.id
    }

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
