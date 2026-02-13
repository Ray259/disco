use serde::{Deserialize, Serialize};
use uuid::Uuid;
use crate::core::domain::values::rich_content::RichContent;
use crate::core::domain::values::relation::Relation;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SchoolOfThought {
    pub id: Uuid,
    pub name: String,
    pub description: Option<RichContent>,
    pub relations: Vec<Relation>,
    
    // Unique flavor: "Ideological Stats"
    // To fit Disco Elysium, maybe "Political Alignment" or "Temperament"
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
