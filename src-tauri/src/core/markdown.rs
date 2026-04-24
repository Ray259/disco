use serde_yaml::Value as YamlValue;
use std::collections::HashMap;

use crate::core::domain::models::event::Event;
use crate::core::domain::models::figure::Figure;
use crate::core::domain::models::geo::Geo;
use crate::core::domain::models::institution::Institution;
use crate::core::domain::models::school_of_thought::SchoolOfThought;
use crate::core::domain::models::work::Work;
use crate::core::domain::traits::DomainEntity;
use crate::core::domain::values::date_range::DateRange;
use crate::core::domain::values::entity_ref::{EntityRef, EntityType};
use crate::core::domain::values::relation::{FixedRelation, Relation, RelationKind};
use crate::core::domain::values::rich_content::RichContent;

/// Standard Jekyll/Obsidian frontmatter boundary.
const FRONTMATTER_DELIMITER: &str = "---";

// ─── Public API ──────────────────────────────────────────────────────────────

/// Serializes an entity to a markdown string.
/// Partitions atomic scalar fields into YAML frontmatter and rich text into the body.
/// The entity name is used as the Markdown H1 header.
pub fn entity_to_markdown<E: DomainEntity + MarkdownSerializable>(entity: &E) -> String {
    let fm = entity.to_frontmatter();
    let body = entity.to_body();
    let yaml = serde_yaml::to_string(&fm).unwrap_or_default();
    format!(
        "{}\n{}{}\n\n# {}\n\n{}\n",
        FRONTMATTER_DELIMITER,
        yaml,
        FRONTMATTER_DELIMITER,
        entity.name(),
        body
    )
}

/// Splitter for `---` delimited blocks.
pub fn parse_markdown(content: &str) -> Result<(HashMap<String, YamlValue>, String), String> {
    let content = content.trim();
    if !content.starts_with(FRONTMATTER_DELIMITER) {
        return Err("Missing frontmatter delimiter".into());
    }
    let after = &content[FRONTMATTER_DELIMITER.len()..];
    let end = after
        .find(FRONTMATTER_DELIMITER)
        .ok_or("Missing closing frontmatter")?;
    let yaml_str = &after[..end];
    let body = after[end + FRONTMATTER_DELIMITER.len()..]
        .trim()
        .to_string();
    let fm: HashMap<String, YamlValue> =
        serde_yaml::from_str(yaml_str).map_err(|e| format!("Failed to parse YAML: {}", e))?;
    Ok((fm, body))
}

/// A parsed entity from a markdown file.
pub struct ParsedEntity {
    pub entity_type: EntityType,
    pub name: String,
    pub data: String,
}

/// High-level markdown factory.
/// Extracts identity/type from document and dispatches to specific model parsers.
pub fn markdown_to_entity_data(content: &str) -> Result<ParsedEntity, String> {
    let (fm, body) = parse_markdown(content)?;
    let et = entity_type_from_frontmatter(&fm)?;
    // Priority: Body H1 > Frontmatter "name" key
    let name = extract_title(&body)
        .or_else(|| {
            fm.get("name")
                .and_then(|v| v.as_str())
                .map(|s| s.to_string())
        })
        .ok_or("No name/title found")?;

    match et {
        EntityType::Figure => {
            let entity = figure_from_md(&fm, &body, &name)?;
            let data = serde_json::to_string(&entity).map_err(|e| e.to_string())?;
            Ok(ParsedEntity {
                entity_type: et,
                name,
                data,
            })
        }
        EntityType::Event => {
            let entity = event_from_md(&fm, &body, &name)?;
            let data = serde_json::to_string(&entity).map_err(|e| e.to_string())?;
            Ok(ParsedEntity {
                entity_type: et,
                name,
                data,
            })
        }
        EntityType::Institution => {
            let entity = institution_from_md(&fm, &body, &name)?;
            let data = serde_json::to_string(&entity).map_err(|e| e.to_string())?;
            Ok(ParsedEntity {
                entity_type: et,
                name,
                data,
            })
        }
        EntityType::Work => {
            let entity = work_from_md(&fm, &body, &name)?;
            let data = serde_json::to_string(&entity).map_err(|e| e.to_string())?;
            Ok(ParsedEntity {
                entity_type: et,
                name,
                data,
            })
        }
        EntityType::Geo => {
            let entity = geo_from_md(&fm, &body, &name)?;
            let data = serde_json::to_string(&entity).map_err(|e| e.to_string())?;
            Ok(ParsedEntity {
                entity_type: et,
                name,
                data,
            })
        }
        EntityType::SchoolOfThought => {
            let entity = school_from_md(&fm, &body, &name)?;
            let data = serde_json::to_string(&entity).map_err(|e| e.to_string())?;
            Ok(ParsedEntity {
                entity_type: et,
                name,
                data,
            })
        }
    }
}

/// Logic to decompose an entity into Obsidian-compatible parts.
pub trait MarkdownSerializable {
    /// Scalar metadata fields.
    fn to_frontmatter(&self) -> HashMap<String, YamlValue>;
    /// Rich text body content.
    fn to_body(&self) -> String;
}

// ─── Helpers ─────────────────────────────────────────────────────────────────

fn y(s: &str) -> YamlValue {
    YamlValue::String(s.to_string())
}

fn md(rc: &RichContent) -> String {
    rc.to_markdown()
}

fn date_str(dr: &DateRange) -> String {
    format!("{} / {}", dr.start, dr.end)
}

fn parse_date(s: &str) -> Result<DateRange, String> {
    let parts: Vec<&str> = s.split(" / ").collect();
    if parts.len() != 2 {
        return Err(format!("Bad date range: '{}'", s));
    }
    let start = chrono::NaiveDate::parse_from_str(parts[0].trim(), "%Y-%m-%d")
        .map_err(|e| format!("Bad start date: {}", e))?;
    let end = chrono::NaiveDate::parse_from_str(parts[1].trim(), "%Y-%m-%d")
        .map_err(|e| format!("Bad end date: {}", e))?;
    Ok(DateRange::new(start, end))
}

fn get_str<'a>(fm: &'a HashMap<String, YamlValue>, key: &str) -> Option<&'a str> {
    fm.get(key).and_then(|v| v.as_str())
}

fn entity_type_from_frontmatter(fm: &HashMap<String, YamlValue>) -> Result<EntityType, String> {
    let s = get_str(fm, "entity_type").ok_or("Missing entity_type")?;
    EntityType::from_str(s).ok_or(format!("Unknown entity_type: {}", s))
}

fn extract_title(body: &str) -> Option<String> {
    for line in body.lines() {
        let trimmed = line.trim();
        if trimmed.starts_with("# ") && !trimmed.starts_with("## ") {
            return Some(trimmed[2..].trim().to_string());
        }
    }
    None
}

fn parse_sections(body: &str) -> Vec<(String, String)> {
    let mut sections = Vec::new();
    let mut heading: Option<String> = None;
    let mut content = String::new();
    for line in body.lines() {
        if line.starts_with("## ") {
            if let Some(h) = heading.take() {
                sections.push((h, content.trim().to_string()));
            }
            heading = Some(line[3..].trim().to_string());
            content.clear();
        } else if heading.is_some() {
            content.push_str(line);
            content.push('\n');
        }
    }
    if let Some(h) = heading {
        sections.push((h, content.trim().to_string()));
    }
    sections
}

fn parse_list(content: &str) -> Vec<String> {
    content
        .lines()
        .filter_map(|l| {
            let t = l.trim();
            if t.starts_with("- ") {
                Some(t[2..].to_string())
            } else if !t.is_empty() {
                Some(t.to_string())
            } else {
                None
            }
        })
        .collect()
}

fn refs_to_links(refs: &[EntityRef]) -> String {
    refs.iter()
        .map(|r| {
            format!(
                "[[{}/{}|{}]]",
                r.entity_type.dir_name(),
                r.display_text,
                r.display_text
            )
        })
        .collect::<Vec<_>>()
        .join(", ")
}

fn parse_wiki_refs(s: &str) -> Vec<EntityRef> {
    let rc = RichContent::from_markdown(s);
    rc.segments
        .into_iter()
        .filter_map(|seg| {
            if let crate::core::domain::values::rich_content::ContentSegment::EntityRef(r) = seg {
                Some(r)
            } else {
                None
            }
        })
        .collect()
}

fn relations_section(relations: &[Relation]) -> String {
    let items: Vec<String> = relations
        .iter()
        .map(|r| {
            let kind_str = match &r.kind {
                RelationKind::Custom(s) => s.clone(),
                RelationKind::Fixed(f) => format!("{:?}", f),
            };
            format!(
                "- {} → [[{}/{}|{}]]",
                kind_str,
                r.target.entity_type.dir_name(),
                r.target.display_text,
                r.target.display_text
            )
        })
        .collect();
    format!("\n## Relations\n\n{}", items.join("\n"))
}

fn parse_relations_section(content: &str) -> Vec<Relation> {
    let mut rels = Vec::new();
    for line in content.lines() {
        let t = line.trim();
        if !t.starts_with("- ") {
            continue;
        }
        let t = &t[2..];
        if let Some(arrow) = t.find(" → ") {
            let kind_str = &t[..arrow];
            let link_part = &t[arrow + " → ".len()..];
            let refs = parse_wiki_refs(link_part);
            if let Some(target) = refs.into_iter().next() {
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
                rels.push(Relation { target, kind });
            }
        }
    }
    rels
}

// ─── Figure ──────────────────────────────────────────────────────────────────

impl MarkdownSerializable for Figure {
    fn to_frontmatter(&self) -> HashMap<String, YamlValue> {
        let mut fm = HashMap::new();
        fm.insert("entity_type".into(), y("Figure"));
        fm.insert("life".into(), y(&date_str(&self.life)));
        fm
    }

    fn to_body(&self) -> String {
        let mut s = Vec::new();
        s.push(format!("**Role:** {}", md(&self.primary_role)));
        s.push(format!("**Origin:** {}", md(&self.primary_location)));
        s.push(format!(
            "> {}",
            self.defining_quote.as_ref().map(md).unwrap_or_default()
        ));
        s.push(format!(
            "**Predecessors:** {}",
            refs_to_links(&self.predecessors)
        ));
        s.push(format!(
            "**Rivals:** {}",
            refs_to_links(&self.contemporary_rivals)
        ));
        s.push(format!(
            "**Successors:** {}",
            refs_to_links(&self.successors)
        ));

        let inst_link = self
            .primary_institution
            .as_ref()
            .map(|inst| {
                format!(
                    "[[{}/{}|{}]]",
                    inst.entity_type.dir_name(),
                    inst.display_text,
                    inst.display_text
                )
            })
            .unwrap_or_default();
        s.push(format!("**Institution:** {}", inst_link));

        let rich_sections = vec![
            ("Axiom", &self.axiom),
            ("Argument Flow", &self.argument_flow),
            ("Funding Model", &self.funding_model),
            ("Institutional Product", &self.institutional_product),
            ("Succession Plan", &self.succession_plan),
            ("Short Term Success", &self.short_term_success),
            ("Modern Relevance", &self.modern_relevance),
            ("Critical Flaw", &self.critical_flaw),
            ("Personal Synthesis", &self.personal_synthesis),
        ];
        for (label, field) in rich_sections {
            s.push(format!(
                "\n## {}\n\n{}",
                label,
                field.as_ref().map(md).unwrap_or_default()
            ));
        }
        s.push(relations_section(&self.relations));
        s.join("\n")
    }
}

fn figure_from_md(
    fm: &HashMap<String, YamlValue>,
    body: &str,
    name: &str,
) -> Result<Figure, String> {
    let life = parse_date(get_str(fm, "life").unwrap_or("0001-01-01 / 0001-01-01"))?;
    let mut fig = Figure::new(
        name.to_string(),
        life,
        RichContent::new(),
        RichContent::new(),
    );

    // Parse inline fields from body
    for line in body.lines() {
        let t = line.trim();
        if let Some(val) = t.strip_prefix("**Role:** ") {
            fig.primary_role = RichContent::from_markdown(val);
        } else if let Some(val) = t.strip_prefix("**Origin:** ") {
            fig.primary_location = RichContent::from_markdown(val);
        } else if let Some(val) = t.strip_prefix("> ") {
            if fig.defining_quote.is_none() {
                fig.defining_quote = Some(RichContent::from_markdown(val));
            }
        } else if let Some(val) = t.strip_prefix("**Predecessors:** ") {
            fig.predecessors = parse_wiki_refs(val);
        } else if let Some(val) = t.strip_prefix("**Rivals:** ") {
            fig.contemporary_rivals = parse_wiki_refs(val);
        } else if let Some(val) = t.strip_prefix("**Successors:** ") {
            fig.successors = parse_wiki_refs(val);
        } else if let Some(val) = t.strip_prefix("**Institution:** ") {
            let refs = parse_wiki_refs(val);
            fig.primary_institution = refs.into_iter().next();
        }
    }

    let sections = parse_sections(body);
    for (h, c) in &sections {
        match h.as_str() {
            "Axiom" => fig.axiom = Some(RichContent::from_markdown(c)),
            "Argument Flow" => fig.argument_flow = Some(RichContent::from_markdown(c)),
            "Funding Model" => fig.funding_model = Some(RichContent::from_markdown(c)),
            "Institutional Product" => {
                fig.institutional_product = Some(RichContent::from_markdown(c))
            }
            "Succession Plan" => fig.succession_plan = Some(RichContent::from_markdown(c)),
            "Short Term Success" => fig.short_term_success = Some(RichContent::from_markdown(c)),
            "Modern Relevance" => fig.modern_relevance = Some(RichContent::from_markdown(c)),
            "Critical Flaw" => fig.critical_flaw = Some(RichContent::from_markdown(c)),
            "Personal Synthesis" => fig.personal_synthesis = Some(RichContent::from_markdown(c)),
            "Relations" => fig.relations = parse_relations_section(c),
            _ => {}
        }
    }
    Ok(fig)
}

// ─── Event ───────────────────────────────────────────────────────────────────

impl MarkdownSerializable for Event {
    fn to_frontmatter(&self) -> HashMap<String, YamlValue> {
        let mut fm = HashMap::new();
        fm.insert("entity_type".into(), y("Event"));
        fm.insert("date_range".into(), y(&date_str(&self.date_range)));
        fm
    }

    fn to_body(&self) -> String {
        let mut s = Vec::new();
        s.push(format!(
            "## Description\n\n{}",
            self.description.as_ref().map(md).unwrap_or_default()
        ));

        let causes_items: Vec<String> =
            self.causes.iter().map(|c| format!("- {}", md(c))).collect();
        s.push(format!("## Causes\n\n{}", causes_items.join("\n")));

        let cons_items: Vec<String> = self
            .consequences
            .iter()
            .map(|c| format!("- {}", md(c)))
            .collect();
        s.push(format!("## Consequences\n\n{}", cons_items.join("\n")));

        s.push(relations_section(&self.relations));
        s.join("\n\n")
    }
}

fn event_from_md(fm: &HashMap<String, YamlValue>, body: &str, name: &str) -> Result<Event, String> {
    let dr = parse_date(get_str(fm, "date_range").unwrap_or("0001-01-01 / 0001-01-01"))?;
    let mut ev = Event::new(name.to_string(), dr);
    let sections = parse_sections(body);
    for (h, c) in &sections {
        match h.as_str() {
            "Description" => ev.description = Some(RichContent::from_markdown(c)),
            "Causes" => {
                ev.causes = parse_list(c)
                    .iter()
                    .map(|s| RichContent::from_markdown(s))
                    .collect()
            }
            "Consequences" => {
                ev.consequences = parse_list(c)
                    .iter()
                    .map(|s| RichContent::from_markdown(s))
                    .collect()
            }
            "Relations" => ev.relations = parse_relations_section(c),
            _ => {}
        }
    }
    Ok(ev)
}

// ─── Institution ─────────────────────────────────────────────────────────────

impl MarkdownSerializable for Institution {
    fn to_frontmatter(&self) -> HashMap<String, YamlValue> {
        let mut fm = HashMap::new();
        fm.insert("entity_type".into(), y("Institution"));
        if let Some(f) = &self.founded {
            fm.insert("founded".into(), y(&date_str(f)));
        }
        fm
    }

    fn to_body(&self) -> String {
        let mut s = Vec::new();
        s.push(format!(
            "## Description\n\n{}",
            self.description.as_ref().map(md).unwrap_or_default()
        ));
        s.push(format!("**Founders:** {}", refs_to_links(&self.founders)));

        let prod_items: Vec<String> = self
            .products
            .iter()
            .map(|p| format!("- {}", md(p)))
            .collect();
        s.push(format!("## Products\n\n{}", prod_items.join("\n")));

        s.push(relations_section(&self.relations));
        s.join("\n\n")
    }
}

fn institution_from_md(
    fm: &HashMap<String, YamlValue>,
    body: &str,
    name: &str,
) -> Result<Institution, String> {
    let mut inst = Institution::new(name.to_string());
    inst.founded = get_str(fm, "founded").and_then(|s| parse_date(s).ok());
    for line in body.lines() {
        let t = line.trim();
        if let Some(val) = t.strip_prefix("**Founders:** ") {
            inst.founders = parse_wiki_refs(val);
        }
    }
    let sections = parse_sections(body);
    for (h, c) in &sections {
        match h.as_str() {
            "Description" => inst.description = Some(RichContent::from_markdown(c)),
            "Products" => {
                inst.products = parse_list(c)
                    .iter()
                    .map(|s| RichContent::from_markdown(s))
                    .collect()
            }
            "Relations" => inst.relations = parse_relations_section(c),
            _ => {}
        }
    }
    Ok(inst)
}

// ─── Work ────────────────────────────────────────────────────────────────────

impl MarkdownSerializable for Work {
    fn to_frontmatter(&self) -> HashMap<String, YamlValue> {
        let mut fm = HashMap::new();
        fm.insert("entity_type".into(), y("Work"));
        if let Some(pd) = &self.publication_date {
            fm.insert("publication_date".into(), y(&date_str(pd)));
        }
        fm
    }

    fn to_body(&self) -> String {
        let mut s = Vec::new();
        s.push(format!("**Authors:** {}", refs_to_links(&self.authors)));
        s.push(format!(
            "## Summary\n\n{}",
            self.summary.as_ref().map(md).unwrap_or_default()
        ));

        let ideas_items: Vec<String> = self
            .key_ideas
            .iter()
            .map(|k| format!("- {}", md(k)))
            .collect();
        s.push(format!("## Key Ideas\n\n{}", ideas_items.join("\n")));

        s.push(relations_section(&self.relations));
        s.join("\n\n")
    }
}

fn work_from_md(fm: &HashMap<String, YamlValue>, body: &str, name: &str) -> Result<Work, String> {
    let mut work = Work::new(name.to_string());
    work.publication_date = get_str(fm, "publication_date").and_then(|s| parse_date(s).ok());
    for line in body.lines() {
        let t = line.trim();
        if let Some(val) = t.strip_prefix("**Authors:** ") {
            work.authors = parse_wiki_refs(val);
        }
    }
    let sections = parse_sections(body);
    for (h, c) in &sections {
        match h.as_str() {
            "Summary" => work.summary = Some(RichContent::from_markdown(c)),
            "Key Ideas" => {
                work.key_ideas = parse_list(c)
                    .iter()
                    .map(|s| RichContent::from_markdown(s))
                    .collect()
            }
            "Relations" => work.relations = parse_relations_section(c),
            _ => {}
        }
    }
    Ok(work)
}

// ─── Geo ─────────────────────────────────────────────────────────────────────

impl MarkdownSerializable for Geo {
    fn to_frontmatter(&self) -> HashMap<String, YamlValue> {
        let mut fm = HashMap::new();
        fm.insert("entity_type".into(), y("Geo"));
        if !self.aliases.is_empty() {
            fm.insert(
                "aliases".into(),
                YamlValue::Sequence(
                    self.aliases
                        .iter()
                        .map(|a| YamlValue::String(a.clone()))
                        .collect(),
                ),
            );
        }
        fm
    }

    fn to_body(&self) -> String {
        let mut s = Vec::new();
        s.push(format!(
            "## Region\n\n{}",
            self.region.as_ref().map(md).unwrap_or_default()
        ));
        s.push(format!(
            "## Description\n\n{}",
            self.description.as_ref().map(md).unwrap_or_default()
        ));
        s.push(relations_section(&self.relations));
        s.join("\n\n")
    }
}

fn geo_from_md(fm: &HashMap<String, YamlValue>, body: &str, name: &str) -> Result<Geo, String> {
    let mut geo = Geo::new(name.to_string());
    geo.aliases = fm
        .get("aliases")
        .and_then(|v| v.as_sequence())
        .map(|seq| {
            seq.iter()
                .filter_map(|v| v.as_str().map(|s| s.to_string()))
                .collect()
        })
        .unwrap_or_default();
    let sections = parse_sections(body);
    for (h, c) in &sections {
        match h.as_str() {
            "Region" => geo.region = Some(RichContent::from_markdown(c)),
            "Description" => geo.description = Some(RichContent::from_markdown(c)),
            "Relations" => geo.relations = parse_relations_section(c),
            _ => {}
        }
    }
    Ok(geo)
}

// ─── SchoolOfThought ─────────────────────────────────────────────────────────

impl MarkdownSerializable for SchoolOfThought {
    fn to_frontmatter(&self) -> HashMap<String, YamlValue> {
        let mut fm = HashMap::new();
        fm.insert("entity_type".into(), y("SchoolOfThought"));
        if !self.sub_schools.is_empty() {
            fm.insert(
                "sub_schools".into(),
                YamlValue::Sequence(
                    self.sub_schools
                        .iter()
                        .map(|s| YamlValue::String(s.clone()))
                        .collect(),
                ),
            );
        }
        fm
    }

    fn to_body(&self) -> String {
        let mut s = Vec::new();
        s.push(format!(
            "## Description\n\n{}",
            self.description.as_ref().map(md).unwrap_or_default()
        ));
        s.push(relations_section(&self.relations));
        s.join("\n\n")
    }
}

fn school_from_md(
    fm: &HashMap<String, YamlValue>,
    body: &str,
    name: &str,
) -> Result<SchoolOfThought, String> {
    let mut school = SchoolOfThought::new(name.to_string());
    school.sub_schools = fm
        .get("sub_schools")
        .and_then(|v| v.as_sequence())
        .map(|seq| {
            seq.iter()
                .filter_map(|v| v.as_str().map(|s| s.to_string()))
                .collect()
        })
        .unwrap_or_default();
    let sections = parse_sections(body);
    for (h, c) in &sections {
        match h.as_str() {
            "Description" => school.description = Some(RichContent::from_markdown(c)),
            "Relations" => school.relations = parse_relations_section(c),
            _ => {}
        }
    }
    Ok(school)
}

// ─── File naming ─────────────────────────────────────────────────────────────

/// Sanitizes a string for use as a filesystem path by replacing invalid characters with underscores.
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

/// Returns the designated directory name for a given entity type to ensure consistent folder structuring.
pub fn entity_type_dir(et: &EntityType) -> &'static str {
    et.dir_name()
}
