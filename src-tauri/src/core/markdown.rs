use std::collections::HashMap;
use uuid::Uuid;
use chrono::{DateTime, Utc};
use serde_yaml::Value as YamlValue;

use crate::core::domain::models::figure::Figure;
use crate::core::domain::models::event::Event;
use crate::core::domain::models::institution::Institution;
use crate::core::domain::models::work::Work;
use crate::core::domain::models::geo::Geo;
use crate::core::domain::models::school_of_thought::SchoolOfThought;
use crate::core::domain::values::entity_ref::{EntityType, EntityRef};
use crate::core::domain::values::rich_content::RichContent;
use crate::core::domain::values::date_range::DateRange;
use crate::core::domain::values::relation::Relation;
use crate::core::domain::traits::DomainEntity;

// ─── Constants ───────────────────────────────────────────────────────────────

const FRONTMATTER_DELIMITER: &str = "---";

// ─── Public API ──────────────────────────────────────────────────────────────

/// Serializes any DomainEntity into a Markdown string with YAML frontmatter.
pub fn entity_to_markdown<E: DomainEntity + MarkdownSerializable>(entity: &E) -> String {
    let frontmatter = entity.to_frontmatter();
    let body = entity.to_body();
    
    let yaml = serde_yaml::to_string(&frontmatter).unwrap_or_default();
    
    let mut md = String::new();
    md.push_str(FRONTMATTER_DELIMITER);
    md.push('\n');
    md.push_str(&yaml);
    md.push_str(FRONTMATTER_DELIMITER);
    md.push('\n');
    if !body.is_empty() {
        md.push('\n');
        md.push_str(&body);
        md.push('\n');
    }
    md
}

/// Parses a Markdown file's content and returns (frontmatter HashMap, body String).
pub fn parse_markdown(content: &str) -> Result<(HashMap<String, YamlValue>, String), String> {
    let content = content.trim();
    if !content.starts_with(FRONTMATTER_DELIMITER) {
        return Err("Missing frontmatter delimiter".into());
    }
    
    let after_first = &content[FRONTMATTER_DELIMITER.len()..];
    let second_pos = after_first.find(FRONTMATTER_DELIMITER)
        .ok_or("Missing closing frontmatter delimiter")?;
    
    let yaml_str = &after_first[..second_pos];
    let body = after_first[second_pos + FRONTMATTER_DELIMITER.len()..].trim().to_string();
    
    let frontmatter: HashMap<String, YamlValue> = serde_yaml::from_str(yaml_str)
        .map_err(|e| format!("Failed to parse YAML frontmatter: {}", e))?;
    
    Ok((frontmatter, body))
}

/// Determines the EntityType from a parsed frontmatter.
pub fn entity_type_from_frontmatter(fm: &HashMap<String, YamlValue>) -> Result<EntityType, String> {
    let type_str = fm.get("entity_type")
        .and_then(|v| v.as_str())
        .ok_or("Missing entity_type in frontmatter")?;
    
    match type_str {
        "Figure" => Ok(EntityType::Figure),
        "Work" => Ok(EntityType::Work),
        "Event" => Ok(EntityType::Event),
        "Geo" => Ok(EntityType::Geo),
        "Institution" => Ok(EntityType::Institution),
        "SchoolOfThought" | "School of Thought" => Ok(EntityType::SchoolOfThought),
        other => Err(format!("Unknown entity_type: {}", other)),
    }
}

/// Deserializes a markdown file into a concrete domain entity, dispatching by type.
/// Returns the JSON-serialized form + metadata for SQLite insertion.
pub fn markdown_to_entity_data(content: &str) -> Result<ParsedEntity, String> {
    let (fm, body) = parse_markdown(content)?;
    let entity_type = entity_type_from_frontmatter(&fm)?;
    
    match entity_type {
        EntityType::Figure => {
            let entity = figure_from_frontmatter(&fm, &body)?;
            let data = serde_json::to_string(&entity).map_err(|e| e.to_string())?;
            Ok(ParsedEntity { id: entity.id, entity_type, name: entity.name.clone(), data })
        }
        EntityType::Event => {
            let entity = event_from_frontmatter(&fm, &body)?;
            let data = serde_json::to_string(&entity).map_err(|e| e.to_string())?;
            Ok(ParsedEntity { id: entity.id, entity_type, name: entity.name.clone(), data })
        }
        EntityType::Institution => {
            let entity = institution_from_frontmatter(&fm, &body)?;
            let data = serde_json::to_string(&entity).map_err(|e| e.to_string())?;
            Ok(ParsedEntity { id: entity.id, entity_type, name: entity.name.clone(), data })
        }
        EntityType::Work => {
            let entity = work_from_frontmatter(&fm, &body)?;
            let data = serde_json::to_string(&entity).map_err(|e| e.to_string())?;
            Ok(ParsedEntity { id: entity.id, entity_type, name: entity.title.clone(), data })
        }
        EntityType::Geo => {
            let entity = geo_from_frontmatter(&fm, &body)?;
            let data = serde_json::to_string(&entity).map_err(|e| e.to_string())?;
            Ok(ParsedEntity { id: entity.id, entity_type, name: entity.name.clone(), data })
        }
        EntityType::SchoolOfThought => {
            let entity = school_from_frontmatter(&fm, &body)?;
            let data = serde_json::to_string(&entity).map_err(|e| e.to_string())?;
            Ok(ParsedEntity { id: entity.id, entity_type, name: entity.name.clone(), data })
        }
    }
}

/// Intermediate struct holding parsed entity data ready for SQLite insertion.
pub struct ParsedEntity {
    pub id: Uuid,
    pub entity_type: EntityType,
    pub name: String,
    pub data: String, // JSON-serialized domain entity
}

// ─── Trait for markdown serialization ────────────────────────────────────────

pub trait MarkdownSerializable {
    fn to_frontmatter(&self) -> HashMap<String, YamlValue>;
    fn to_body(&self) -> String;
}

// ─── Helper functions ────────────────────────────────────────────────────────

fn yaml_str(s: &str) -> YamlValue {
    YamlValue::String(s.to_string())
}


fn rich_to_plain(rc: &RichContent) -> String {
    rc.to_plain_text()
}


fn relations_to_yaml(relations: &[Relation]) -> YamlValue {
    let items: Vec<YamlValue> = relations.iter().map(|r| {
        let mut map = serde_yaml::Mapping::new();
        map.insert(
            YamlValue::String("target_id".into()),
            YamlValue::String(r.target.entity_id.to_string()),
        );
        map.insert(
            YamlValue::String("target_type".into()),
            YamlValue::String(format!("{}", r.target.entity_type)),
        );
        map.insert(
            YamlValue::String("display_name".into()),
            YamlValue::String(r.target.display_text.clone()),
        );
        let kind_str = match &r.kind {
            crate::core::domain::values::relation::RelationKind::Custom(s) => s.clone(),
            crate::core::domain::values::relation::RelationKind::Fixed(f) => format!("{:?}", f),
        };
        map.insert(
            YamlValue::String("relation_type".into()),
            YamlValue::String(kind_str),
        );
        YamlValue::Mapping(map)
    }).collect();
    YamlValue::Sequence(items)
}

fn entity_refs_to_yaml(refs: &[EntityRef]) -> YamlValue {
    let items: Vec<YamlValue> = refs.iter().map(|r| {
        let mut map = serde_yaml::Mapping::new();
        map.insert(YamlValue::String("id".into()), YamlValue::String(r.entity_id.to_string()));
        map.insert(YamlValue::String("type".into()), YamlValue::String(format!("{}", r.entity_type)));
        map.insert(YamlValue::String("name".into()), YamlValue::String(r.display_text.clone()));
        YamlValue::Mapping(map)
    }).collect();
    YamlValue::Sequence(items)
}

fn date_range_to_str(dr: &DateRange) -> String {
    format!("{} / {}", dr.start, dr.end)
}

fn str_to_uuid(s: &str) -> Result<Uuid, String> {
    Uuid::parse_str(s).map_err(|e| format!("Invalid UUID '{}': {}", s, e))
}

fn yaml_get_str<'a>(fm: &'a HashMap<String, YamlValue>, key: &str) -> Result<&'a str, String> {
    fm.get(key)
        .and_then(|v| v.as_str())
        .ok_or(format!("Missing or invalid '{}' in frontmatter", key))
}

fn yaml_get_opt_str(fm: &HashMap<String, YamlValue>, key: &str) -> Option<String> {
    fm.get(key).and_then(|v| v.as_str()).map(|s| s.to_string())
}

fn yaml_get_datetime(fm: &HashMap<String, YamlValue>, key: &str) -> Result<DateTime<Utc>, String> {
    let s = yaml_get_str(fm, key)?;
    s.parse::<DateTime<Utc>>().map_err(|e| format!("Invalid datetime '{}': {}", s, e))
}

fn parse_date_range_str(s: &str) -> Result<DateRange, String> {
    let parts: Vec<&str> = s.split(" / ").collect();
    if parts.len() != 2 {
        return Err(format!("Invalid date range format: '{}'", s));
    }
    let start = chrono::NaiveDate::parse_from_str(parts[0].trim(), "%Y-%m-%d")
        .map_err(|e| format!("Invalid start date '{}': {}", parts[0], e))?;
    let end = chrono::NaiveDate::parse_from_str(parts[1].trim(), "%Y-%m-%d")
        .map_err(|e| format!("Invalid end date '{}': {}", parts[1], e))?;
    Ok(DateRange::new(start, end))
}

fn parse_entity_refs_yaml(fm: &HashMap<String, YamlValue>, key: &str) -> Vec<EntityRef> {
    let seq = match fm.get(key).and_then(|v| v.as_sequence()) {
        Some(s) => s,
        None => return Vec::new(),
    };
    seq.iter().filter_map(|item| {
        let map = item.as_mapping()?;
        let id_str = map.get(&YamlValue::String("id".into()))?.as_str()?;
        let id = Uuid::parse_str(id_str).ok()?;
        let type_str = map.get(&YamlValue::String("type".into()))?.as_str()?;
        let entity_type = match type_str {
            "Figure" => EntityType::Figure,
            "Work" => EntityType::Work,
            "Event" => EntityType::Event,
            "Geo" => EntityType::Geo,
            "Institution" => EntityType::Institution,
            "SchoolOfThought" | "School of Thought" => EntityType::SchoolOfThought,
            _ => return None,
        };
        let name = map.get(&YamlValue::String("name".into()))?.as_str()?.to_string();
        Some(EntityRef::new(entity_type, id, name))
    }).collect()
}

fn parse_opt_entity_ref_yaml(fm: &HashMap<String, YamlValue>, key: &str) -> Option<EntityRef> {
    let refs = parse_entity_refs_yaml(fm, key);
    refs.into_iter().next()
}

fn parse_relations_yaml(fm: &HashMap<String, YamlValue>) -> Vec<Relation> {
    let seq = match fm.get("relations").and_then(|v| v.as_sequence()) {
        Some(s) => s,
        None => return Vec::new(),
    };
    seq.iter().filter_map(|item| {
        let map = item.as_mapping()?;
        let id_str = map.get(&YamlValue::String("target_id".into()))?.as_str()?;
        let id = Uuid::parse_str(id_str).ok()?;
        let type_str = map.get(&YamlValue::String("target_type".into()))?.as_str()?;
        let entity_type = match type_str {
            "Figure" => EntityType::Figure,
            "Work" => EntityType::Work,
            "Event" => EntityType::Event,
            "Geo" => EntityType::Geo,
            "Institution" => EntityType::Institution,
            "SchoolOfThought" | "School of Thought" => EntityType::SchoolOfThought,
            _ => return None,
        };
        let display_name = map.get(&YamlValue::String("display_name".into()))?.as_str()?.to_string();
        let kind_str = map.get(&YamlValue::String("relation_type".into()))?.as_str()?;

        use crate::core::domain::values::relation::{RelationKind, FixedRelation};
        let kind = match kind_str {
            "MemberOf" => RelationKind::Fixed(FixedRelation::MemberOf),
            "FounderOf" => RelationKind::Fixed(FixedRelation::FounderOf),
            "HeadOf" => RelationKind::Fixed(FixedRelation::HeadOf),
            "EnemyOf" => RelationKind::Fixed(FixedRelation::EnemyOf),
            "AuthorOf" => RelationKind::Fixed(FixedRelation::AuthorOf),
            "SubjectOf" => RelationKind::Fixed(FixedRelation::SubjectOf),
            "CritiqueOf" => RelationKind::Fixed(FixedRelation::CritiqueOf),
            "ParticipantIn" => RelationKind::Fixed(FixedRelation::ParticipantIn),
            "WitnessOf" => RelationKind::Fixed(FixedRelation::WitnessOf),
            "Caused" => RelationKind::Fixed(FixedRelation::Caused),
            "HappenedAt" => RelationKind::Fixed(FixedRelation::HappenedAt),
            "HeadquarteredAt" => RelationKind::Fixed(FixedRelation::HeadquarteredAt),
            "AdherentOf" => RelationKind::Fixed(FixedRelation::AdherentOf),
            "CriticalOf" => RelationKind::Fixed(FixedRelation::CriticalOf),
            "BranchOf" => RelationKind::Fixed(FixedRelation::BranchOf),
            custom => RelationKind::Custom(custom.to_string()),
        };
        
        let target = EntityRef::new(entity_type, id, display_name);
        Some(Relation { target, kind })
    }).collect()
}

fn parse_string_list_yaml(fm: &HashMap<String, YamlValue>, key: &str) -> Vec<String> {
    fm.get(key)
        .and_then(|v| v.as_sequence())
        .map(|seq| {
            seq.iter().filter_map(|v| v.as_str().map(|s| s.to_string())).collect()
        })
        .unwrap_or_default()
}

// ─── Figure ──────────────────────────────────────────────────────────────────

impl MarkdownSerializable for Figure {
    fn to_frontmatter(&self) -> HashMap<String, YamlValue> {
        let mut fm = HashMap::new();
        fm.insert("id".into(), yaml_str(&self.id.to_string()));
        fm.insert("entity_type".into(), yaml_str("Figure"));
        fm.insert("name".into(), yaml_str(&self.name));
        fm.insert("life".into(), yaml_str(&date_range_to_str(&self.life)));
        fm.insert("primary_role".into(), yaml_str(&rich_to_plain(&self.primary_role)));
        fm.insert("primary_location".into(), yaml_str(&rich_to_plain(&self.primary_location)));
        
        if let Some(q) = &self.defining_quote {
            fm.insert("defining_quote".into(), yaml_str(&rich_to_plain(q)));
        }
        
        if !self.predecessors.is_empty() {
            fm.insert("predecessors".into(), entity_refs_to_yaml(&self.predecessors));
        }
        if !self.contemporary_rivals.is_empty() {
            fm.insert("contemporary_rivals".into(), entity_refs_to_yaml(&self.contemporary_rivals));
        }
        if !self.successors.is_empty() {
            fm.insert("successors".into(), entity_refs_to_yaml(&self.successors));
        }
        if !self.relations.is_empty() {
            fm.insert("relations".into(), relations_to_yaml(&self.relations));
        }
        
        fm.insert("created_at".into(), yaml_str(&self.created_at.to_rfc3339()));
        fm.insert("updated_at".into(), yaml_str(&self.updated_at.to_rfc3339()));
        fm
    }
    
    fn to_body(&self) -> String {
        let mut sections = Vec::new();
        
        if let Some(axiom) = &self.axiom {
            sections.push(format!("## Axiom\n\n{}", rich_to_plain(axiom)));
        }
        if let Some(flow) = &self.argument_flow {
            sections.push(format!("## Argument Flow\n\n{}", rich_to_plain(flow)));
        }
        if let Some(fund) = &self.funding_model {
            sections.push(format!("## Funding Model\n\n{}", rich_to_plain(fund)));
        }
        if let Some(prod) = &self.institutional_product {
            sections.push(format!("## Institutional Product\n\n{}", rich_to_plain(prod)));
        }
        if let Some(succ) = &self.succession_plan {
            sections.push(format!("## Succession Plan\n\n{}", rich_to_plain(succ)));
        }
        if let Some(short) = &self.short_term_success {
            sections.push(format!("## Short Term Success\n\n{}", rich_to_plain(short)));
        }
        if let Some(modern) = &self.modern_relevance {
            sections.push(format!("## Modern Relevance\n\n{}", rich_to_plain(modern)));
        }
        if let Some(flaw) = &self.critical_flaw {
            sections.push(format!("## Critical Flaw\n\n{}", rich_to_plain(flaw)));
        }
        if let Some(synth) = &self.personal_synthesis {
            sections.push(format!("## Personal Synthesis\n\n{}", rich_to_plain(synth)));
        }
        
        sections.join("\n\n")
    }
}

fn figure_from_frontmatter(fm: &HashMap<String, YamlValue>, body: &str) -> Result<Figure, String> {
    let id = str_to_uuid(yaml_get_str(fm, "id")?)?;
    let name = yaml_get_str(fm, "name")?.to_string();
    let life = parse_date_range_str(yaml_get_str(fm, "life")?)?;
    let primary_role = RichContent::from_text(yaml_get_str(fm, "primary_role")?);
    let primary_location = RichContent::from_text(yaml_get_str(fm, "primary_location")?);
    
    let mut figure = Figure::new(id, name, life, primary_role, primary_location);
    
    if let Some(q) = yaml_get_opt_str(fm, "defining_quote") {
        figure.defining_quote = Some(RichContent::from_text(&q));
    }
    
    figure.predecessors = parse_entity_refs_yaml(fm, "predecessors");
    figure.contemporary_rivals = parse_entity_refs_yaml(fm, "contemporary_rivals");
    figure.successors = parse_entity_refs_yaml(fm, "successors");
    figure.relations = parse_relations_yaml(fm);
    
    // Parse body sections
    let sections = parse_body_sections(body);
    for (heading, content) in &sections {
        match heading.as_str() {
            "Axiom" => figure.axiom = Some(RichContent::from_text(content)),
            "Argument Flow" => figure.argument_flow = Some(RichContent::from_text(content)),
            "Funding Model" => figure.funding_model = Some(RichContent::from_text(content)),
            "Institutional Product" => figure.institutional_product = Some(RichContent::from_text(content)),
            "Succession Plan" => figure.succession_plan = Some(RichContent::from_text(content)),
            "Short Term Success" => figure.short_term_success = Some(RichContent::from_text(content)),
            "Modern Relevance" => figure.modern_relevance = Some(RichContent::from_text(content)),
            "Critical Flaw" => figure.critical_flaw = Some(RichContent::from_text(content)),
            "Personal Synthesis" => figure.personal_synthesis = Some(RichContent::from_text(content)),
            _ => {}
        }
    }
    
    figure.created_at = yaml_get_datetime(fm, "created_at").unwrap_or_else(|_| Utc::now());
    figure.updated_at = yaml_get_datetime(fm, "updated_at").unwrap_or_else(|_| Utc::now());
    
    Ok(figure)
}

// ─── Event ───────────────────────────────────────────────────────────────────

impl MarkdownSerializable for Event {
    fn to_frontmatter(&self) -> HashMap<String, YamlValue> {
        let mut fm = HashMap::new();
        fm.insert("id".into(), yaml_str(&self.id.to_string()));
        fm.insert("entity_type".into(), yaml_str("Event"));
        fm.insert("name".into(), yaml_str(&self.name));
        fm.insert("date_range".into(), yaml_str(&date_range_to_str(&self.date_range)));
        
        if let Some(loc) = &self.location_ref {
            fm.insert("location_ref".into(), entity_refs_to_yaml(&[loc.clone()]));
        }
        if !self.participants.is_empty() {
            fm.insert("participants".into(), entity_refs_to_yaml(&self.participants));
        }
        if !self.relations.is_empty() {
            fm.insert("relations".into(), relations_to_yaml(&self.relations));
        }
        
        fm.insert("created_at".into(), yaml_str(&self.created_at.to_rfc3339()));
        fm.insert("updated_at".into(), yaml_str(&self.updated_at.to_rfc3339()));
        fm
    }
    
    fn to_body(&self) -> String {
        let mut sections = Vec::new();
        if let Some(desc) = &self.description {
            sections.push(format!("## Description\n\n{}", rich_to_plain(desc)));
        }
        if !self.causes.is_empty() {
            let items: Vec<String> = self.causes.iter().map(|c| format!("- {}", rich_to_plain(c))).collect();
            sections.push(format!("## Causes\n\n{}", items.join("\n")));
        }
        if !self.consequences.is_empty() {
            let items: Vec<String> = self.consequences.iter().map(|c| format!("- {}", rich_to_plain(c))).collect();
            sections.push(format!("## Consequences\n\n{}", items.join("\n")));
        }
        sections.join("\n\n")
    }
}

fn event_from_frontmatter(fm: &HashMap<String, YamlValue>, body: &str) -> Result<Event, String> {
    let id = str_to_uuid(yaml_get_str(fm, "id")?)?;
    let name = yaml_get_str(fm, "name")?.to_string();
    let date_range = parse_date_range_str(yaml_get_str(fm, "date_range")?)?;
    
    let mut event = Event::new(id, name, date_range);
    event.location_ref = parse_opt_entity_ref_yaml(fm, "location_ref");
    event.participants = parse_entity_refs_yaml(fm, "participants");
    event.relations = parse_relations_yaml(fm);
    
    let sections = parse_body_sections(body);
    for (heading, content) in &sections {
        match heading.as_str() {
            "Description" => event.description = Some(RichContent::from_text(content)),
            "Causes" => {
                event.causes = parse_list_items(content).iter()
                    .map(|s| RichContent::from_text(s)).collect();
            }
            "Consequences" => {
                event.consequences = parse_list_items(content).iter()
                    .map(|s| RichContent::from_text(s)).collect();
            }
            _ => {}
        }
    }
    
    event.created_at = yaml_get_datetime(fm, "created_at").unwrap_or_else(|_| Utc::now());
    event.updated_at = yaml_get_datetime(fm, "updated_at").unwrap_or_else(|_| Utc::now());
    
    Ok(event)
}

// ─── Institution ─────────────────────────────────────────────────────────────

impl MarkdownSerializable for Institution {
    fn to_frontmatter(&self) -> HashMap<String, YamlValue> {
        let mut fm = HashMap::new();
        fm.insert("id".into(), yaml_str(&self.id.to_string()));
        fm.insert("entity_type".into(), yaml_str("Institution"));
        fm.insert("name".into(), yaml_str(&self.name));
        
        if let Some(loc) = &self.location_ref {
            fm.insert("location_ref".into(), entity_refs_to_yaml(&[loc.clone()]));
        }
        if let Some(founded) = &self.founded {
            fm.insert("founded".into(), yaml_str(&date_range_to_str(founded)));
        }
        if !self.founders.is_empty() {
            fm.insert("founders".into(), entity_refs_to_yaml(&self.founders));
        }
        if !self.relations.is_empty() {
            fm.insert("relations".into(), relations_to_yaml(&self.relations));
        }
        
        fm.insert("created_at".into(), yaml_str(&self.created_at.to_rfc3339()));
        fm.insert("updated_at".into(), yaml_str(&self.updated_at.to_rfc3339()));
        fm
    }
    
    fn to_body(&self) -> String {
        let mut sections = Vec::new();
        if let Some(desc) = &self.description {
            sections.push(format!("## Description\n\n{}", rich_to_plain(desc)));
        }
        if !self.products.is_empty() {
            let items: Vec<String> = self.products.iter().map(|p| format!("- {}", rich_to_plain(p))).collect();
            sections.push(format!("## Products\n\n{}", items.join("\n")));
        }
        sections.join("\n\n")
    }
}

fn institution_from_frontmatter(fm: &HashMap<String, YamlValue>, body: &str) -> Result<Institution, String> {
    let id = str_to_uuid(yaml_get_str(fm, "id")?)?;
    let name = yaml_get_str(fm, "name")?.to_string();
    
    let mut inst = Institution::new(id, name);
    inst.location_ref = parse_opt_entity_ref_yaml(fm, "location_ref");
    inst.founded = yaml_get_opt_str(fm, "founded").and_then(|s| parse_date_range_str(&s).ok());
    inst.founders = parse_entity_refs_yaml(fm, "founders");
    inst.relations = parse_relations_yaml(fm);
    
    let sections = parse_body_sections(body);
    for (heading, content) in &sections {
        match heading.as_str() {
            "Description" => inst.description = Some(RichContent::from_text(content)),
            "Products" => {
                inst.products = parse_list_items(content).iter()
                    .map(|s| RichContent::from_text(s)).collect();
            }
            _ => {}
        }
    }
    
    inst.created_at = yaml_get_datetime(fm, "created_at").unwrap_or_else(|_| Utc::now());
    inst.updated_at = yaml_get_datetime(fm, "updated_at").unwrap_or_else(|_| Utc::now());
    
    Ok(inst)
}

// ─── Work ────────────────────────────────────────────────────────────────────

impl MarkdownSerializable for Work {
    fn to_frontmatter(&self) -> HashMap<String, YamlValue> {
        let mut fm = HashMap::new();
        fm.insert("id".into(), yaml_str(&self.id.to_string()));
        fm.insert("entity_type".into(), yaml_str("Work"));
        fm.insert("name".into(), yaml_str(&self.title));
        
        if !self.authors.is_empty() {
            fm.insert("authors".into(), entity_refs_to_yaml(&self.authors));
        }
        if let Some(pd) = &self.publication_date {
            fm.insert("publication_date".into(), yaml_str(&date_range_to_str(pd)));
        }
        if !self.relations.is_empty() {
            fm.insert("relations".into(), relations_to_yaml(&self.relations));
        }
        
        fm.insert("created_at".into(), yaml_str(&self.created_at.to_rfc3339()));
        fm.insert("updated_at".into(), yaml_str(&self.updated_at.to_rfc3339()));
        fm
    }
    
    fn to_body(&self) -> String {
        let mut sections = Vec::new();
        if let Some(summary) = &self.summary {
            sections.push(format!("## Summary\n\n{}", rich_to_plain(summary)));
        }
        if !self.key_ideas.is_empty() {
            let items: Vec<String> = self.key_ideas.iter().map(|k| format!("- {}", rich_to_plain(k))).collect();
            sections.push(format!("## Key Ideas\n\n{}", items.join("\n")));
        }
        sections.join("\n\n")
    }
}

fn work_from_frontmatter(fm: &HashMap<String, YamlValue>, body: &str) -> Result<Work, String> {
    let id = str_to_uuid(yaml_get_str(fm, "id")?)?;
    let title = yaml_get_str(fm, "name")?.to_string();
    
    let mut work = Work::new(id, title);
    work.authors = parse_entity_refs_yaml(fm, "authors");
    work.publication_date = yaml_get_opt_str(fm, "publication_date").and_then(|s| parse_date_range_str(&s).ok());
    work.relations = parse_relations_yaml(fm);
    
    let sections = parse_body_sections(body);
    for (heading, content) in &sections {
        match heading.as_str() {
            "Summary" => work.summary = Some(RichContent::from_text(content)),
            "Key Ideas" => {
                work.key_ideas = parse_list_items(content).iter()
                    .map(|s| RichContent::from_text(s)).collect();
            }
            _ => {}
        }
    }
    
    work.created_at = yaml_get_datetime(fm, "created_at").unwrap_or_else(|_| Utc::now());
    work.updated_at = yaml_get_datetime(fm, "updated_at").unwrap_or_else(|_| Utc::now());
    
    Ok(work)
}

// ─── Geo ─────────────────────────────────────────────────────────────────────

impl MarkdownSerializable for Geo {
    fn to_frontmatter(&self) -> HashMap<String, YamlValue> {
        let mut fm = HashMap::new();
        fm.insert("id".into(), yaml_str(&self.id.to_string()));
        fm.insert("entity_type".into(), yaml_str("Geo"));
        fm.insert("name".into(), yaml_str(&self.name));
        
        if !self.aliases.is_empty() {
            let items: Vec<YamlValue> = self.aliases.iter().map(|a| YamlValue::String(a.clone())).collect();
            fm.insert("aliases".into(), YamlValue::Sequence(items));
        }
        if !self.relations.is_empty() {
            fm.insert("relations".into(), relations_to_yaml(&self.relations));
        }
        
        fm.insert("created_at".into(), yaml_str(&self.created_at.to_rfc3339()));
        fm.insert("updated_at".into(), yaml_str(&self.updated_at.to_rfc3339()));
        fm
    }
    
    fn to_body(&self) -> String {
        let mut sections = Vec::new();
        if let Some(region) = &self.region {
            sections.push(format!("## Region\n\n{}", rich_to_plain(region)));
        }
        if let Some(desc) = &self.description {
            sections.push(format!("## Description\n\n{}", rich_to_plain(desc)));
        }
        sections.join("\n\n")
    }
}

fn geo_from_frontmatter(fm: &HashMap<String, YamlValue>, body: &str) -> Result<Geo, String> {
    let id = str_to_uuid(yaml_get_str(fm, "id")?)?;
    let name = yaml_get_str(fm, "name")?.to_string();
    
    let mut geo = Geo::new(id, name);
    geo.aliases = parse_string_list_yaml(fm, "aliases");
    geo.relations = parse_relations_yaml(fm);
    
    let sections = parse_body_sections(body);
    for (heading, content) in &sections {
        match heading.as_str() {
            "Region" => geo.region = Some(RichContent::from_text(content)),
            "Description" => geo.description = Some(RichContent::from_text(content)),
            _ => {}
        }
    }
    
    geo.created_at = yaml_get_datetime(fm, "created_at").unwrap_or_else(|_| Utc::now());
    geo.updated_at = yaml_get_datetime(fm, "updated_at").unwrap_or_else(|_| Utc::now());
    
    Ok(geo)
}

// ─── SchoolOfThought ─────────────────────────────────────────────────────────

impl MarkdownSerializable for SchoolOfThought {
    fn to_frontmatter(&self) -> HashMap<String, YamlValue> {
        let mut fm = HashMap::new();
        fm.insert("id".into(), yaml_str(&self.id.to_string()));
        fm.insert("entity_type".into(), yaml_str("SchoolOfThought"));
        fm.insert("name".into(), yaml_str(&self.name));
        
        if !self.sub_schools.is_empty() {
            let items: Vec<YamlValue> = self.sub_schools.iter().map(|s| YamlValue::String(s.clone())).collect();
            fm.insert("sub_schools".into(), YamlValue::Sequence(items));
        }
        if !self.relations.is_empty() {
            fm.insert("relations".into(), relations_to_yaml(&self.relations));
        }
        
        fm.insert("created_at".into(), yaml_str(&self.created_at.to_rfc3339()));
        fm.insert("updated_at".into(), yaml_str(&self.updated_at.to_rfc3339()));
        fm
    }
    
    fn to_body(&self) -> String {
        match &self.description {
            Some(desc) => format!("## Description\n\n{}", rich_to_plain(desc)),
            None => String::new(),
        }
    }
}

fn school_from_frontmatter(fm: &HashMap<String, YamlValue>, body: &str) -> Result<SchoolOfThought, String> {
    let id = str_to_uuid(yaml_get_str(fm, "id")?)?;
    let name = yaml_get_str(fm, "name")?.to_string();
    
    let mut school = SchoolOfThought::new(id, name);
    school.sub_schools = parse_string_list_yaml(fm, "sub_schools");
    school.relations = parse_relations_yaml(fm);
    
    let sections = parse_body_sections(body);
    for (heading, content) in &sections {
        if heading == "Description" {
            school.description = Some(RichContent::from_text(content));
        }
    }
    
    school.created_at = yaml_get_datetime(fm, "created_at").unwrap_or_else(|_| Utc::now());
    school.updated_at = yaml_get_datetime(fm, "updated_at").unwrap_or_else(|_| Utc::now());
    
    Ok(school)
}

// ─── Body parsing utilities ─────────────────────────────────────────────────

/// Parses markdown body into (heading, content) tuples.
/// Only matches `## Heading` (h2) level sections.
fn parse_body_sections(body: &str) -> Vec<(String, String)> {
    let mut sections = Vec::new();
    let mut current_heading: Option<String> = None;
    let mut current_content = String::new();
    
    for line in body.lines() {
        if line.starts_with("## ") {
            // Save previous section
            if let Some(heading) = current_heading.take() {
                sections.push((heading, current_content.trim().to_string()));
            }
            current_heading = Some(line[3..].trim().to_string());
            current_content = String::new();
        } else if current_heading.is_some() {
            current_content.push_str(line);
            current_content.push('\n');
        }
    }
    
    // Save last section
    if let Some(heading) = current_heading {
        sections.push((heading, current_content.trim().to_string()));
    }
    
    sections
}

/// Parses markdown list items (lines starting with `- `).
fn parse_list_items(content: &str) -> Vec<String> {
    content.lines()
        .filter_map(|line| {
            let trimmed = line.trim();
            if trimmed.starts_with("- ") {
                Some(trimmed[2..].to_string())
            } else if !trimmed.is_empty() {
                Some(trimmed.to_string())
            } else {
                None
            }
        })
        .collect()
}

// ─── File naming ─────────────────────────────────────────────────────────────

/// Generates a safe filename from an entity name.
/// Replaces characters that are invalid in filenames.
pub fn safe_filename(name: &str) -> String {
    name.chars()
        .map(|c| match c {
            '/' | '\\' | ':' | '*' | '?' | '"' | '<' | '>' | '|' => '_',
            _ => c,
        })
        .collect::<String>()
        .trim()
        .to_string()
}

/// Returns the subdirectory name for an entity type.
pub fn entity_type_dir(et: &EntityType) -> &'static str {
    match et {
        EntityType::Figure => "Figures",
        EntityType::Work => "Works",
        EntityType::Event => "Events",
        EntityType::Geo => "Geo",
        EntityType::Institution => "Institutions",
        EntityType::SchoolOfThought => "SchoolsOfThought",
    }
}
