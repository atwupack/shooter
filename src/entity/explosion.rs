
use derive_builder::Builder;
use crate::engine::traits::{HasPosition, HasVelocity, IsRendered};
use crate::engine::util::remove_or_apply;
use crate::entity::EntityType;
use crate::engine::draw::Graphics;
use rand::random;
use crate::defs::FPS;

#[derive(Default, Builder, Debug)]
#[builder(default)]
#[builder(setter(into))]
pub struct Explosion {
    pub x: f32,
    pub y: f32,
    pub dx: f32,
    pub dy: f32,
    r: u8,
    g: u8,
    b: u8,
    pub a: u8,
}

impl HasPosition for Explosion {
    fn x(&self) -> f32 {
        self.x
    }

    fn set_x(&mut self, x: f32) {
        self.x = x
    }

    fn y(&self) -> f32 {
        self.y
    }

    fn set_y(&mut self, y: f32) {
        self.y = y
    }
}

impl HasVelocity for Explosion {
    fn dx(&self) -> f32 {
        self.dx
    }

    fn set_dx(&mut self, dx: f32) {
        self.dx = dx
    }

    fn dy(&self) -> f32 {
        self.dy
    }

    fn set_dy(&mut self, dy: f32) {
        self.dy = dy
    }
}

impl IsRendered<EntityType> for Explosion {
    fn entity_type(&self) -> &EntityType {
        &EntityType::Explosion
    }
}

pub fn do_explosions(explosions: &mut Vec<Explosion>) {
    remove_or_apply(
        explosions,
        |ex| {
            ex.a <= 1
        } ,
        |ex| {
            ex.a -= 1;
            ex.apply_velocity();
        },
    );
}

pub fn draw_explosions(explosions: &Vec<Explosion>, graphics: &mut Graphics<EntityType>) {
    for explosion in explosions {
        graphics.blit_add(explosion, explosion.r, explosion.g, explosion.b, explosion.a);
    }
}

pub fn add_explosions(explosions: &mut Vec<Explosion>, x: f32, y: f32, num: u8) {
    for i in 0..num {

        let mode = random::<u32>() % 4;

        let mut r: u8 = 0;
        let mut g: u8 = 0;
        let mut b: u8 = 0;

        match mode {
            0 => r = 255,
            1 => {
                r = 255;
                g = 128
            },
            2 => {
                r = 255;
                g = 255;
            },
            _ => {
                r = 255;
                g = 255;
                b = 255;
            }
        }


        let explosion = ExplosionBuilder::default()
            .x(x + ((random::<i32>() % 32) - (random::<i32>() % 32)) as f32)
            .y(y + ((random::<i32>() % 32) - (random::<i32>() % 32)) as f32)
            .dx(0.1 *  ((random::<i32>() % 10) - (random::<i32>() % 10)) as f32)
            .dy(0.1 * ((random::<i32>() % 10) - (random::<i32>() % 10)) as f32)
            .r(r)
            .g(g)
            .b(b)
            .a((random::<u32>() % FPS * 3) as u8)
            .build().unwrap();
        explosions.push(explosion);
    }
}



