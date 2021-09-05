use engine::app::App;

use crate::defs::{FPS, SCREEN_HEIGHT, SCREEN_WIDTH};
use crate::stage::Stage;

mod defs;
mod engine;
mod entity;
mod stage;

extern crate sdl2;

pub fn main() {
    let mut app = App::new("Shooter", SCREEN_WIDTH, SCREEN_HEIGHT, FPS);

    let mut stage = Stage::default();

    app.run_scene(&mut stage);
}
