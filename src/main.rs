mod input;
mod draw;

extern crate sdl2;

use sdl2::{TimerSubsystem, EventPump};
use sdl2::render::{WindowCanvas};
use sdl2::image::{InitFlag, Sdl2ImageContext};
use crate::input::{Inputs, do_input};
use crate::draw::{prepare_scene, present_scene, Textures};

const SCREEN_WIDTH : u32 = 1280;
const SCREEN_HEIGHT : u32 = 720;

fn init_sdl() -> App {
    let sdl_context = sdl2::init().unwrap();
    let video = sdl_context.video().unwrap();

    let window = video.window("Shooter 01", SCREEN_WIDTH, SCREEN_HEIGHT).build().unwrap();

    sdl2::hint::set("SDL_RENDER_SCALE_QUALITY", "linear");

    let canvas = window.into_canvas().accelerated().build().unwrap();

    let timer = sdl_context.timer().unwrap();

    let event = sdl_context.event_pump().unwrap();

    let image = sdl2::image::init(InitFlag::JPG | InitFlag::PNG).unwrap();

    let textures = Textures::new(&canvas);

    App {
        timer,
        canvas,
        event,
        image,
        textures,
        inputs: Inputs::default(),
    }
}


struct App {
    timer: TimerSubsystem,
    canvas: WindowCanvas,
    event: EventPump,
    image: Sdl2ImageContext,
    textures: Textures,
    inputs: Inputs,
}

impl App {
    fn prepare_scene(&mut self) {
        prepare_scene(&mut self.canvas);
    }

    fn present_scene(&mut self) {
        present_scene(&mut self.canvas);
    }

    fn do_input(&mut self) {
        do_input(&mut self.event, &mut self.inputs);
    }

    fn load_texture(&mut self, name: &str, filename: &str) {
        self.textures.load_texture(name, filename);
    }

    fn blit(&mut self, name: &str, x: i32, y: i32) {
        self.textures.blit(&mut self.canvas, name, x, y);
    }
}

struct Player {
    x: i32,
    y: i32,
    texture: String,
}

pub fn main() {
    let mut app = init_sdl();

    app.load_texture("player","gfx\\player.png");

    let mut player = Player {
        x: 100,
        y: 100,
        texture: "player".to_string(),
    };

    loop {
        app.prepare_scene();
        app.do_input();
        if app.inputs.up() {
            player.y -= 4;
        }
        if app.inputs.down() {
            player.y += 4;
        }
        if app.inputs.left() {
            player.x -= 4;
        }
        if app.inputs.right() {
            player.x += 4;
        }
        app.blit(&player.texture, player.x, player.y);
        app.present_scene();
        app.timer.delay(16);
    }
}