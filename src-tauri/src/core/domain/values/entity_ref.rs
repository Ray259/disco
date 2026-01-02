use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum EntityType {
    Figure,
    Work,
    Event,
    Geo,
    Institution,
}

impl std::fmt::Display for EntityType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            EntityType::Figure => write!(f, "Figure"),
            EntityType::Work => write!(f, "Work"),
            EntityType::Event => write!(f, "Event"),
            EntityType::Geo => write!(f, "Geo"),
            EntityType::Institution => write!(f, "Institution"),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct EntityRef {
    pub entity_type: EntityType,
    pub entity_id: Uuid,
    pub display_text: String,
}

impl EntityRef {
    pub fn new(entity_type: EntityType, entity_id: Uuid, display_text: String) -> Self {
        Self { entity_type, entity_id, display_text }
    }

    pub fn figure(id: Uuid, display_text: String) -> Self {
        Self::new(EntityType::Figure, id, display_text)
    }

    pub fn work(id: Uuid, display_text: String) -> Self {
        Self::new(EntityType::Work, id, display_text)
    }

    pub fn event(id: Uuid, display_text: String) -> Self {
        Self::new(EntityType::Event, id, display_text)
    }

    pub fn geo(id: Uuid, display_text: String) -> Self {
        Self::new(EntityType::Geo, id, display_text)
    }

    pub fn institution(id: Uuid, display_text: String) -> Self {
        Self::new(EntityType::Institution, id, display_text)
    }
}
