use crate::defs::{PLAYER_BULLET_SPEED, PLAYER_SPEED, SCREEN_HEIGHT, SCREEN_WIDTH};
use crate::draw::Textures;
use crate::entity::EntityType::{AlienBullet, Enemy, Player, PlayerBullet};
use crate::entity::{Entity, EntityBuilder, EntityType};
use crate::input::Inputs;
use crate::util::{collision, remove_or_apply};
use rand::random;
use sdl2::render::WindowCanvas;

pub struct Stage {
    fighters: Vec<Entity>,
    bullets: Vec<Entity>,
    player: Entity,
    textures: Textures,
    enemy_spawn_timer: u32,
}

impl Stage {
    pub(crate) fn logic(&mut self, inputs: &Inputs) {
        self.do_player(inputs);
        self.do_bullets_hit_fighters();
        self.do_fighters();
        self.do_bullets();
        self.spawn_enemies();
    }

    pub(crate) fn draw(&mut self, canvas: &mut WindowCanvas) {
        self.draw_player(canvas);
        self.draw_bullets(canvas);
        self.draw_fighters(canvas);
    }

    pub(crate) fn init_stage(canvas: &WindowCanvas) -> Self {
        let mut textures = Textures::new(canvas);
        // bullets
        textures.load_texture(PlayerBullet, "gfx\\playerBullet.png");
        // enemy
        textures.load_texture(Enemy, "gfx\\enemy.png");
        textures.load_texture(AlienBullet, "gfx\\alienBullet.png");
        // player
        textures.load_texture(Player, "gfx\\player.png");
        let player = init_player(&mut textures);

        let mut stage = Stage {
            fighters: Vec::new(),
            bullets: Vec::new(),
            textures,
            player,
            enemy_spawn_timer: 1,
        };

        stage
    }

    fn fire_bullet(&mut self) {
        let (width, height) = self.textures.texture_size(EntityType::PlayerBullet);
        let bullet_y =
            self.player.y() + (self.player.height() as f32 / 2.0) - (height as f32 / 2.0);
        let bullet = EntityBuilder::default()
            .x(self.player.x())
            .y(bullet_y)
            .dx(PLAYER_BULLET_SPEED)
            .width(width)
            .height(height)
            .entity_type(PlayerBullet)
            .health(1 as u32)
            .build()
            .unwrap();
        self.bullets.push(bullet);
        self.player.set_reload(8);
    }

    fn draw_bullets(&mut self, canvas: &mut WindowCanvas) {
        for bullet in &self.bullets {
            self.textures.blit(
                canvas,
                bullet.entity_type(),
                bullet.x() as i32,
                bullet.y() as i32,
            )
        }
    }

    fn do_bullets(&mut self) {
        remove_or_apply(
            &mut self.bullets,
            |bullet| bullet.health() == 0 || bullet.x() > SCREEN_WIDTH as f32,
            |bullet| {
                bullet.apply_speed();
            },
        );
    }

    fn do_bullets_hit_fighters(&mut self) {
        for bullet in &mut self.bullets {
            for fighter in &mut self.fighters {
                if collision(
                    fighter.x() as i32,
                    fighter.y() as i32,
                    fighter.width() as i32,
                    fighter.height() as i32,
                    bullet.x() as i32,
                    bullet.y() as i32,
                    bullet.width() as i32,
                    bullet.height() as i32,
                ) {
                    fighter.set_health(0);
                    bullet.set_health(0);
                }
            }
        }
    }

    fn do_player(&mut self, inputs: &Inputs) {
        self.player.set_dx(0.0);
        self.player.set_dy(0.0);

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
        self.textures.blit(
            canvas,
            self.player.entity_type(),
            self.player.x() as i32,
            self.player.y() as i32,
        )
    }

    fn do_fighters(&mut self) {
        remove_or_apply(
            &mut self.fighters,
            |fighter| (fighter.x() < -(fighter.width() as f32)) || fighter.health() == 0,
            |fighter| {
                fighter.apply_speed();
            },
        );
    }

    fn spawn_enemies(&mut self) {
        self.enemy_spawn_timer -= 1;
        if self.enemy_spawn_timer <= 0 {
            let (width, height) = self.textures.texture_size(EntityType::Enemy);
            let speed = 2 + (random::<u32>() % 4);
            let enemy = EntityBuilder::default()
                .x(SCREEN_WIDTH as f32)
                .y((random::<u32>() % SCREEN_HEIGHT) as f32)
                .dx(-(speed as f32))
                .width(width)
                .height(height)
                .entity_type(Enemy)
                .health(1 as u32)
                .build()
                .unwrap();
            self.fighters.push(enemy);
            self.enemy_spawn_timer = 30 + (random::<u32>() % 60);
        }
    }

    fn draw_fighters(&mut self, canvas: &mut WindowCanvas) {
        for fighter in &self.fighters {
            self.textures.blit(
                canvas,
                fighter.entity_type(),
                fighter.x() as i32,
                fighter.y() as i32,
            )
        }
    }
}

fn init_player(textures: &mut Textures) -> Entity {
    let (width, height) = textures.texture_size(Player);
    let player = EntityBuilder::default()
        .x(100.0)
        .y(100.0)
        .width(width)
        .height(height)
        .entity_type(Player)
        .build()
        .unwrap();
    player
}
