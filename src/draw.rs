use crate::entity::EntityType;
use sdl2::image::LoadTexture;
use sdl2::rect::Rect;
use sdl2::render::{Texture, TextureCreator, WindowCanvas};
use sdl2::video::WindowContext;
use std::collections::HashMap;

pub struct Textures {
    texture_creator: TextureCreator<WindowContext>,
    texture_store: HashMap<EntityType, Texture>,
}

impl Textures {
    pub fn new(canvas: &WindowCanvas) -> Textures {
        Textures {
            texture_creator: canvas.texture_creator(),
            texture_store: HashMap::new(),
        }
    }

    pub fn load_texture(&mut self, entity: EntityType, filename: &str) {
        let texture = self.texture_creator.load_texture(filename).unwrap();
        self.texture_store.insert(entity, texture);
    }

    pub fn blit(&mut self, canvas: &mut WindowCanvas, entity: EntityType, x: i32, y: i32) {
        let texture = self.texture_store.get(&entity).unwrap();
        let query = texture.query();
        let rect = Rect::new(x, y, query.width, query.height);
        canvas.copy(&texture, None, rect).unwrap();
    }

    pub(crate) fn texture_size(&self, entity: EntityType) -> (u32, u32) {
        let texture = self.texture_store.get(&entity).unwrap();
        let query = texture.query();
        (query.width, query.height)
    }
}
