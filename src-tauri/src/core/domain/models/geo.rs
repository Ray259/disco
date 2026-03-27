use serde::{Deserialize, Serialize};

use crate::core::domain::values::rich_content::RichContent;
use crate::core::domain::values::relation::Relation;

/// A geographic location or region.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Geo {
    pub name: String,
    pub region: Option<RichContent>,
    pub description: Option<RichContent>,
    pub aliases: Vec<String>,
    pub relations: Vec<Relation>,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

impl Geo {
    pub fn new(name: String) -> Self {
        let now = chrono::Utc::now();
        Self {
            name, region: None, description: None,
            aliases: Vec::new(), relations: Vec::new(),
            created_at: now, updated_at: now,
        }
    }
}

use crate::core::domain::values::entity_ref::EntityType;
use crate::core::domain::traits::DomainEntity;

impl DomainEntity for Geo {
    fn entity_type(&self) -> EntityType { EntityType::Geo }
    fn name(&self) -> String { self.name.clone() }
    fn set_updated_at(&mut self, date: chrono::DateTime<chrono::Utc>) { self.updated_at = date; }
}
