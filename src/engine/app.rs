use crate::engine::draw::Graphics;
use crate::engine::input::{do_input, Inputs};
use crate::engine::scene::Scene;
use crate::engine::util::FrameRateTimer;
use sdl2::EventPump;
use std::hash::Hash;
use crate::engine::audio::Sounds;
use crate::defs::MAX_SND_CHANNELS;

pub struct App<T, S> {
    graphics: Graphics<T>,
    sounds: Sounds<S> ,
    event: EventPump,
    inputs: Inputs,
    requested_fps: u32,
}

impl<T: Eq + Hash, S: Eq + Hash> App<T, S> {
    pub fn new(title: &str, width: u32, height: u32, requested_fps: u32) -> Self {
        let sdl_context = sdl2::init().unwrap();

        let video = sdl_context.video().unwrap();
        let window = video.window(title, width, height).build().unwrap();

        sdl2::hint::set("SDL_RENDER_SCALE_QUALITY", "linear");
        let canvas = window.into_canvas().accelerated().build().unwrap();
        let event = sdl_context.event_pump().unwrap();

        sdl_context.mouse().show_cursor(false);

        App {
            graphics: Graphics::new(canvas),
            sounds: Sounds::new(MAX_SND_CHANNELS),
            event,
            inputs: Inputs::default(),
            requested_fps,
        }
    }

    fn do_input(&mut self) {
        do_input(&mut self.event, &mut self.inputs);
    }

    pub(crate) fn run_scene(&mut self, scene: &mut impl Scene<T, S>) {
        scene.init_scene(&mut self.graphics, &mut self.sounds);
        let mut frt = FrameRateTimer::new(self.requested_fps);
        loop {
            scene.prepare_scene(&mut self.graphics);
            self.do_input();
            scene.logic(&self.inputs, &mut self.graphics, &mut self.sounds);
            scene.draw(&mut self.graphics);
            scene.present_scene(&mut self.graphics);
            frt.cap_frame_rate();
        }
    }
}
