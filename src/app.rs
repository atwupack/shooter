use sdl2::{TimerSubsystem, EventPump};
use sdl2::render::WindowCanvas;
use sdl2::image::{Sdl2ImageContext, InitFlag};
use crate::input::{Inputs, do_input};
use crate::stage::Stage;
use sdl2::pixels::Color;

pub const SCREEN_WIDTH : u32 = 1280;
pub const SCREEN_HEIGHT : u32 = 720;


pub struct App {
    timer: TimerSubsystem,
    canvas: WindowCanvas,
    event: EventPump,
    _image: Sdl2ImageContext,
    inputs: Inputs,
}

impl App {

    pub fn init_sdl() -> Self {
        let sdl_context = sdl2::init().unwrap();

        let video = sdl_context.video().unwrap();
        let window = video.window("Shooter", SCREEN_WIDTH, SCREEN_HEIGHT).build().unwrap();

        sdl2::hint::set("SDL_RENDER_SCALE_QUALITY", "linear");
        let canvas = window.into_canvas().accelerated().build().unwrap();
        let timer = sdl_context.timer().unwrap();
        let event = sdl_context.event_pump().unwrap();
        let image = sdl2::image::init(InitFlag::JPG | InitFlag::PNG).unwrap();

        App {
            timer,
            canvas,
            event,
            _image: image,
            inputs: Inputs::default(),
        }
    }

    fn prepare_scene(&mut self) {
        self.canvas.set_draw_color(Color::RGBA(96, 128, 255, 255));
        self.canvas.clear();
    }

    fn present_scene(&mut self) {
        self.canvas.present();
    }

    fn do_input(&mut self) {
        do_input(&mut self.event, &mut self.inputs);
    }

    pub(crate) fn canvas(&self) -> &WindowCanvas {
        &self.canvas
    }

    pub(crate) fn run_stage(&mut self, stage: &mut Stage) {
        loop {
            self.prepare_scene();
            self.do_input();
            stage.logic(&self.inputs);
            stage.draw(&mut self.canvas);
            self.present_scene();
            self.timer.delay(16);
        }
    }
}

