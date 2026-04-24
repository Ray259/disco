use serde::{Deserialize, Serialize};

use super::date_range::DateRange;
use super::entity_ref::{EntityRef, EntityType};

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
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct RichContent {
    pub segments: Vec<ContentSegment>,
}

impl RichContent {
    pub fn new() -> Self {
        Self {
            segments: Vec::new(),
        }
    }

    pub fn from_text(text: impl Into<String>) -> Self {
        Self {
            segments: vec![ContentSegment::Text(text.into())],
        }
    }

    pub fn push_text(mut self, text: impl Into<String>) -> Self {
        self.segments.push(ContentSegment::Text(text.into()));
        self
    }

    pub fn push_entity_ref(mut self, r: EntityRef) -> Self {
        self.segments.push(ContentSegment::EntityRef(r));
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
        self.segments
            .iter()
            .map(|seg| match seg {
                ContentSegment::Text(t) => t.clone(),
                ContentSegment::EntityRef(r) => r.display_text.clone(),
                ContentSegment::DateRef(d) => format!("{} - {}", d.start, d.end),
            })
            .collect::<Vec<_>>()
            .join("")
    }

    /// Serialize to markdown with Obsidian `[[Type/Name|Name]]` wiki-links.
    pub fn to_markdown(&self) -> String {
        self.segments
            .iter()
            .map(|seg| match seg {
                ContentSegment::Text(t) => t.clone(),
                ContentSegment::EntityRef(r) => {
                    format!(
                        "[[{}/{}|{}]]",
                        r.entity_type.dir_name(),
                        r.display_text,
                        r.display_text
                    )
                }
                ContentSegment::DateRef(d) => format!("{} - {}", d.start, d.end),
            })
            .collect::<Vec<_>>()
            .join("")
    }

    /// Parse markdown with `[[Type/Name|Display]]` wiki-links back into RichContent.
    pub fn from_markdown(md: &str) -> Self {
        let mut segments = Vec::new();
        let mut rest = md;

        while !rest.is_empty() {
            if let Some(start) = rest.find("[[") {
                if start > 0 {
                    segments.push(ContentSegment::Text(rest[..start].to_string()));
                }
                let after = &rest[start + 2..];
                if let Some(end) = after.find("]]") {
                    let inner = &after[..end];
                    // Try [[Type/Name|Display]] format
                    if let Some(pipe) = inner.find('|') {
                        let path = &inner[..pipe];
                        let display = &inner[pipe + 1..];
                        if let Some(slash) = path.find('/') {
                            let type_str = &path[..slash];
                            if let Some(entity_type) = EntityType::from_str(type_str) {
                                segments.push(ContentSegment::EntityRef(EntityRef::new(
                                    entity_type,
                                    display.to_string(),
                                )));
                                rest = &after[end + 2..];
                                continue;
                            }
                        }
                    }
                    // Fallback: [[Name]] — treat as plain text link
                    segments.push(ContentSegment::Text(inner.to_string()));
                    rest = &after[end + 2..];
                } else {
                    segments.push(ContentSegment::Text("[[".to_string()));
                    rest = after;
                }
            } else {
                segments.push(ContentSegment::Text(rest.to_string()));
                break;
            }
        }

        // Merge adjacent Text segments
        let mut merged = Vec::new();
        for seg in segments {
            if let ContentSegment::Text(t) = &seg {
                if let Some(ContentSegment::Text(prev)) = merged.last_mut() {
                    prev.push_str(t);
                    continue;
                }
            }
            merged.push(seg);
        }

        Self { segments: merged }
    }

    pub fn entity_refs(&self) -> Vec<&EntityRef> {
        self.segments
            .iter()
            .filter_map(|seg| match seg {
                ContentSegment::EntityRef(r) => Some(r),
                _ => None,
            })
            .collect()
    }
}

impl From<String> for RichContent {
    fn from(s: String) -> Self {
        RichContent::from_text(s)
    }
}

impl From<&str> for RichContent {
    fn from(s: &str) -> Self {
        RichContent::from_text(s)
    }
}
