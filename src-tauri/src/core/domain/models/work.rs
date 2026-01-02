use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::core::domain::values::date_range::DateRange;
use crate::core::domain::values::entity_ref::EntityRef;
use crate::core::domain::values::rich_content::RichContent;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Work {
    pub id: Uuid,
    pub title: String,
    pub authors: Vec<EntityRef>,
    pub publication_date: Option<DateRange>,
    pub summary: Option<RichContent>,
    pub key_ideas: Vec<RichContent>,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

impl Work {
    pub fn new(id: Uuid, title: String) -> Self {
        let now = chrono::Utc::now();
        Self {
            id,
            title,
            authors: Vec::new(),
            publication_date: None,
            summary: None,
            key_ideas: Vec::new(),
            created_at: now,
            updated_at: now,
        }
    }

    pub fn with_author(mut self, author: EntityRef) -> Self {
        self.authors.push(author);
        self.updated_at = chrono::Utc::now();
        self
    }

    pub fn with_summary(mut self, summary: RichContent) -> Self {
        self.summary = Some(summary);
        self.updated_at = chrono::Utc::now();
        self
    }
}