

pub struct Entity {
    x: i32,
    y: i32,
    dx: i32,
    dy: i32,
    width: u32,
    height: u32,
    health: u32,
    reload: u32,
    texture: String,
}

impl Entity {

    pub fn new(x: i32, y: i32, dx: i32, dy: i32, width:u32, height:u32, health:u32, texture: &str) -> Entity {
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

    pub(crate) fn set_dx(&mut self, dx: i32) {
        self.dx = dx;
    }

    pub(crate) fn set_dy(&mut self, dy: i32) {
        self.dy = dy;
    }

    pub fn x(&self) -> i32 {
        self.x
    }

    pub fn set_x(&mut self, x: i32) {
        self.x = x;
    }

    pub fn y(&self) -> i32 {
        self.y
    }

    pub fn set_y(&mut self, y: i32) {
        self.y = y;
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
}
