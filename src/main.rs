mod app;
mod draw;
mod entity;
mod input;
mod stage;
mod util;
mod defs;

extern crate sdl2;

use crate::app::App;
use crate::stage::Stage;

pub fn main() {
    let mut app = App::init_sdl();

    let mut stage = Stage::init_stage(app.canvas());

    app.run_stage(&mut stage);
}
