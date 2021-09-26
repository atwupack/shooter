use derive_builder::Builder;
use rand::random;
use crate::defs::FPS;
use geemu_engine::draw::Graphics;
use geemu_engine::util::remove_or_apply;
use geemu_engine::traits::IsRendered;

#[derive(Default, Builder, Debug)]
#[builder(default)]
#[builder(setter(into))]
pub struct Debris {
    pub x: f32,
    pub y: f32,
    pub dx: f32,
    pub dy: f32,
    life: i32,
    entity_type: String,
    rect: (i32, i32, u32, u32),
}

pub(crate) fn draw_debris(debris: &Vec<Debris>, graphics: &mut Graphics) {
    for d in debris {
        graphics.blit_rect(d.entity_type.clone(), d.rect, d.x as i32, d.y as i32)
    }
}

pub(crate) fn add_debris(entity: &impl IsRendered, debris: &mut Vec<Debris>) {

    let w = entity.width() / 2;
    let h = entity.height() / 2;

    for y in (0..=h).step_by(h as usize) {
        for x in (0..=w).step_by(w as usize) {
            debris.push(DebrisBuilder::default()
                .x(entity.x() + (entity.width() as f32) / 2.0)
                .y(entity.y() + (entity.height() as f32) / 2.0)
                .dx(((random::<i32>() % 5) - (random::<i32>() % 5)) as f32)
                .dy((-5 - (random::<i32>() % 12)) as f32)
                .life((FPS * 2) as i32)
                .entity_type(entity.entity_type())
                .rect((x as i32, y as i32, w, h))
                .build().unwrap());
        }
    }
}

pub(crate) fn do_debris(debris: &mut Vec<Debris>) {
    remove_or_apply(debris, |d| {
        d.life <= 1
    }, |d| {
        d.x += d.dx;
        d.y += d.dy;
        d.dy += 0.5;
        d.life -= 1;
    });
}

