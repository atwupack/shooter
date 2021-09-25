use crate::defs::{ALIEN_BULLET_SPEED, FPS, PLAYER_BULLET_SPEED, PLAYER_SPEED, SCREEN_HEIGHT, SCREEN_WIDTH};
use crate::engine::draw::Graphics;
use crate::engine::input::Inputs;
use crate::engine::scene::{Scene, SceneResult};
use crate::engine::util::{calc_slope, collision, remove_or_apply};
use crate::entity::EntityType::{AlienBullet, Enemy, Player, PlayerBullet};
use crate::entity::{Entity, EntityBuilder, EntityType};
use rand::random;
use crate::engine::traits::{HasVelocity, IsRendered, HasPosition};
use crate::entity::explosion::{Explosion, do_explosions, draw_explosions, add_explosions};
use crate::util::{is_outside_screen, draw_entities};
use crate::background::Background;
use crate::entity::debris::{Debris, add_debris, draw_debris, do_debris};
use crate::entity::bullet::{Bullet, do_bullets, BulletType};
use crate::entity::bullet::{BulletType::EnemyBullet, BulletBuilder};
use crate::sound::SoundType::{PlayerFire, AlienFire, PlayerDie, AlienDie};
use crate::sound::SoundType;
use crate::text::{draw_text, init_fonts};
use geemu_audio::Sounds;
use std::error::Error;

pub struct Stage {
    enemies: Vec<Entity>,
    player_bullets: Vec<Bullet>,
    enemy_bullets: Vec<Bullet>,
    player: Option<Entity>,
    explosions: Vec<Explosion>,
    debris: Vec<Debris>,
    enemy_spawn_timer: u32,
    stage_reset_timer: u32,
    background: Background,
    score: u32,
    high_score: u32,
}

impl Scene<EntityType, SoundType> for Stage {
    fn init_scene(&mut self, graphics: &mut Graphics<EntityType>, sounds: &mut Sounds<SoundType>) -> SceneResult<()> {
        // bullets
        graphics.load_texture(PlayerBullet, "gfx\\playerBullet.png");
        // enemy
        graphics.load_texture(Enemy, "gfx\\enemy.png");
        graphics.load_texture(AlienBullet, "gfx\\alienBullet.png");
        // player
        graphics.load_texture(Player, "gfx\\player.png");

        graphics.load_texture(EntityType::Background, "gfx\\background.png");
        graphics.load_texture(EntityType::Explosion, "gfx\\explosion.png");

        sounds.load_sound(PlayerFire, "sound\\334227__jradcoolness__laser.ogg")?;
        sounds.load_sound(AlienFire, "sound\\196914__dpoggioli__laser-gun.ogg")?;
        sounds.load_sound(PlayerDie, "sound\\245372__quaker540__hq-explosion.ogg")?;
        sounds.load_sound(AlienDie, "sound\\10 Guage Shotgun-SoundBible.com-74120584.ogg")?;

        sounds.play_music("music\\Mercury.ogg")?;

        init_fonts(graphics);

        let player = init_player(graphics);
        self.player = Some(player);
        self.background.init_starfield();
        Ok(())
    }

    fn prepare_scene(&self, graphics: &mut Graphics<EntityType>) {
        graphics.set_draw_color(96, 128, 255, 255);
        graphics.clear();
    }

    fn present_scene(&self, graphics: &mut Graphics<EntityType>) {
        graphics.present();
    }

    fn draw(&mut self, graphics: &mut Graphics<EntityType>) {
        self.background.draw_background(graphics);
        draw_entities(&self.player, graphics);
        draw_entities(&self.enemies, graphics);
        draw_debris(&self.debris, graphics);
        draw_explosions(&self.explosions, graphics);
        draw_entities(&self.player_bullets, graphics);
        draw_entities(&self.enemy_bullets, graphics);
        draw_hud(self.score, self.high_score, graphics);
    }

    fn logic(&mut self, inputs: &Inputs, graphics: &mut Graphics<EntityType>, sounds: &mut Sounds<SoundType>) -> SceneResult<()> {
        self.background.do_background();
        self.do_player(inputs, graphics, sounds)?;
        self.do_bullets_hit_fighters(sounds)?;
        self.do_enemies(graphics, sounds)?;
        self.do_bullets();
        do_debris(&mut self.debris);
        do_explosions(&mut self.explosions);

        self.spawn_enemies(graphics);
        if let Some(player) = &mut self.player {
            clip_entity_to_screen(player);
        } else {
            self.stage_reset_timer -= 1;
            if self.stage_reset_timer <= 0 {
                self.reset_stage(graphics);
            }
        }
        Ok(())
    }
}

impl Default for Stage {
    fn default() -> Self {
        Stage {
            enemies: Vec::new(),
            player_bullets: Vec::new(),
            enemy_bullets: Vec::new(),
            explosions: Vec::new(),
            debris: Vec::new(),
            player: None,
            enemy_spawn_timer: 1,
            stage_reset_timer: FPS * 2,
            background: Background::default(),
            score: 0,
            high_score: 0,
        }
    }
}

impl Stage {

    fn reset_stage(&mut self, graphics: &mut Graphics<EntityType>) {
        self.enemies.clear();
        self.player_bullets.clear();
        self.enemy_bullets.clear();
        self.explosions.clear();
        self.debris.clear();
        self.enemy_spawn_timer = 1;
        self.stage_reset_timer = FPS * 3;
        self.player = Some(init_player(graphics));
        self.background.init_starfield();
        self.score = 0;
    }


    fn do_bullets(&mut self) {
        do_bullets(&mut self.player_bullets);
        do_bullets(&mut self.enemy_bullets);
    }

    fn do_bullets_hit_fighters(&mut self, sounds: &mut Sounds<SoundType>) -> SceneResult<()> {
        for fighter in &mut self.enemies {
            bullets_hit_fighter(&mut self.player_bullets, fighter);
            if fighter.health <= 0 {
                add_explosions(&mut self.explosions, fighter.x, fighter.y, 32);
                add_debris(fighter, &mut self.debris);
                sounds.play_sound(&AlienDie)?;
                self.score += 1;
                self.high_score = self.score.max(self.high_score);
            }
        }
        if let Some(player) = &mut self.player {
            bullets_hit_fighter(&mut self.enemy_bullets, player);
            if player.health <= 0 {
                add_explosions(&mut self.explosions, player.x, player.y, 32);
                add_debris(player, &mut self.debris);
                sounds.play_sound(&PlayerDie)?;
                self.player = None;
            }
        }
        Ok(())
    }

    fn do_enemies(&mut self, graphics: &Graphics<EntityType>, sounds: &mut Sounds<SoundType>) -> SceneResult<()> {
        remove_or_apply(
            &mut self.enemies,
            |fighter| is_outside_screen(fighter) || fighter.health == 0,
            |fighter| {
                fighter.apply_velocity();
            },
        );

        if let Some(player) = &self.player {
            for enemy in &mut self.enemies {
                if enemy.reload_done() {
                    sounds.play_sound(&AlienFire)?;
                    self.enemy_bullets
                        .push(fire_enemy_bullet(enemy, player, graphics));
                }
            }
        }
        Ok(())
    }

    fn do_player(&mut self, inputs: &Inputs, graphics: &mut Graphics<EntityType>, sounds: &mut Sounds<SoundType>) -> Result<(), Box<dyn Error>> {
        if let Some(player) = &mut self.player {
            player.dx = 0.0;
            player.dy = 0.0;

            if inputs.up() {
                player.dy = -PLAYER_SPEED;
            }
            if inputs.down() {
                player.dy = PLAYER_SPEED;
            }
            if inputs.left() {
                player.dx = -PLAYER_SPEED;
            }
            if inputs.right() {
                player.dx = PLAYER_SPEED;
            }

            if inputs.fire() && player.reload_done() {
                sounds.play_sound(&PlayerFire)?;
                self.player_bullets
                    .push(fire_player_bullet(player, graphics));
            }

            player.apply_velocity();
        }
        Ok(())
    }

    fn spawn_enemies(&mut self, graphics: &Graphics<EntityType>) {
        self.enemy_spawn_timer -= 1;
        if self.enemy_spawn_timer <= 0 {
            let (width, height) = graphics.texture_size(EntityType::Enemy);
            let speed = 2 + (random::<u32>() % 4);
            let enemy = EntityBuilder::default()
                .x(SCREEN_WIDTH as f32)
                .y((random::<u32>() % SCREEN_HEIGHT) as f32)
                .dx(-(speed as f32))
                .width(width)
                .height(height)
                .entity_type(Enemy)
                .health(1 as u32)
                .reload(FPS * (1 + (random::<u32>() % 3)))
                .build()
                .unwrap();
            self.enemies.push(enemy);
            self.enemy_spawn_timer = 30 + (random::<u32>() % 60);
        }
    }
}

fn init_player(graphics: &mut Graphics<EntityType>) -> Entity {
    let (width, height) = graphics.texture_size(Player);
    EntityBuilder::default()
        .x(100.0)
        .y(100.0)
        .width(width)
        .height(height)
        .entity_type(Player)
        .health(1 as u32)
        .build()
        .unwrap()
}

fn fire_enemy_bullet(
    enemy: &mut Entity,
    player: &Entity,
    graphics: &Graphics<EntityType>,
) -> Bullet {
    let (width, height) = graphics.texture_size(EntityType::AlienBullet);
    let bullet_x = enemy.x + (enemy.width() as f32 / 2.0) - (width as f32 / 2.0);
    let bullet_y = enemy.y + (enemy.height() as f32 / 2.0) - (height as f32 / 2.0);

    enemy.set_reload(random::<u32>() % FPS * 2);

    let (slope_x, slope_y) = calc_slope(
        player.x + (player.width() as f32 / 2.0),
        player.y + (player.height() as f32 / 2.0),
        enemy.x,
        enemy.y,
    );

    BulletBuilder::default()
        .x(bullet_x)
        .y(bullet_y)
        .dx(slope_x * ALIEN_BULLET_SPEED)
        .dy(slope_y * ALIEN_BULLET_SPEED)
        .width(width)
        .height(height)
        .bullet_type(EnemyBullet)
        .has_hit(false)
        .build()
        .unwrap()
}

fn bullets_hit_fighter(bullets: &mut Vec<Bullet>, fighter: &mut Entity) {
    for bullet in bullets {
        if collision(
            fighter.x as i32,
            fighter.y as i32,
            fighter.width() as i32,
            fighter.height() as i32,
            bullet.x() as i32,
            bullet.y() as i32,
            bullet.width() as i32,
            bullet.height() as i32,
        ) {
            fighter.health = 0;
            bullet.has_hit= true;
        }
    }
}

fn clip_entity_to_screen(entity: &mut Entity) {
    if entity.health <= 0 {
        return;
    }
    entity.restrict_position(0.0, 0.0, SCREEN_WIDTH as f32 / 2.0, (SCREEN_HEIGHT - entity.height()) as f32);
}

fn fire_player_bullet(player: &mut Entity, graphics: &mut Graphics<EntityType>) -> Bullet {
    let (width, height) = graphics.texture_size(EntityType::PlayerBullet);
    let bullet_x = player.x;
    let bullet_y = player.y + (player.height() as f32 / 2.0) - (height as f32 / 2.0);
    player.set_reload(8);
    BulletBuilder::default()
        .x(bullet_x)
        .y(bullet_y)
        .dx(PLAYER_BULLET_SPEED)
        .width(width)
        .height(height)
        .bullet_type(BulletType::PlayerBullet)
        .has_hit(false)
        .build()
        .unwrap()
}

fn draw_hud(score: u32, high_score: u32, graphics: &mut Graphics<EntityType>) {
    let score_str = format!("SCORE: {}", score);
    draw_text(10, 10, 255, 255, 255, score_str.as_str(), graphics);

    let high_score_str = format!("HIGH SCORE: {}", high_score);
    if score > 0 && score == high_score {
        draw_text(960, 10, 0, 255, 0, high_score_str.as_str(), graphics);
    }
    else {
        draw_text(960, 10, 255, 255, 255, high_score_str.as_str(), graphics);
    }
}



