use serde::{Deserialize, Serialize};
use uuid::Uuid;
use crate::core::domain::values::rich_content::RichContent;
use crate::core::domain::values::relation::Relation;

/// Represents a broad ideological framework or movement (e.g., "Moralism", "Communism").
///
/// Schools of Thought group Figures and Works under a shared intellectual umbrella.
/// They differ from Institutions in that they are often decentralized and abstract.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SchoolOfThought {
    pub id: Uuid,
    pub name: String,
    pub description: Option<RichContent>,
    pub relations: Vec<Relation>,
    
    /// Variations or branches within this school (e.g., "Mazovian Socio-Economics").
    pub sub_schools: Vec<String>,
    
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

impl SchoolOfThought {
    pub fn new(id: Uuid, name: String) -> Self {
        let now = chrono::Utc::now();
        Self {
            id,
            name,
            description: None,
            relations: Vec::new(),
            sub_schools: Vec::new(),
            created_at: now,
            updated_at: now,
        }
    }

    pub fn with_description(mut self, description: RichContent) -> Self {
        self.description = Some(description);
        self.updated_at = chrono::Utc::now();
        self
    }
}
