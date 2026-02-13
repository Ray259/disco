use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::core::domain::values::date_range::DateRange;
use crate::core::domain::values::entity_ref::EntityRef;

use crate::core::domain::values::rich_content::RichContent;
use crate::core::domain::values::relation::Relation;

/// Represents an intellectual or artistic output (book, theory, painting).
///
/// Works are the primary vehicle for transmitting ideas between Figures.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Work {
    pub id: Uuid,
    pub title: String,
    /// The creators of the work.
    pub authors: Vec<EntityRef>,
    pub publication_date: Option<DateRange>,
    pub summary: Option<RichContent>,
    /// The core concepts introduced or discussed in the work.
    pub key_ideas: Vec<RichContent>,
    pub relations: Vec<Relation>,
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
            relations: Vec::new(),
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