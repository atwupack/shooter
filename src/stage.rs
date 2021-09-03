use crate::defs::{
    ALIEN_BULLET_SPEED, FPS, PLAYER_BULLET_SPEED, PLAYER_SPEED, SCREEN_HEIGHT, SCREEN_WIDTH,
};
use crate::engine::draw::{Graphics, Textures};
use crate::engine::input::Inputs;
use crate::engine::scene::Scene;
use crate::engine::util::{collision, remove_or_apply, calc_slope};
use crate::entity::EntityType::{AlienBullet, Enemy, Player, PlayerBullet};
use crate::entity::{Entity, EntityBuilder, EntityType};
use rand::random;

pub struct Stage {
    enemies: Vec<Entity>,
    player_bullets: Vec<Entity>,
    enemy_bullets: Vec<Entity>,
    player: Entity,
    textures: Textures<EntityType>,
    enemy_spawn_timer: u32,
    stage_reset_timer: u32,
}

impl Scene for Stage {
    fn prepare_scene(&self, graphics: &mut Graphics) {
        graphics.set_draw_color(96, 128, 255, 255);
        graphics.clear();
    }

    fn present_scene(&self, graphics: &mut Graphics) {
        graphics.present();
    }

    fn draw(&mut self, graphics: &mut Graphics) {
        self.draw_player(graphics);
        self.draw_fighters(graphics);
        self.draw_bullets(graphics);
    }

    fn logic(&mut self, inputs: &Inputs) {
        self.do_player(inputs);
        self.do_bullets_hit_fighters();
        self.do_enemies();
        self.do_bullets();
        self.spawn_enemies();
        clip_entity_to_screen(&mut self.player);

        if self.player.health() <= 0 {
            self.stage_reset_timer -= 1;
            if self.stage_reset_timer <= 0 {
                self.reset_stage();
            }
        }
    }
}

impl Stage {


    pub(crate) fn init_stage(graphics: &Graphics) -> Self {
        let mut textures = Textures::new(graphics);
        // bullets
        textures.load_texture(PlayerBullet, "gfx\\playerBullet.png");
        // enemy
        textures.load_texture(Enemy, "gfx\\enemy.png");
        textures.load_texture(AlienBullet, "gfx\\alienBullet.png");
        // player
        textures.load_texture(Player, "gfx\\player.png");
        let player = init_player(&mut textures);

        Stage {
            enemies: Vec::new(),
            player_bullets: Vec::new(),
            enemy_bullets: Vec::new(),
            textures,
            player,
            enemy_spawn_timer: 1,
            stage_reset_timer: FPS * 2,
        }
    }

    fn reset_stage(&mut self) {
        self.enemies.clear();
        self.player_bullets.clear();
        self.enemy_bullets.clear();
        self.enemy_spawn_timer = 1;
        self.stage_reset_timer = FPS * 2;
        self.player = init_player(&mut self.textures);
    }

    fn fire_player_bullet(&mut self) {
        let (width, height) = self.textures.texture_size(EntityType::PlayerBullet);
        let bullet_x = self.player.x();
        let bullet_y =
            self.player.y() + (self.player.height() as f32 / 2.0) - (height as f32 / 2.0);
        let bullet = EntityBuilder::default()
            .x(bullet_x)
            .y(bullet_y)
            .dx(PLAYER_BULLET_SPEED)
            .width(width)
            .height(height)
            .entity_type(PlayerBullet)
            .health(1 as u32)
            .build()
            .unwrap();
        self.player_bullets.push(bullet);
        self.player.set_reload(8);
    }

    fn draw_bullets(&mut self, graphics: &mut Graphics) {
        for bullet in &self.player_bullets {
            self.textures.blit(
                graphics,
                bullet.entity_type(),
                bullet.x() as i32,
                bullet.y() as i32,
            )
        }

        for bullet in &self.enemy_bullets {
            self.textures.blit(
                graphics,
                bullet.entity_type(),
                bullet.x() as i32,
                bullet.y() as i32,
            )
        }
    }

    fn do_bullets(&mut self) {
        remove_or_apply(
            &mut self.player_bullets,
            |bullet| bullet.health() == 0 || bullet.x() > SCREEN_WIDTH as f32,
            |bullet| {
                bullet.apply_speed();
            },
        );
        remove_or_apply(
            &mut self.enemy_bullets,
            |bullet| bullet.health() == 0 || bullet.x() > SCREEN_WIDTH as f32,
            |bullet| {
                bullet.apply_speed();
            },
        );
    }

    fn do_bullets_hit_fighters(&mut self) {
        for fighter in &mut self.enemies {
            bullets_hit_fighter(&mut self.player_bullets, fighter);
        }

        bullets_hit_fighter(&mut self.enemy_bullets, &mut self.player);
    }

    fn do_enemies(&mut self) {
        remove_or_apply(
            &mut self.enemies,
            |fighter| (fighter.x() < -(fighter.width() as f32)) || fighter.health() == 0,
            |fighter| {
                fighter.apply_speed();
            },
        );

        if self.player.health() <= 0 {
            return;
        }
        for enemy in &mut self.enemies {
            enemy.dec_reload();
            if enemy.reload() <= 0 {
                self.enemy_bullets
                    .push(fire_enemy_bullet(enemy, &self.player, &self.textures));
            }
        }
    }

    fn do_player(&mut self, inputs: &Inputs) {
        if self.player.health() <= 0 {
            return;
        }

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
            self.fire_player_bullet();
        }

        self.player.apply_speed();
    }

    fn draw_player(&mut self, graphics: &mut Graphics) {
        if self.player.health() <= 0 {
            return;
        }
        self.textures.blit(
            graphics,
            self.player.entity_type(),
            self.player.x() as i32,
            self.player.y() as i32,
        )
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
                .reload(FPS * (1 + (random::<u32>() % 3)))
                .build()
                .unwrap();
            self.enemies.push(enemy);
            self.enemy_spawn_timer = 30 + (random::<u32>() % 60);
        }
    }

    fn draw_fighters(&mut self, graphics: &mut Graphics) {
        for fighter in &self.enemies {
            self.textures.blit(
                graphics,
                fighter.entity_type(),
                fighter.x() as i32,
                fighter.y() as i32,
            )
        }
    }
}

fn init_player(textures: &mut Textures<EntityType>) -> Entity {
    let (width, height) = textures.texture_size(Player);
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
    textures: &Textures<EntityType>,
) -> Entity {
    let (width, height) = textures.texture_size(EntityType::AlienBullet);
    let bullet_x = enemy.x() + (enemy.width() as f32 / 2.0) - (width as f32 / 2.0);
    let bullet_y = enemy.y() + (enemy.height() as f32 / 2.0) - (height as f32 / 2.0);

    enemy.set_reload(random::<u32>() % FPS * 2);

    let (slope_x, slope_y) = calc_slope(player.x() + (player.width() as f32 / 2.0), player.y() + (player.height() as f32 / 2.0), enemy.x(), enemy.y());

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

fn clip_entity_to_screen(entity: &mut Entity) {
    if entity.health() <= 0 {
        return;
    }
    if entity.x() < 0.0 {
        entity.set_x(0.0);
    }
    if entity.y() < 0.0 {
        entity.set_y(0.0);
    }
    if entity.x() > SCREEN_WIDTH as f32 / 2.0 {
        entity.set_x(SCREEN_WIDTH as f32 / 2.0);
    }
    if entity.y() > (SCREEN_HEIGHT - entity.height()) as f32 {
        entity.set_y((SCREEN_HEIGHT - entity.height()) as f32);
    }

}

