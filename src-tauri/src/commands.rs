pub mod figure;
pub mod institution;
pub mod event;
pub mod geo;
pub mod work;
pub mod school;
pub mod search;
pub mod common;

use serde::Deserialize;
use uuid::Uuid;

#[derive(Deserialize)]
pub struct RelationDto {
    pub target_id: Uuid,
    pub relation_type: String,
}
