

pub struct Entity {
    x: f32,
    y: f32,
    dx: f32,
    dy: f32,
    width: u32,
    height: u32,
    health: u32,
    reload: u32,
    texture: String,
}

impl Entity {

    pub fn new(x: f32, y: f32, dx: f32, dy: f32, width:u32, height:u32, health:u32, texture: &str) -> Entity {
        Entity {
            x,
            y,
            dx,
            dy,
            width,
            height,
            health,
            reload: 0,
            texture: texture.to_string(),
        }
    }

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

    pub fn x(&self) -> f32 {
        self.x
    }

    pub fn y(&self) -> f32 {
        self.y
    }

    pub fn texture(&self) -> &str {
        self.texture.as_str()
    }

    pub fn set_health(&mut self, health: u32) {
        self.health = health;
    }

    pub fn health(&self) -> u32 {
        self.health
    }

    pub(crate) fn width(&self) -> u32 {
        self.width
    }
}
