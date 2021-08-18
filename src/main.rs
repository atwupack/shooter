mod input;
mod draw;
mod entity;

extern crate sdl2;

use sdl2::{TimerSubsystem, EventPump};
use sdl2::render::{WindowCanvas};
use sdl2::image::{InitFlag, Sdl2ImageContext};
use crate::input::{Inputs, do_input};
use crate::draw::{prepare_scene, present_scene, Textures};
use crate::entity::Entity;

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

    fn blit(&mut self, entity: &Entity) {
        self.textures.blit(&mut self.canvas, entity.texture(), entity.x(), entity.y());
    }
}


pub fn main() {
    let mut app = init_sdl();

    app.load_texture("player","gfx\\player.png");
    let mut player = Entity::new(100, 100,0,0,0,"player");

    app.load_texture("bullet","gfx\\playerBullet.png");
    let mut bullet = Entity::new(100, 100,16,0,0,"bullet");

    loop {
        app.prepare_scene();
        app.do_input();
        if app.inputs.up() {
            player.move_up(4);
        }
        if app.inputs.down() {
            player.move_down(4);
        }
        if app.inputs.left() {
            player.move_left(4);
        }
        if app.inputs.right() {
            player.move_right(4);
        }
        if app.inputs.fire() && bullet.health()==0 {
            bullet.set_x(player.x());
            bullet.set_y(player.y());
            bullet.set_health(1);
        }
        bullet.apply_speed();
        if bullet.x() > SCREEN_WIDTH as i32 {
            bullet.set_health(0);
        }
        app.blit(&player);
        if bullet.health() > 0 {
            app.blit(&bullet);
        }

        app.present_scene();
        app.timer.delay(16);
    }
}