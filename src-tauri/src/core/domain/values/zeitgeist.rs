use serde::{Deserialize, Serialize};

use super::entity_ref::EntityRef;
use super::rich_content::RichContent;

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Zeitgeist {
    pub era: Option<RichContent>,
    pub catalyst: Option<RichContent>,
    pub opposition: Option<RichContent>,
    pub influences: Vec<EntityRef>,
}

impl Zeitgeist {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_era(mut self, era: RichContent) -> Self {
        self.era = Some(era);
        self
    }

    pub fn with_catalyst(mut self, catalyst: RichContent) -> Self {
        self.catalyst = Some(catalyst);
        self
    }

    pub fn with_opposition(mut self, opposition: RichContent) -> Self {
        self.opposition = Some(opposition);
        self
    }

    pub fn add_influence(mut self, influence: EntityRef) -> Self {
        self.influences.push(influence);
        self
    }

    pub fn with_influences(mut self, influences: Vec<EntityRef>) -> Self {
        self.influences = influences;
        self
    }
}
