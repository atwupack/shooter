use sdl2::image::LoadTexture;
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::{Texture, TextureCreator, WindowCanvas};
use sdl2::video::WindowContext;
use std::collections::HashMap;
use std::hash::Hash;

pub struct Graphics<T> {
    canvas: WindowCanvas,
    textures: Textures<T>,
}

impl<T: Eq + Hash> Graphics<T> {
    pub fn new(canvas: WindowCanvas) -> Graphics<T> {
        let textures = Textures::new(&canvas);

        Graphics { canvas, textures }
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

    pub fn load_texture(&mut self, entity: T, filename: &str) {
        let texture = self
            .textures
            .texture_creator
            .load_texture(filename)
            .unwrap();
        self.textures.texture_store.insert(entity, texture);
    }

    pub(crate) fn texture_size(&self, entity: T) -> (u32, u32) {
        let texture = self.textures.texture_store.get(&entity).unwrap();
        let query = texture.query();
        (query.width, query.height)
    }

    pub fn blit(&mut self, entity: T, x: i32, y: i32) {
        let texture = self.textures.texture_store.get(&entity).unwrap();
        let query = texture.query();
        let rect = Rect::new(x, y, query.width, query.height);
        self.canvas.copy(&texture, None, rect).unwrap();
    }
}

struct Textures<T> {
    texture_creator: TextureCreator<WindowContext>,
    texture_store: HashMap<T, Texture>,
}

impl<T: Eq + Hash> Textures<T> {
    fn new(canvas: &WindowCanvas) -> Textures<T> {
        Textures {
            texture_creator: canvas.texture_creator(),
            texture_store: HashMap::new(),
        }
    }
}
