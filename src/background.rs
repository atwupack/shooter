use derive_builder::Builder;
use crate::defs::{MAX_STARS, SCREEN_WIDTH, SCREEN_HEIGHT};
use rand::random;
use crate::engine::draw::Graphics;
use crate::entity::EntityType;

#[derive(Default, Debug)]
pub struct Background {
    background_x: i32,
    stars: Vec<Star>,
}

impl Background {
    pub(crate) fn init_starfield(&mut self) {
        self.stars.clear();
        for _i in 0..MAX_STARS {
            self.stars.push(StarBuilder::default()
                .x((random::<u32>() % SCREEN_WIDTH) as i32)
                .y((random::<u32>() % SCREEN_HEIGHT) as i32)
                .speed(1 + (random::<u8>() % 8))
                .build().unwrap());
        }
    }

    fn do_starfield(&mut self) {
        for star in &mut self.stars{
            star.x -= star.speed as i32;
            if star.x < 0 {
                star.x = SCREEN_WIDTH as i32 + star.x;
            }
        }
    }

    pub(crate) fn do_background(&mut self) {
        self.background_x -= 1;
        if self.background_x < -(SCREEN_WIDTH as i32) {
            self.background_x = 0;
        }

        self.do_starfield();
    }

    pub(crate) fn draw_background(&self, graphics: &mut Graphics<EntityType>) {
        for x in (self.background_x..SCREEN_WIDTH as i32).step_by(SCREEN_WIDTH as usize) {
            graphics.blit_size(EntityType::Background, x, 0, SCREEN_WIDTH, SCREEN_HEIGHT);
        }

        self.draw_starfield(graphics);
    }

    fn draw_starfield(&self, graphics: &mut Graphics<EntityType>) {
        for star in &self.stars {
            let c = 31 * star.speed;
            graphics.set_draw_color(c,c,c,255);
            graphics.draw_line(star.x, star.y, star.x +3, star.y);
        }
    }

}




#[derive(Default, Builder, Debug)]
#[builder(default)]
#[builder(setter(into))]
pub struct Star {
    pub x: i32,
    pub y: i32,
    pub speed: u8,
}
