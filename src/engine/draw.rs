use sdl2::render::{WindowCanvas, TextureCreator, Texture};
use sdl2::video::WindowContext;
use std::collections::HashMap;
use sdl2::image::LoadTexture;
use sdl2::rect::Rect;
use sdl2::pixels::Color;
use std::hash::Hash;

pub struct Graphics {
    canvas: WindowCanvas,
}

impl Graphics {
    pub fn new(canvas: WindowCanvas) -> Graphics {
        Graphics {
            canvas: canvas,
        }
    }

    pub fn set_draw_color(&mut self, r: u8, g: u8, b: u8, a: u8) {
        self.canvas.set_draw_color(Color::RGBA(r, g, b, a));
    }

    pub fn clear(&mut self) {
        self.canvas.clear();
    }

    pub fn present(&mut self) {
        self.canvas.present();
    }

    fn texture_creator(&self) -> TextureCreator<WindowContext> {
        self.canvas.texture_creator()
    }
}

pub struct Textures<T> {
    texture_creator: TextureCreator<WindowContext>,
    texture_store: HashMap<T, Texture>,
}

impl<T: Eq+Hash> Textures<T> {
    pub fn new(graphics: &Graphics) -> Textures<T> {
        Textures {
            texture_creator: graphics.texture_creator(),
            texture_store: HashMap::new(),
        }
    }

    pub fn load_texture(&mut self, entity: T, filename: &str) {
        let texture = self.texture_creator.load_texture(filename).unwrap();
        self.texture_store.insert(entity, texture);
    }

    pub fn blit(&mut self, graphics: &mut Graphics, entity: T, x: i32, y: i32) {
        let texture = self.texture_store.get(&entity).unwrap();
        let query = texture.query();
        let rect = Rect::new(x, y, query.width, query.height);
        graphics.canvas.copy(&texture, None, rect).unwrap();
    }

    pub(crate) fn texture_size(&self, entity: T) -> (u32, u32) {
        let texture = self.texture_store.get(&entity).unwrap();
        let query = texture.query();
        (query.width, query.height)
    }
}
