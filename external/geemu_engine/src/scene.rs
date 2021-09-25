use geemu_audio::Sounds;
use std::error::Error;
use crate::draw::Graphics;
use crate::input::Inputs;

pub trait Scene<T, S> {
    fn init_scene(&mut self, graphics: &mut Graphics<T>, sounds: &mut Sounds<S>) -> SceneResult<()>;
    fn prepare_scene(&self, graphics: &mut Graphics<T>);
    fn present_scene(&self, graphics: &mut Graphics<T>);
    fn draw(&mut self, graphics: &mut Graphics<T>);
    fn logic(&mut self, inputs: &Inputs, graphics: &mut Graphics<T>, sounds: &mut Sounds<S>) -> SceneResult<()>;
}

pub type SceneResult<T> = Result<T, Box<dyn Error>>;