use crate::engine::draw::Graphics;
use crate::engine::input::{do_input, Inputs};
use crate::engine::scene::Scene;
use crate::engine::util::FrameRateTimer;
use sdl2::image::{InitFlag, Sdl2ImageContext};
use sdl2::EventPump;
use std::hash::Hash;

pub struct App<T> {
    graphics: Graphics<T>,
    event: EventPump,
    _image: Sdl2ImageContext,
    inputs: Inputs,
    requested_fps: u32,
}

impl<T: Eq + Hash> App<T> {
    pub fn new(title: &str, width: u32, height: u32, requested_fps: u32) -> Self {
        let sdl_context = sdl2::init().unwrap();

        let video = sdl_context.video().unwrap();
        let window = video.window(title, width, height).build().unwrap();

        sdl2::hint::set("SDL_RENDER_SCALE_QUALITY", "linear");
        let canvas = window.into_canvas().accelerated().build().unwrap();
        let event = sdl_context.event_pump().unwrap();
        let image = sdl2::image::init(InitFlag::JPG | InitFlag::PNG).unwrap();

        sdl_context.mouse().show_cursor(false);

        App {
            graphics: Graphics::new(canvas),
            event,
            _image: image,
            inputs: Inputs::default(),
            requested_fps,
        }
    }

    fn do_input(&mut self) {
        do_input(&mut self.event, &mut self.inputs);
    }

    pub(crate) fn run_scene(&mut self, scene: &mut impl Scene<T>) {
        scene.init_scene(&mut self.graphics);
        let mut frt = FrameRateTimer::new(self.requested_fps);
        loop {
            scene.prepare_scene(&mut self.graphics);
            self.do_input();
            scene.logic(&self.inputs, &mut self.graphics);
            scene.draw(&mut self.graphics);
            scene.present_scene(&mut self.graphics);
            frt.cap_frame_rate();
        }
    }
}
