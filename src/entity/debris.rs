use derive_builder::Builder;
use crate::entity::EntityType;

#[derive(Default, Builder, Debug)]
#[builder(default)]
#[builder(setter(into))]
pub struct Debris {
    pub x: f32,
    pub y: f32,
    pub dx: f32,
    pub dy: f32,
    life: i32,
    entity_type: EntityType,
}

