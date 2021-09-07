use derive_builder::Builder;

#[derive(PartialEq, Eq, Hash, Debug, Clone)]
pub enum EntityType {
    Player,
    Enemy,
    PlayerBullet,
    AlienBullet,
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

    pub(crate) fn apply_speed(&mut self) {
        self.x += self.dx;
        self.y += self.dy;
    }

    pub(crate) fn width(&self) -> u32 {
        self.width
    }

    pub(crate) fn entity_type(&self) -> EntityType {
        self.entity_type.clone()
    }
}
