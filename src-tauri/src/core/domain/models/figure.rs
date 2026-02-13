use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use uuid::Uuid;

use crate::core::domain::values::date_range::DateRange;
use crate::core::domain::values::entity_ref::EntityRef;
use crate::core::domain::values::rich_content::RichContent;
use crate::core::domain::values::zeitgeist::Zeitgeist;
use crate::core::domain::values::relation::Relation;

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

/// Represents a significant individual in the encyclopedia.
///
/// Figures are the central nodes of the graph, connecting to Institutions, Events, Works,
/// and other Figures. The model captures not just biographical data but "role-playing"
/// attributes like `Zeitgeist` (their historical spirit), `Axiom` (core belief), and
/// `Critical Flaw`.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Figure {
    pub id: Uuid,

    // Profile & Metadata
    /// The display name of the figure.
    pub name: String,
    /// The birth and death dates (or active period).
    pub life: DateRange,
    /// A rich text description of their primary historical function (e.g., "Revolutionary", "Physicist").
    pub primary_role: RichContent,
    /// Where they were primarily based or born.
    pub primary_location: RichContent,
    /// A short, defining quote that encapsulates their character.
    pub defining_quote: Option<RichContent>,

    // Zeitgeist
    /// The "Spirit of the Age" they represent.
    pub zeitgeist: Option<Zeitgeist>,

    // Core Ideology
    /// The fundamental truth or rule they lived by.
    pub axiom: Option<RichContent>,
    /// Specific vocabulary or terms they coined or popularized.
    pub key_terminology: HashMap<String, RichContent>,
    /// A summary of their main logical or rhetorical progression.
    pub argument_flow: Option<RichContent>,

    // Institutional Power Base
    /// The main organization they are associated with.
    pub primary_institution: Option<EntityRef>,
    /// How they sustained their work (e.g., "Royal Patronage", "Crowdfunding").
    pub funding_model: Option<RichContent>,
    /// What they produced within that institution.
    pub institutional_product: Option<RichContent>,
    /// Who took over after them.
    pub succession_plan: Option<RichContent>,

    // Timeline
    /// Key moments or outputs in their life.
    pub major_contributions: Vec<MajorContribution>,

    // Intellectual Lineage
    /// Who influenced them.
    pub predecessors: Vec<EntityRef>,
    /// Who they argued with or competed against.
    pub contemporary_rivals: Vec<EntityRef>,
    /// Who carried on their work.
    pub successors: Vec<EntityRef>,

    // Legacy
    /// Immediate impact.
    pub short_term_success: Option<RichContent>,
    /// How they are viewed today.
    pub modern_relevance: Option<RichContent>,
    /// The failing that eventually undid them or limited their scope.
    pub critical_flaw: Option<RichContent>,
    /// Their own final view on their life's work.
    pub personal_synthesis: Option<RichContent>,

    /// Generic graph edges to other entities.
    pub relations: Vec<Relation>,

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
            relations: Vec::new(),
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