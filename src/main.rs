use crate::defs::{FPS, SCREEN_HEIGHT, SCREEN_WIDTH, MAX_SND_CHANNELS};
use crate::stage::Stage;
use std::error::Error;
use geemu_engine::app::App;

mod defs;
mod stage;
mod entity;
mod util;
mod background;
mod sound;
mod text;

pub fn main() -> Result<(), Box<dyn Error>> {

    let mut app = App::new("Shooter", SCREEN_WIDTH, SCREEN_HEIGHT, FPS, MAX_SND_CHANNELS);

    let mut stage = Stage::default();

    app.run_scene(&mut stage)?;
    Ok(())
}
