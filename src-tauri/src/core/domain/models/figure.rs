use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use crate::core::domain::values::date_range::DateRange;
use crate::core::domain::values::entity_ref::EntityRef;
use crate::core::domain::values::rich_content::RichContent;
use crate::core::domain::values::zeitgeist::Zeitgeist;
use crate::core::domain::values::relation::Relation;

/// A major contribution made by a figure.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MajorContribution {
    pub title: String,
    pub entity_ref: Option<EntityRef>,
    pub date: DateRange,
    pub impact: RichContent,
}

/// A historical figure or person.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Figure {
    pub name: String,
    pub life: DateRange,
    pub primary_role: RichContent,
    pub primary_location: RichContent,
    pub defining_quote: Option<RichContent>,
    pub zeitgeist: Option<Zeitgeist>,
    pub axiom: Option<RichContent>,
    pub key_terminology: HashMap<String, RichContent>,
    pub argument_flow: Option<RichContent>,
    pub primary_institution: Option<EntityRef>,
    pub funding_model: Option<RichContent>,
    pub institutional_product: Option<RichContent>,
    pub succession_plan: Option<RichContent>,
    pub major_contributions: Vec<MajorContribution>,
    pub predecessors: Vec<EntityRef>,
    pub contemporary_rivals: Vec<EntityRef>,
    pub successors: Vec<EntityRef>,
    pub short_term_success: Option<RichContent>,
    pub modern_relevance: Option<RichContent>,
    pub critical_flaw: Option<RichContent>,
    pub personal_synthesis: Option<RichContent>,
    pub relations: Vec<Relation>,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

impl Figure {
    pub fn new(name: String, life: DateRange, primary_role: RichContent, primary_location: RichContent) -> Self {
        let now = chrono::Utc::now();
        Self {
            name, life, primary_role, primary_location,
            defining_quote: None, zeitgeist: None, axiom: None,
            key_terminology: HashMap::new(), argument_flow: None,
            primary_institution: None, funding_model: None,
            institutional_product: None, succession_plan: None,
            major_contributions: Vec::new(),
            predecessors: Vec::new(), contemporary_rivals: Vec::new(), successors: Vec::new(),
            short_term_success: None, modern_relevance: None,
            critical_flaw: None, personal_synthesis: None,
            relations: Vec::new(),
            created_at: now, updated_at: now,
        }
    }

    pub fn with_defining_quote(mut self, quote: RichContent) -> Self {
        self.defining_quote = Some(quote); self
    }
}

use crate::core::domain::values::entity_ref::EntityType;
use crate::core::domain::traits::DomainEntity;

impl DomainEntity for Figure {
    fn entity_type(&self) -> EntityType { EntityType::Figure }
    fn name(&self) -> String { self.name.clone() }
    fn set_updated_at(&mut self, date: chrono::DateTime<chrono::Utc>) { self.updated_at = date; }
}