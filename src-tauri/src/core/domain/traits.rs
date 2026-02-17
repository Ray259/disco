use uuid::Uuid;
use crate::core::domain::values::entity_ref::EntityType;
use crate::commands::RelationDto;

/// Trait implemented by all Domain Entities (Figure, Institution, etc.)
/// Allows generic access to common metadata.
pub trait DomainEntity: serde::Serialize + serde::de::DeserializeOwned + Send + Sync {
    fn id(&self) -> Uuid;
    fn entity_type(&self) -> EntityType;
    fn name(&self) -> String;
    fn set_updated_at(&mut self, date: chrono::DateTime<chrono::Utc>);
}

/// Trait implemented by Request DTOs (CreateFigureRequest, etc.)
/// Encapsulates the logic to convert a request into a Domain Entity and update it.
pub trait InputDto<E: DomainEntity> {
    /// Converts the DTO into a new Domain Entity with the given ID.
    fn to_entity(&self, id: Uuid) -> Result<E, String>;

    /// Updates an existing Domain Entity with values from the DTO.
    fn update_entity(&self, entity: &mut E) -> Result<(), String>;

    /// Returns the list of relations to be inserted/updated.
    fn get_relations(&self) -> Option<Vec<RelationDto>>;
}
