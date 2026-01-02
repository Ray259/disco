use serde::{Deserialize, Serialize};

use super::date_range::DateRange;
use super::entity_ref::EntityRef;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ContentSegment {
    Text(String),
    EntityRef(EntityRef),
    DateRef(DateRange),
}

impl ContentSegment {
    pub fn text(s: impl Into<String>) -> Self {
        ContentSegment::Text(s.into())
    }

    pub fn entity_ref(r: EntityRef) -> Self {
        ContentSegment::EntityRef(r)
    }

    pub fn date_ref(d: DateRange) -> Self {
        ContentSegment::DateRef(d)
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct RichContent {
    pub segments: Vec<ContentSegment>,
}

impl RichContent {
    pub fn new() -> Self {
        Self { segments: Vec::new() }
    }

    pub fn from_text(text: impl Into<String>) -> Self {
        Self { segments: vec![ContentSegment::Text(text.into())] }
    }

    pub fn push_text(mut self, text: impl Into<String>) -> Self {
        self.segments.push(ContentSegment::Text(text.into()));
        self
    }

    pub fn push_entity_ref(mut self, r: EntityRef) -> Self {
        self.segments.push(ContentSegment::EntityRef(r));
        self
    }

    pub fn push_date_ref(mut self, d: DateRange) -> Self {
        self.segments.push(ContentSegment::DateRef(d));
        self
    }

    pub fn push(mut self, segment: ContentSegment) -> Self {
        self.segments.push(segment);
        self
    }

    pub fn is_empty(&self) -> bool {
        self.segments.is_empty()
    }

    pub fn to_plain_text(&self) -> String {
        self.segments.iter().map(|seg| match seg {
            ContentSegment::Text(t) => t.clone(),
            ContentSegment::EntityRef(r) => r.display_text.clone(),
            ContentSegment::DateRef(d) => format!("{} - {}", d.start, d.end),
        }).collect::<Vec<_>>().join("")
    }

    pub fn entity_refs(&self) -> Vec<&EntityRef> {
        self.segments.iter().filter_map(|seg| match seg {
            ContentSegment::EntityRef(r) => Some(r),
            _ => None,
        }).collect()
    }
}

impl From<String> for RichContent {
    fn from(s: String) -> Self { RichContent::from_text(s) }
}

impl From<&str> for RichContent {
    fn from(s: &str) -> Self { RichContent::from_text(s) }
}
