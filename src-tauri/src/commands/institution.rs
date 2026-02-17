use tauri::State;
use uuid::Uuid;
use crate::core::db::EncyclopediaDb;
use crate::core::domain::models::institution::Institution;
use crate::core::domain::values::entity_ref::EntityType;
use crate::core::domain::values::rich_content::RichContent;
use crate::core::domain::values::date_range::DateRange;
use crate::core::domain::traits::InputDto;
use serde::Deserialize;
use chrono::NaiveDate;
use super::RelationDto;
use super::common::{handle_create, handle_update};

/// DTO for creating a new Institution.
#[derive(Deserialize)]
pub struct CreateInstitutionRequest {
    pub name: String,
    pub founded_start: Option<String>,
    pub founded_end: Option<String>,
    pub description: Option<String>,
    pub relations: Option<Vec<crate::commands::RelationDto>>,
}

impl InputDto<Institution> for CreateInstitutionRequest {
    fn to_entity(&self, id: Uuid) -> Result<Institution, String> {
        let mut institution = Institution::new(id, self.name.clone());

        if let Some(desc) = &self.description {
            if !desc.is_empty() {
                institution = institution.with_description(RichContent::from_text(desc));
            }
        }

        if let (Some(start), Some(end)) = (&self.founded_start, &self.founded_end) {
            if !start.is_empty() && !end.is_empty() {
                 let s = NaiveDate::parse_from_str(start, "%Y-%m-%d")
                    .or_else(|_| NaiveDate::parse_from_str(&format!("{}-01-01", start), "%Y-%m-%d"))
                    .map_err(|_| "Invalid founded start year".to_string())?;
                 let e = NaiveDate::parse_from_str(end, "%Y-%m-%d")
                    .or_else(|_| NaiveDate::parse_from_str(&format!("{}-01-01", end), "%Y-%m-%d"))
                    .map_err(|_| "Invalid founded end year".to_string())?;
                 
                 institution.founded = Some(DateRange::new(s, e));
            }
        }
        
        Ok(institution)
    }

    fn update_entity(&self, institution: &mut Institution) -> Result<(), String> {
        institution.name = self.name.clone();
        
        if let Some(desc) = &self.description {
            if !desc.is_empty() {
                institution.description = Some(RichContent::from_text(desc));
            } else {
                institution.description = None;
            }
        } else {
            institution.description = None;
        }
    
        if let (Some(start), Some(end)) = (&self.founded_start, &self.founded_end) {
            if !start.is_empty() && !end.is_empty() {
                 let s = NaiveDate::parse_from_str(start, "%Y-%m-%d")
                    .or_else(|_| NaiveDate::parse_from_str(&format!("{}-01-01", start), "%Y-%m-%d"))
                    .map_err(|_| "Invalid founded start year".to_string())?;
                 let e = NaiveDate::parse_from_str(end, "%Y-%m-%d")
                    .or_else(|_| NaiveDate::parse_from_str(&format!("{}-01-01", end), "%Y-%m-%d"))
                    .map_err(|_| "Invalid founded end year".to_string())?;
                 
                 institution.founded = Some(DateRange::new(s, e));
            } else {
                institution.founded = None;
            }
        } else {
            institution.founded = None;
        }

        Ok(())
    }

    fn get_relations(&self) -> Option<Vec<RelationDto>> {
        let rels = self.relations.as_ref()?;
        Some(rels.iter().map(|r| RelationDto {
            target_id: r.target_id,
            relation_type: r.relation_type.clone()
        }).collect())
    }
}

/// Retrieves all entities with type `Institution`.
#[tauri::command]
pub async fn get_all_institutions(state: State<'_, EncyclopediaDb>) -> Result<Vec<Institution>, String> {
    let entities = state.list_entities(Some(EntityType::Institution))
        .await
        .map_err(|e| e.to_string())?;

    let items: Result<Vec<Institution>, String> = entities.into_iter()
        .map(|(id, _name, data)| {
             let mut entity: Institution = serde_json::from_str(&data).map_err(|e| e.to_string())?;
             entity.id = id;
             Ok(entity)
        })
        .collect();

    items
}

/// Creates a new Institution and persists it.
#[tauri::command]
pub async fn create_institution(state: State<'_, EncyclopediaDb>, request: CreateInstitutionRequest) -> Result<String, String> {
    handle_create(state, request).await
}

#[tauri::command]
pub async fn update_institution(state: State<'_, EncyclopediaDb>, id: Uuid, request: CreateInstitutionRequest) -> Result<String, String> {
    handle_update(state, id, request).await
}
