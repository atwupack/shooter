
use derive_builder::Builder;

#[derive(Default, Builder, Debug)]
#[builder(default)]
#[builder(setter(into))]
pub struct Explosion {
    pub x: f32,
    pub y: f32,
    pub dx: f32,
    pub dy: f32,
    r: u8,
    g: u8,
    b: u8,
    a: u8,
}


