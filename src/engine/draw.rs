use sdl2::image::LoadTexture;
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::{Texture, TextureCreator, WindowCanvas, BlendMode};
use sdl2::video::WindowContext;
use std::collections::HashMap;
use std::hash::Hash;
use crate::engine::traits::IsRendered;

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

    pub fn draw_line(&mut self, x1: i32, y1: i32, x2: i32, y2: i32) {
        self.canvas.draw_line((x1, y1), (x2, y2)).unwrap();
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

    pub fn blit(&mut self, entity: &impl IsRendered<T>) {
        let texture = self.textures.texture_store.get(&entity.entity_type()).unwrap();
        let query = texture.query();
        let rect = Rect::new(entity.x() as i32, entity.y() as i32, query.width, query.height);
        self.canvas.copy(&texture, None, rect).unwrap();
    }

    pub fn blit_size(&mut self, entity: T, x: i32, y: i32, w: u32, h: u32) {
        let texture = self.textures.texture_store.get(&entity).unwrap();
        let rect = Rect::new(x, y, w, h);
        self.canvas.copy(&texture, None, rect).unwrap();
    }

    pub fn blit_add(&mut self, entity: &impl IsRendered<T>, r: u8, g: u8, b: u8, a: u8) {
        self.canvas.set_blend_mode(BlendMode::Add);
        let texture = self.textures.texture_store.get_mut(&entity.entity_type()).unwrap();
        texture.set_blend_mode(BlendMode::Add);
        texture.set_color_mod(r,g,b);
        texture.set_alpha_mod(a);
        self.blit(entity);
        self.canvas.set_blend_mode(BlendMode::None);
    }

    pub fn blit_rect(&mut self, entity: &T, src: (i32, i32, u32, u32), x: i32, y: i32) {
        let texture = self.textures.texture_store.get(entity).unwrap();
        self.canvas.copy(&texture, Rect::from(src), Rect::new(x, y, src.2, src.3)).unwrap();
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
