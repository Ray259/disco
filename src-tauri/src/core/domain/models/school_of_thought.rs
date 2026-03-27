use serde::{Deserialize, Serialize};

use crate::core::domain::values::rich_content::RichContent;
use crate::core::domain::values::relation::Relation;

/// A school of thought or philosophical movement.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SchoolOfThought {
    pub name: String,
    pub description: Option<RichContent>,
    pub relations: Vec<Relation>,
    pub sub_schools: Vec<String>,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

impl SchoolOfThought {
    pub fn new(name: String) -> Self {
        let now = chrono::Utc::now();
        Self {
            name, description: None, relations: Vec::new(),
            sub_schools: Vec::new(), created_at: now, updated_at: now,
        }
    }
}

use crate::core::domain::values::entity_ref::EntityType;
use crate::core::domain::traits::DomainEntity;

impl DomainEntity for SchoolOfThought {
    fn entity_type(&self) -> EntityType { EntityType::SchoolOfThought }
    fn name(&self) -> String { self.name.clone() }
    fn set_updated_at(&mut self, date: chrono::DateTime<chrono::Utc>) { self.updated_at = date; }
}
