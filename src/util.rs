use crate::engine::traits::IsRendered;
use crate::entity::EntityType;
use crate::defs::{SCREEN_WIDTH, SCREEN_HEIGHT};

pub fn is_outside_screen(entity: &impl IsRendered<EntityType>) -> bool {
    (entity.x() < -(entity.width() as f32)) || (entity.y() < -(entity.height() as f32)) ||  entity.x() > SCREEN_WIDTH as f32 || entity.y() > SCREEN_HEIGHT as f32
}
