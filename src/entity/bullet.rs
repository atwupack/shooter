use derive_builder::Builder;
use crate::engine::traits::{HasPosition, HasVelocity, IsRendered};
use crate::entity::EntityType;
use crate::engine::util::remove_or_apply;
use crate::util::is_outside_screen;

#[derive(Debug, Clone, PartialEq)]
pub enum BulletType {
    PlayerBullet,
    EnemyBullet,
}

impl Default for BulletType {
    fn default() -> Self {
        BulletType::PlayerBullet
    }
}

#[derive(Default, Builder, Debug)]
#[builder(default)]
#[builder(setter(into))]
pub struct Bullet {
    x: f32,
    y: f32,
    dx: f32,
    dy: f32,
    width: u32,
    height: u32,
    pub has_hit: bool,
    bullet_type: BulletType,
}

impl HasPosition for Bullet {
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
        self.y = y;
    }
}

impl HasVelocity for Bullet {
    fn dx(&self) -> f32 {
        self.dx
    }

    fn set_dx(&mut self, dx: f32) {
        self.dx = dx;
    }

    fn dy(&self) -> f32 {
        self.dy
    }

    fn set_dy(&mut self, dy: f32) {
        self.dy = dy;
    }
}

impl IsRendered<EntityType> for Bullet {
    fn entity_type(&self) -> &EntityType {
        if self.bullet_type == BulletType::PlayerBullet {
            &EntityType::PlayerBullet
        }
        else {
            &EntityType::AlienBullet
        }
    }

    fn width(&self) -> u32 {
        self.width
    }

    fn height(&self) -> u32 {
        self.height
    }
}

pub(crate) fn do_bullets(bullets: &mut Vec<Bullet>) {
    remove_or_apply(
        bullets,
        |bullet| bullet.has_hit || is_outside_screen(bullet),
        |bullet| {
            bullet.apply_velocity();
        },
    );

}
