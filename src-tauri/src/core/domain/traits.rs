use crate::core::domain::values::entity_ref::EntityType;

pub trait DomainEntity: serde::Serialize + serde::de::DeserializeOwned + Send + Sync {
    fn entity_type(&self) -> EntityType;
    fn name(&self) -> String;
    fn set_updated_at(&mut self, date: chrono::DateTime<chrono::Utc>);
}

pub trait InputDto<E: DomainEntity> {
    fn to_entity(&self) -> Result<E, String>;
    fn update_entity(&self, entity: &mut E) -> Result<(), String>;
}
