use geemu_audio::Sounds;
use std::error::Error;
use crate::draw::Graphics;
use crate::input::Inputs;

pub trait Scene {
    fn init_scene(&mut self, graphics: &mut Graphics, sounds: &mut Sounds) -> SceneResult<()>;
    fn prepare_scene(&self, graphics: &mut Graphics);
    fn present_scene(&self, graphics: &mut Graphics);
    fn draw(&mut self, graphics: &mut Graphics);
    fn logic(&mut self, inputs: &Inputs, graphics: &mut Graphics, sounds: &mut Sounds) -> SceneResult<()>;
}

pub type SceneResult<T> = Result<T, Box<dyn Error>>;
