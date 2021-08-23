use derive_builder::Builder;
use crate::entity::EntityType::PLAYER;

#[derive(PartialEq, Eq,Hash, Debug, Clone)]
pub enum EntityType {
    PLAYER,
    ENEMY,
    BULLET,
}

impl Default for EntityType {
    fn default() -> Self {
        PLAYER
    }
}

#[derive(Default, Builder, Debug)]
#[builder(default)]
#[builder(setter(into))]
pub struct Entity {
    x: f32,
    y: f32,
    dx: f32,
    dy: f32,
    width: u32,
    height: u32,
    health: u32,
    reload: u32,
    entity_type: EntityType
}

impl Entity {

    pub(crate) fn height(&self) -> u32 {
        self.height
    }

    pub(crate) fn reload(&self) -> u32 {
        self.reload
    }

    pub(crate) fn set_reload(&mut self, reload: u32) {
        self.reload = reload;
    }

    pub(crate) fn dec_reload(&mut self) {
        if self.reload > 0 {
            self.reload -= 1;
        }
    }

    pub(crate) fn apply_speed(&mut self) {
        self.x += self.dx;
        self.y += self.dy;
    }

    pub(crate) fn set_dx(&mut self, dx: f32) {
        self.dx = dx;
    }

    pub(crate) fn set_dy(&mut self, dy: f32) {
        self.dy = dy;
    }

    pub(crate) fn x(&self) -> f32 {
        self.x
    }

    pub(crate) fn y(&self) -> f32 {
        self.y
    }

    pub(crate) fn set_health(&mut self, health: u32) {
        self.health = health;
    }

    pub(crate) fn health(&self) -> u32 {
        self.health
    }

    pub(crate) fn width(&self) -> u32 {
        self.width
    }

    pub(crate) fn entity_type(&self) -> EntityType {
        self.entity_type.clone()
    }
}
