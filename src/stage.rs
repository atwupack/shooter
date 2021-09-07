use crate::defs::{
    ALIEN_BULLET_SPEED, FPS, PLAYER_BULLET_SPEED, PLAYER_SPEED, SCREEN_HEIGHT, SCREEN_WIDTH,
};
use crate::engine::draw::Graphics;
use crate::engine::input::Inputs;
use crate::engine::scene::Scene;
use crate::engine::util::{calc_slope, collision, remove_or_apply};
use crate::entity::EntityType::{AlienBullet, Enemy, Player, PlayerBullet};
use crate::entity::{Entity, EntityBuilder, EntityType};
use rand::random;

pub struct Stage {
    enemies: Vec<Entity>,
    player_bullets: Vec<Entity>,
    enemy_bullets: Vec<Entity>,
    player: Option<Entity>,
    enemy_spawn_timer: u32,
    stage_reset_timer: u32,
}

impl Scene<EntityType> for Stage {
    fn init_scene(&mut self, graphics: &mut Graphics<EntityType>) {
        // bullets
        graphics.load_texture(PlayerBullet, "gfx\\playerBullet.png");
        // enemy
        graphics.load_texture(Enemy, "gfx\\enemy.png");
        graphics.load_texture(AlienBullet, "gfx\\alienBullet.png");
        // player
        graphics.load_texture(Player, "gfx\\player.png");

        let player = init_player(graphics);
        self.player = Some(player);
    }

    fn prepare_scene(&self, graphics: &mut Graphics<EntityType>) {
        graphics.set_draw_color(96, 128, 255, 255);
        graphics.clear();
    }

    fn present_scene(&self, graphics: &mut Graphics<EntityType>) {
        graphics.present();
    }

    fn draw(&mut self, graphics: &mut Graphics<EntityType>) {
        draw_entities(&self.player, graphics);
        draw_entities(&self.enemies, graphics);
        draw_entities(&self.player_bullets, graphics);
        draw_entities(&self.enemy_bullets, graphics);
    }

    fn logic(&mut self, inputs: &Inputs, graphics: &mut Graphics<EntityType>) {
        self.do_player(inputs, graphics);
        self.do_bullets_hit_fighters();
        self.do_enemies(graphics);
        self.do_bullets();
        self.spawn_enemies(graphics);
        if let Some(player) = &mut self.player {
            clip_entity_to_screen(player);
        } else {
            self.stage_reset_timer -= 1;
            if self.stage_reset_timer <= 0 {
                self.reset_stage(graphics);
            }
        }
    }
}

impl Default for Stage {
    fn default() -> Self {
        Stage {
            enemies: Vec::new(),
            player_bullets: Vec::new(),
            enemy_bullets: Vec::new(),
            player: None,
            enemy_spawn_timer: 1,
            stage_reset_timer: FPS * 2,
        }
    }
}

impl Stage {
    fn reset_stage(&mut self, graphics: &mut Graphics<EntityType>) {
        self.enemies.clear();
        self.player_bullets.clear();
        self.enemy_bullets.clear();
        self.enemy_spawn_timer = 1;
        self.stage_reset_timer = FPS * 2;
        self.player = Some(init_player(graphics));
    }

    fn do_bullets(&mut self) {
        remove_or_apply(
            &mut self.player_bullets,
            |bullet| bullet.health == 0 || is_outside_screen(bullet),
            |bullet| {
                bullet.apply_speed();
            },
        );
        remove_or_apply(
            &mut self.enemy_bullets,
            |bullet| bullet.health == 0 || is_outside_screen(bullet),
            |bullet| {
                bullet.apply_speed();
            },
        );
    }

    fn do_bullets_hit_fighters(&mut self) {
        for fighter in &mut self.enemies {
            bullets_hit_fighter(&mut self.player_bullets, fighter);
        }
        if let Some(player) = &mut self.player {
            bullets_hit_fighter(&mut self.enemy_bullets, player);
            if player.health <= 0 {
                self.player = None;
            }
        }
    }

    fn do_enemies(&mut self, graphics: &Graphics<EntityType>) {
        remove_or_apply(
            &mut self.enemies,
            |fighter| is_outside_screen(fighter) || fighter.health == 0,
            |fighter| {
                fighter.apply_speed();
            },
        );

        if let Some(player) = &self.player {
            for enemy in &mut self.enemies {
                if enemy.reload_done() {
                    self.enemy_bullets
                        .push(fire_enemy_bullet(enemy, player, graphics));
                }
            }
        }
    }

    fn do_player(&mut self, inputs: &Inputs, graphics: &mut Graphics<EntityType>) {
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
                self.player_bullets
                    .push(fire_player_bullet(player, graphics));
            }

            player.apply_speed();
        }
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

fn draw_entities<'a>(
    entities: impl IntoIterator<Item = &'a Entity>,
    graphics: &mut Graphics<EntityType>,
) {
    for entity in entities {
        graphics.blit(entity.entity_type(), entity.x as i32, entity.y as i32)
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
) -> Entity {
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

    EntityBuilder::default()
        .x(bullet_x)
        .y(bullet_y)
        .dx(slope_x * ALIEN_BULLET_SPEED)
        .dy(slope_y * ALIEN_BULLET_SPEED)
        .width(width)
        .height(height)
        .entity_type(AlienBullet)
        .health(1 as u32)
        .build()
        .unwrap()
}

fn bullets_hit_fighter(bullets: &mut Vec<Entity>, fighter: &mut Entity) {
    for bullet in bullets {
        if collision(
            fighter.x as i32,
            fighter.y as i32,
            fighter.width() as i32,
            fighter.height() as i32,
            bullet.x as i32,
            bullet.y as i32,
            bullet.width() as i32,
            bullet.height() as i32,
        ) {
            fighter.health = 0;
            bullet.health = 0;
        }
    }
}

fn clip_entity_to_screen(entity: &mut Entity) {
    if entity.health <= 0 {
        return;
    }
    entity.restrict_position(0.0, 0.0, SCREEN_WIDTH as f32 / 2.0, (SCREEN_HEIGHT - entity.height()) as f32);
}

fn fire_player_bullet(player: &mut Entity, graphics: &mut Graphics<EntityType>) -> Entity {
    let (width, height) = graphics.texture_size(EntityType::PlayerBullet);
    let bullet_x = player.x;
    let bullet_y = player.y + (player.height() as f32 / 2.0) - (height as f32 / 2.0);
    player.set_reload(8);
    EntityBuilder::default()
        .x(bullet_x)
        .y(bullet_y)
        .dx(PLAYER_BULLET_SPEED)
        .width(width)
        .height(height)
        .entity_type(PlayerBullet)
        .health(1 as u32)
        .build()
        .unwrap()
}

fn is_outside_screen(entity: &Entity) -> bool {
    (entity.x < -(entity.width() as f32)) || (entity.y < -(entity.height() as f32)) ||  entity.x > SCREEN_WIDTH as f32 || entity.y > SCREEN_HEIGHT as f32
}
