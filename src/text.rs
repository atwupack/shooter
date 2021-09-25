use crate::entity::EntityType;
use crate::entity::EntityType::Text;
use geemu_engine::draw::Graphics;

const GLYPH_HEIGHT: u32 = 28;
const GLYPH_WIDTH: u32 = 18;


pub(crate) fn init_fonts(graphics: &mut Graphics<EntityType>) {
    graphics.load_texture(Text, "gfx\\font.png");
}

pub(crate) fn draw_text(x: u32, y :u32, r: u8, g: u8, b: u8, text: &str, graphics: &mut Graphics<EntityType>) {

}

