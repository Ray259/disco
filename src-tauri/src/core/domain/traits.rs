use crate::core::domain::values::entity_ref::EntityType;

/// Base trait implemented by all core domain models.
/// Requires serialization, deserialization, thread-safety, and common identification fields.
pub trait DomainEntity: serde::Serialize + serde::de::DeserializeOwned + Send + Sync {
    /// Returns the type identifier for this entity.
    fn entity_type(&self) -> EntityType;
    /// Returns the unique name of the entity.
    fn name(&self) -> String;
    /// Updates the modification timestamp.
    fn set_updated_at(&mut self, date: chrono::DateTime<chrono::Utc>);
}

/// Trait for converting input DTOs to domain entities.
pub trait InputDto<E: DomainEntity> {
    /// Creates a new entity from the DTO.
    fn to_entity(&self) -> Result<E, String>;
    /// Updates an existing entity from the DTO.
    fn update_entity(&self, entity: &mut E) -> Result<(), String>;
}
