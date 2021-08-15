extern crate sdl2;

use sdl2::pixels::Color;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use std::time::Duration;
use sdl2::{TimerSubsystem, EventPump};
use sdl2::render::{WindowCanvas, Texture, TextureCreator};
use std::process::exit;
use sdl2::image::{InitFlag, Sdl2ImageContext, LoadTexture};
use sdl2::video::WindowContext;

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

    let texture_creator = canvas.texture_creator();

    App {
        timer,
        canvas,
        event,
        image,
        texture_creator,
    }
}


struct App {
    timer: TimerSubsystem,
    canvas: WindowCanvas,
    event: EventPump,
    image: Sdl2ImageContext,
    texture_creator: TextureCreator<WindowContext>,
}

impl App {
    fn prepare_scene(&mut self) {
        self.canvas.set_draw_color(Color::RGBA(96, 128, 255, 255));
        self.canvas.clear();
    }

    fn present_scene(&mut self) {
        self.canvas.present();
    }

    fn do_input(&mut self) {
        for event in self.event.poll_iter() {
            match event {
                Event::Quit {..} => exit(0),
                _ => {}
            }
        }
    }

    fn load_texture(&self, filename: &str) -> Texture {
        return self.texture_creator.load_texture(filename).unwrap();
    }
}

struct Player<'r> {
    x: i32,
    y: i32,
    texture: Texture<'r>,
}

pub fn main() {
    let mut app = init_sdl();

    let player = Player {
        x: 100,
        y: 100,
        texture: app.load_texture("gfx\\player.png"),
    };

    loop {
        app.prepare_scene();
        app.do_input();
        app.present_scene();
        app.timer.delay(16);
    }
}