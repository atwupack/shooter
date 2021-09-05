use crate::engine::draw::Graphics;
use crate::engine::input::Inputs;

pub trait Scene<T> {
    fn init_scene(&mut self, graphics: &mut Graphics<T>);
    fn prepare_scene(&self, graphics: &mut Graphics<T>);
    fn present_scene(&self, graphics: &mut Graphics<T>);
    fn draw(&mut self, graphics: &mut Graphics<T>);
    fn logic(&mut self, inputs: &Inputs, graphics: &mut Graphics<T>);
}
