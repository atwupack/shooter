use crate::entity::Entity;
use crate::draw::Textures;
use sdl2::render::WindowCanvas;
use crate::input::Inputs;

const PLAYER_SPEED: i32 = 4;
const PLAYER_BULLET_SPEED: i32 = 16;

pub struct Stage {
    fighters: Vec<Entity>,
    bullets: Vec<Entity>,
    player: Entity,
    textures: Textures,
}

impl Stage {
    pub(crate) fn logic(&mut self, inputs: &Inputs) {
        self.do_player(inputs);
        self.do_bullets();
    }

    pub(crate) fn draw(&mut self, canvas: &mut WindowCanvas) {
        self.draw_player(canvas);
        self.draw_bullets(canvas)
    }

    pub fn init_stage(canvas: &WindowCanvas) -> Self {
        let mut textures = Textures::new(canvas);
        // bullets
        textures.load_texture("bullet","gfx\\playerBullet.png");
        // player
        let player = init_player(&mut textures);

        Stage {
            fighters: Vec::new(),
            bullets: Vec::new(),
            textures,
            player,
        }
    }

    fn fire_bullet(&mut self) {
        let (width, height) = self.textures.texture_size("bullet");
        let bullet_y = self.player.y() + (self.player.height() as i32 / 2) - (height as i32 / 2);
        let bullet = Entity::new(self.player.x(), bullet_y, PLAYER_BULLET_SPEED,0, width, height, 1,"bullet");
        self.bullets.push(bullet);
        self.player.set_reload(8);
    }

    fn draw_bullets(&mut self, canvas: &mut WindowCanvas) {
        for bullet in &self.bullets {
            self.textures.blit(canvas, bullet.texture(), bullet.x(), bullet.y())
        }
    }

    fn do_bullets(&mut self) {

        let mut i = 0;
        while i < self.bullets.len() {
            let bullet = &mut self.bullets[i];
            if bullet.x() > 1280 {
                let _val = self.bullets.remove(i);
            } else {
                bullet.apply_speed();
                i += 1;
            }
        }
    }

    fn do_player(&mut self, inputs: &Inputs) {

        self.player.set_dx(0);
        self.player.set_dy(0);

        self.player.dec_reload();

        if inputs.up() {
            self.player.set_dy(-PLAYER_SPEED);
        }
        if inputs.down() {
            self.player.set_dy(PLAYER_SPEED);
        }
        if inputs.left() {
            self.player.set_dx(-PLAYER_SPEED);
        }
        if inputs.right() {
            self.player.set_dx(PLAYER_SPEED);
        }
        if inputs.fire() && self.player.reload() == 0 {
            self.fire_bullet();
        }

        self.player.apply_speed();
    }

    fn draw_player(&mut self, canvas: &mut WindowCanvas) {
        self.textures.blit(canvas, self.player.texture(), self.player.x(), self.player.y())
    }


}

fn init_player(textures: &mut Textures) -> Entity {
    textures.load_texture("player", "gfx\\player.png");
    let (width, height) = textures.texture_size("player");
    let player = Entity::new(100, 100, 0,0, width, height, 16, "player");
    player
}