use serde::{Deserialize, Serialize};

use crate::core::domain::values::date_range::DateRange;
use crate::core::domain::values::entity_ref::EntityRef;
use crate::core::domain::values::relation::Relation;
use crate::core::domain::values::rich_content::RichContent;

/// A published work, book, or creation.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Work {
    pub title: String,
    pub authors: Vec<EntityRef>,
    pub publication_date: Option<DateRange>,
    pub summary: Option<RichContent>,
    pub key_ideas: Vec<RichContent>,
    pub relations: Vec<Relation>,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

impl Work {
    pub fn new(title: String) -> Self {
        let now = chrono::Utc::now();
        Self {
            title,
            authors: Vec::new(),
            publication_date: None,
            summary: None,
            key_ideas: Vec::new(),
            relations: Vec::new(),
            created_at: now,
            updated_at: now,
        }
    }
}

use crate::core::domain::traits::DomainEntity;
use crate::core::domain::values::entity_ref::EntityType;

impl DomainEntity for Work {
    fn entity_type(&self) -> EntityType {
        EntityType::Work
    }
    fn name(&self) -> String {
        self.title.clone()
    }
    fn set_updated_at(&mut self, date: chrono::DateTime<chrono::Utc>) {
        self.updated_at = date;
    }
}
