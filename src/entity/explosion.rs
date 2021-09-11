
use derive_builder::Builder;
use crate::engine::traits::{HasPosition, HasVelocity};

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


