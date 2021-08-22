use sdl2::render::{WindowCanvas, TextureCreator, Texture};
use sdl2::video::WindowContext;
use std::collections::HashMap;
use sdl2::image::LoadTexture;
use sdl2::rect::Rect;

pub struct Textures {
    texture_creator: TextureCreator<WindowContext>,
    texture_store: HashMap<String, Texture>,
}

impl Textures {

    pub fn new(canvas: &WindowCanvas) -> Textures {
        Textures {
            texture_creator: canvas.texture_creator(),
            texture_store: HashMap::new(),
        }
    }

    pub fn load_texture(&mut self, name: &str, filename: &str) {
        let texture = self.texture_creator.load_texture(filename).unwrap();
        self.texture_store.insert(name.to_string(), texture);
    }

    pub fn blit(&mut self,canvas: &mut WindowCanvas, name: &str, x: i32, y: i32) {
        let texture = self.texture_store.get(&name.to_string()).unwrap();
        let query = texture.query();
        let rect = Rect::new(x, y, query.width, query.height);
        canvas.copy(&texture, None, rect).unwrap();
    }

    pub(crate) fn texture_size(&self, name: &str) -> (u32, u32) {
        let texture = self.texture_store.get(&name.to_string()).unwrap();
        let query = texture.query();
        (query.width, query.height)
    }
}