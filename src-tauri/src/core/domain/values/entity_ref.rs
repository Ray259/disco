use serde::{Deserialize, Serialize};

/// Enumerates the primary ontological entity categories modeled in the system.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum EntityType {
    Figure,
    Work,
    Event,
    Geo,
    Institution,
    SchoolOfThought,
}

impl std::fmt::Display for EntityType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            EntityType::Figure => write!(f, "Figure"),
            EntityType::Work => write!(f, "Work"),
            EntityType::Event => write!(f, "Event"),
            EntityType::Geo => write!(f, "Geo"),
            EntityType::Institution => write!(f, "Institution"),
            EntityType::SchoolOfThought => write!(f, "SchoolOfThought"),
        }
    }
}

impl EntityType {
    pub fn from_str(s: &str) -> Option<Self> {
        match s {
            "Figure" | "Figures" => Some(EntityType::Figure),
            "Work" | "Works" => Some(EntityType::Work),
            "Event" | "Events" => Some(EntityType::Event),
            "Geo" | "Geos" => Some(EntityType::Geo),
            "Institution" | "Institutions" => Some(EntityType::Institution),
            "SchoolOfThought" | "SchoolsOfThought" | "School of Thought" => Some(EntityType::SchoolOfThought),
            _ => None,
        }
    }

    /// Returns the vault subdirectory name for this type.
    pub fn dir_name(&self) -> &'static str {
        match self {
            EntityType::Figure => "Figures",
            EntityType::Work => "Works",
            EntityType::Event => "Events",
            EntityType::Geo => "Geos",
            EntityType::Institution => "Institutions",
            EntityType::SchoolOfThought => "SchoolsOfThought",
        }
    }
}

/// A lightweight reference pointing to another entity, containing its classification and identifying name.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct EntityRef {
    pub entity_type: EntityType,
    pub display_text: String,
}

impl EntityRef {
    pub fn new(entity_type: EntityType, display_text: String) -> Self {
        Self { entity_type, display_text }
    }

    pub fn figure(name: String) -> Self { Self::new(EntityType::Figure, name) }
    pub fn work(name: String) -> Self { Self::new(EntityType::Work, name) }
    pub fn event(name: String) -> Self { Self::new(EntityType::Event, name) }
    pub fn geo(name: String) -> Self { Self::new(EntityType::Geo, name) }
    pub fn institution(name: String) -> Self { Self::new(EntityType::Institution, name) }
    pub fn school(name: String) -> Self { Self::new(EntityType::SchoolOfThought, name) }
}
