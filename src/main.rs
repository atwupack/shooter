use engine::app::App;

use crate::defs::{FPS, SCREEN_HEIGHT, SCREEN_WIDTH};
use crate::stage::Stage;
use std::error::Error;

mod defs;
mod engine;
mod stage;
mod entity;
mod util;
mod background;
mod sound;
mod text;

extern crate sdl2;

pub fn main() -> Result<(), Box<dyn Error>> {

    let mut app = App::new("Shooter", SCREEN_WIDTH, SCREEN_HEIGHT, FPS);

    let mut stage = Stage::default();

    app.run_scene(&mut stage)?;
    Ok(())
}
