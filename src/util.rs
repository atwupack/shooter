use crate::defs::{SCREEN_WIDTH, SCREEN_HEIGHT};
use geemu_engine::traits::IsRendered;
use geemu_engine::draw::Graphics;

pub fn is_outside_screen(entity: &impl IsRendered) -> bool {
    (entity.x() < -(entity.width() as f32)) || (entity.y() < -(entity.height() as f32)) ||  entity.x() > SCREEN_WIDTH as f32 || entity.y() > SCREEN_HEIGHT as f32
}

pub(crate) fn draw_entities<'a>(
    entities: impl IntoIterator<Item = &'a(impl IsRendered + 'a)>,
    graphics: &mut Graphics,
) {
    for entity in entities {
        graphics.blit(entity)
    }
}


