use crate::engine::draw::Graphics;
use crate::engine::input::Inputs;

pub trait Scene {
    fn prepare_scene(&self, graphics: &mut Graphics);
    fn present_scene(&self, graphics: &mut Graphics);
    fn draw(&mut self, graphics: &mut Graphics);
    fn logic(&mut self, inputs: &Inputs);
}
