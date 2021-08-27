use engine::app::App;

use crate::defs::{FPS, SCREEN_HEIGHT, SCREEN_WIDTH};
use crate::stage::Stage;

mod defs;
mod entity;
mod stage;
mod engine;

extern crate sdl2;

pub fn main() {
    let mut app = App::new("Shooter", SCREEN_WIDTH, SCREEN_HEIGHT, FPS);

    let mut stage = Stage::init_stage(app.graphics());

    app.run_scene(&mut stage);
}
