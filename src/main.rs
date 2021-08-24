mod input;
mod draw;
mod entity;
mod app;
mod stage;
mod util;
mod backend;

extern crate sdl2;

use crate::app::App;
use crate::stage::Stage;


pub fn main() {
    let mut app = App::init_sdl();

    let mut stage = Stage::init_stage(app.canvas());

    app.run_stage(&mut stage);
}