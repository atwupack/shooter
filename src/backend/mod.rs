mod sdl2;

pub trait Backend {
    fn init(title: &str, width: u32, height:u32) -> Self;
}