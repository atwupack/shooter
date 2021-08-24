use crate::backend::Backend;
use sdl2::render::WindowCanvas;
use sdl2::EventPump;
use sdl2::image::{Sdl2ImageContext, InitFlag};

pub struct Sdl2Backend {
    canvas: WindowCanvas,
    event: EventPump,
    _image: Sdl2ImageContext,
}

impl Backend for Sdl2Backend {
    fn init(title: &str, width: u32, height:u32) -> Self {
        let sdl_context = sdl2::init().unwrap();

        let video = sdl_context.video().unwrap();
        let window = video.window(title, width, height).build().unwrap();

        sdl2::hint::set("SDL_RENDER_SCALE_QUALITY", "linear");
        let canvas = window.into_canvas().accelerated().build().unwrap();
        let event = sdl_context.event_pump().unwrap();
        let image = sdl2::image::init(InitFlag::JPG | InitFlag::PNG).unwrap();
        Sdl2Backend {
            canvas,
            event,
            _image: image,
        }
    }
}