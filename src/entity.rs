

pub struct Entity {
    x: i32,
    y: i32,
    dx: i32,
    dy: i32,
    health: u32,
    texture: String,
}

impl Entity {

    pub fn new(x: i32, y: i32, dx: i32, dy: i32, health: u32, texture: &str) -> Entity {
        Entity {
            x,
            y,
            dx,
            dy,
            health,
            texture: texture.to_string(),
        }
    }

    pub fn apply_speed(&mut self) {
        self.x += self.dx;
        self.y += self.dy;
    }

    pub fn move_up(&mut self, dist: i32) {
        self.y -= dist;
    }

    pub fn move_down(&mut self, dist: i32) {
        self.y += dist;
    }

    pub fn move_left(&mut self, dist: i32) {
        self.x -= dist;
    }

    pub fn move_right(&mut self, dist: i32) {
        self.x += dist;
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
