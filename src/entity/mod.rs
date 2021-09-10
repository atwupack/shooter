pub mod explosion;

use derive_builder::Builder;
use crate::engine::traits::{HasPosition, IsRendered, HasVelocity};

#[derive(PartialEq, Eq, Hash, Debug, Clone)]
pub enum EntityType {
    Player,
    Enemy,
    PlayerBullet,
    AlienBullet,
    Background,
    Explosion,
}

impl Default for EntityType {
    fn default() -> Self {
        Self::Player
    }
}

#[derive(Default, Builder, Debug)]
#[builder(default)]
#[builder(setter(into))]
pub struct Entity {
    pub x: f32,
    pub y: f32,
    pub dx: f32,
    pub dy: f32,
    width: u32,
    height: u32,
    pub health: u32,
    reload: u32,
    entity_type: EntityType,
}

impl Entity {
    pub(crate) fn restrict_position(&mut self, min_x: f32, min_y: f32, max_x: f32, max_y: f32) {
        if self.x < min_x {
            self.x = min_x;
        }
        if self.y < min_y {
            self.y = min_y;
        }
        if self.x > max_x {
            self.x = max_x;
        }
        if self.y > max_y {
            self.y = max_y;
        }
    }

    pub(crate) fn height(&self) -> u32 {
        self.height
    }

    pub(crate) fn set_reload(&mut self, reload: u32) {
        self.reload = reload;
    }

    pub(crate) fn reload_done(&mut self) -> bool {
        if self.reload > 0 {
            self.reload -= 1;
        }
        self.reload <= 0
    }

    pub(crate) fn width(&self) -> u32 {
        self.width
    }

}

impl HasPosition for Entity {
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

impl HasVelocity for Entity {
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

impl IsRendered<EntityType> for Entity {
    fn entity_type(&self) -> &EntityType {
        &self.entity_type
    }
}


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

#[derive(Default, Builder, Debug)]
#[builder(default)]
#[builder(setter(into))]
pub struct Star {
    pub x: i32,
    pub y: i32,
    pub speed: u8,
}
