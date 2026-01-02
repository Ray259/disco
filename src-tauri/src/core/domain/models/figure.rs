use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use uuid::Uuid;

use crate::core::domain::values::date_range::DateRange;
use crate::core::domain::values::entity_ref::EntityRef;
use crate::core::domain::values::rich_content::RichContent;
use crate::core::domain::values::zeitgeist::Zeitgeist;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MajorContribution {
    pub title: String,
    pub entity_ref: Option<EntityRef>,
    pub date: DateRange,
    pub impact: RichContent,
}

impl MajorContribution {
    pub fn new(title: String, date: DateRange, impact: RichContent) -> Self {
        Self { title, entity_ref: None, date, impact }
    }

    pub fn with_entity_ref(mut self, entity_ref: EntityRef) -> Self {
        self.entity_ref = Some(entity_ref);
        self
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Figure {
    pub id: Uuid,

    // Profile & Metadata
    pub name: String,
    pub life: DateRange,
    pub primary_role: RichContent,
    pub primary_location: RichContent,
    pub defining_quote: Option<RichContent>,

    // Zeitgeist
    pub zeitgeist: Option<Zeitgeist>,

    // Core Ideology
    pub axiom: Option<RichContent>,
    pub key_terminology: HashMap<String, RichContent>,
    pub argument_flow: Option<RichContent>,

    // Institutional Power Base
    pub primary_institution: Option<EntityRef>,
    pub funding_model: Option<RichContent>,
    pub institutional_product: Option<RichContent>,
    pub succession_plan: Option<RichContent>,

    // Timeline
    pub major_contributions: Vec<MajorContribution>,

    // Intellectual Lineage
    pub predecessors: Vec<EntityRef>,
    pub contemporary_rivals: Vec<EntityRef>,
    pub successors: Vec<EntityRef>,

    // Legacy
    pub short_term_success: Option<RichContent>,
    pub modern_relevance: Option<RichContent>,
    pub critical_flaw: Option<RichContent>,
    pub personal_synthesis: Option<RichContent>,

    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

impl Figure {
    pub fn new(id: Uuid, name: String, life: DateRange, primary_role: RichContent, primary_location: RichContent) -> Self {
        let now = chrono::Utc::now();
        Self {
            id,
            name,
            life,
            primary_role,
            primary_location,
            defining_quote: None,
            zeitgeist: None,
            axiom: None,
            key_terminology: HashMap::new(),
            argument_flow: None,
            primary_institution: None,
            funding_model: None,
            institutional_product: None,
            succession_plan: None,
            major_contributions: Vec::new(),
            predecessors: Vec::new(),
            contemporary_rivals: Vec::new(),
            successors: Vec::new(),
            short_term_success: None,
            modern_relevance: None,
            critical_flaw: None,
            personal_synthesis: None,
            created_at: now,
            updated_at: now,
        }
    }

    pub fn with_defining_quote(mut self, quote: RichContent) -> Self {
        self.defining_quote = Some(quote);
        self.updated_at = chrono::Utc::now();
        self
    }

    pub fn with_zeitgeist(mut self, zeitgeist: Zeitgeist) -> Self {
        self.zeitgeist = Some(zeitgeist);
        self.updated_at = chrono::Utc::now();
        self
    }

    pub fn with_axiom(mut self, axiom: RichContent) -> Self {
        self.axiom = Some(axiom);
        self.updated_at = chrono::Utc::now();
        self
    }

    pub fn add_terminology(mut self, term: String, definition: RichContent) -> Self {
        self.key_terminology.insert(term, definition);
        self.updated_at = chrono::Utc::now();
        self
    }

    pub fn add_contribution(mut self, contribution: MajorContribution) -> Self {
        self.major_contributions.push(contribution);
        self.updated_at = chrono::Utc::now();
        self
    }

    pub fn add_predecessor(mut self, predecessor: EntityRef) -> Self {
        self.predecessors.push(predecessor);
        self.updated_at = chrono::Utc::now();
        self
    }

    pub fn add_rival(mut self, rival: EntityRef) -> Self {
        self.contemporary_rivals.push(rival);
        self.updated_at = chrono::Utc::now();
        self
    }

    pub fn add_successor(mut self, successor: EntityRef) -> Self {
        self.successors.push(successor);
        self.updated_at = chrono::Utc::now();
        self
    }
}