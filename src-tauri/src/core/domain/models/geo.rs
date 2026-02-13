use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::core::domain::values::rich_content::RichContent;
use crate::core::domain::values::relation::Relation;

/// Represents a physical location, ranging from a city to a specific building.
///
/// locations provide the spatial context for Figures and Events.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Geo {
    pub id: Uuid,
    pub name: String,
    /// The broader region or political entity this location belongs to (e.g., "Revachol West").
    pub region: Option<RichContent>,
    pub description: Option<RichContent>,
    /// Alternative names or historical names.
    pub aliases: Vec<String>,
    pub relations: Vec<Relation>,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

impl Geo {
    pub fn new(id: Uuid, name: String) -> Self {
        let now = chrono::Utc::now();
        Self {
            id,
            name,
            region: None,
            description: None,
            aliases: Vec::new(),
            relations: Vec::new(),
            created_at: now,
            updated_at: now,
        }
    }

    pub fn with_region(mut self, region: RichContent) -> Self {
        self.region = Some(region);
        self.updated_at = chrono::Utc::now();
        self
    }

    pub fn with_description(mut self, description: RichContent) -> Self {
        self.description = Some(description);
        self.updated_at = chrono::Utc::now();
        self
    }
}
