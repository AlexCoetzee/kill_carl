use macroquad::math::f32;
use macroquad::prelude::*;
use crate::player::Player;
use crate::sprite::Sprite;
use crate::utilities;

#[derive(Clone)]
pub(crate) struct Enemies {
    pub enemy_pool: Vec<Enemy>,
    pub last_enemy_spawn_time: f64,
    pub enemy_spawn_rate: f64,
}

#[derive(Clone)]
pub(crate) struct Enemy {
    pub attack_speed: f64,
    pub elite: bool,
    pub damage: f32,
    pub active: bool,
    base_health: f32,
    pub health: f32,
    pub last_attack_time: f64,
    pub position: Vec2,
    pub movement_speed: f32,
    pub size: Vec2,
    pub sprite: Sprite,
    pub experience: f32,
}

#[derive(Debug, Clone)]
pub struct EnemyTextureDetails {
    columns: f32,
    rows: f32,
    frames: f32,
}

#[derive(Clone)]
pub struct EnemyTextures {
    pub texture: Texture2D,
    pub details: EnemyTextureDetails
}

impl Enemy {
    pub async fn new(movement_speed: f32, health: f32, damage: f32, elite: bool, size: Vec2, active: bool, attack_speed: f64, texture_details: EnemyTextures) -> Self {
        let enemy_sprite = Sprite::new(texture_details.texture, Vec2::new(100.0, 100.0), texture_details.details.frames, texture_details.details.columns, texture_details.details.rows);

        Enemy {
            attack_speed,
            elite,
            damage,
            health,
            base_health: health,
            last_attack_time: 0.0,
            active,
            position: Enemy::spawn_position(active),
            movement_speed,
            size,
            sprite: enemy_sprite,
            experience: utilities::random_number(1..=10) as f32,
        }
    }

    pub fn update(enemies: &mut Vec<Enemy>, player: &mut Player) {
        let enemies_len = enemies.len();

        for i in 0..enemies_len {
            // Split the enemies list into two parts: one part for the current enemy (mutable) and the rest (mutable)
            let (before, after) = enemies.split_at_mut(i);
            let (enemy, after) = after.split_first_mut().unwrap(); // Get the current enemy

            // Combine the parts before and after the current enemy to pass as an immutable slice
            let rest_of_enemies = before.iter().chain(after.iter());

            // Collect the rest of enemies as a Vec of mutable references
            let rest_of_enemies: Vec<&Enemy> = rest_of_enemies.collect();

            // Call movement with the rest of the enemies
            Enemy::movement(enemy, player, rest_of_enemies, 30.0);
            Enemy::attack(enemy, player);
            enemy.sprite.update();
            enemy.draw();
        }
    }

    pub  fn draw(&mut self) {
        if self.active {
            self.sprite.size = self.size;
            Sprite::draw(&self.sprite, self.position);
        }
    }

    pub fn movement(enemy: &mut Enemy, player: &Player, enemies: Vec<&Enemy>, separation_radius: f32) {
        if enemy.active {
            let mut direction_to_player = (player.position - enemy.position).normalize();

            // Separation logic
            let mut separation_force = Vec2::ZERO;
            let mut neighbors_count = 0;

            for other_enemy in enemies {
                if enemy.position.distance(other_enemy.position) < separation_radius && enemy.position != other_enemy.position {
                    // Calculate separation vector
                    let away_from_other = (enemy.position - other_enemy.position).normalize();
                    separation_force += away_from_other;
                    neighbors_count += 1;
                }
            }

            // Average out the separation force from nearby enemies
            if neighbors_count > 0 {
                separation_force /= neighbors_count as f32;
            }

            // Combine the movement towards player with the separation force
            let combined_direction = direction_to_player + separation_force;
            let final_direction = combined_direction.normalize();

            // Apply movement
            enemy.position += final_direction * enemy.movement_speed * get_frame_time();
        }
    }

    fn is_colliding(enemy: &Enemy, player: &Player) -> bool {
        let player_rect = Rect::new(player.position.x, player.position.y, 20.0, 20.0);
        let enemy_rect = Rect::new(enemy.position.x, enemy.position.y, 50.0, 50.0);
        player_rect.overlaps(&enemy_rect)
    }

    pub fn attack(&mut self, player: &mut Player) {
        if Enemy::is_colliding(&self, &player) && get_time() - self.last_attack_time > self.attack_speed && player.stats.health > 0.0 && self.active {
            player.stats.health -= self.damage;
            self.last_attack_time = get_time();
        }
    }

    pub async fn enemy_collection(current_type: f64, enemies: &mut Enemies, player: &Player) {
        let mut spawn_rate = 1.0;
        if player.stats.level.level > 1 {
            spawn_rate = enemies.enemy_spawn_rate / (player.stats.level.level as f64);
        }
        if current_type - enemies.last_enemy_spawn_time > spawn_rate {
            enemies.last_enemy_spawn_time = current_type;
            for enemy in enemies.enemy_pool.iter_mut() {
                if !enemy.active {
                    enemy.active = true;
                    enemy.position = Enemy::spawn_position(true);
                    break;
                }
            }
        }
    }

    pub async fn enemy_textures() -> Vec<EnemyTextures> {
        let mut textures: Vec<EnemyTextures> = Vec::new();
        for i in 1..=4 {
            textures.push(EnemyTextures {
                texture: load_texture(&format!("assets/enemies/enemy_s_{}.png", i)).await.unwrap(),
                details: Enemy::enemy_texture_details(i),
            });
        }
        textures
    }

    fn enemy_texture_details(index: u32) -> EnemyTextureDetails {
        if index == 1 {
            EnemyTextureDetails { columns: 8.0, rows: 1.0, frames: 8.0 }
        } else if index == 2 {
            EnemyTextureDetails { columns: 8.0, rows: 1.0, frames: 8.0 }
        } else if index == 3 {
            EnemyTextureDetails { columns: 6.0, rows: 1.0, frames: 6.0 }
        } else {
            EnemyTextureDetails { columns: 8.0, rows: 1.0, frames: 8.0 }
        }
    }

    pub async fn spawn_enemy_pool(enemy_textures: Vec<EnemyTextures>) -> Enemies {
        let mut enemies = vec![];
        let mut count = 0;
        let mut texture_count = 0;
        loop {
            let enemy_speed = utilities::random_number(150..=200) as f32;
            /*if count < 20 {
                enemies.push(Enemy::new(enemy_speed, 100.0, 5.0, true, vec2(50.0, 50.0), false, 1.0, enemy_textures[texture_count].clone()).await);
            }*/
            if count < 20 {
                enemies.push(Enemy::new(enemy_speed, 30.0, 1.0, false, vec2(50.0, 50.0), true, 1.5, enemy_textures[texture_count].clone()).await);
            } else {
                enemies.push(Enemy::new(enemy_speed, 30.0, 1.0, false, vec2(50.0, 50.0), false, 1.5, enemy_textures[texture_count].clone()).await);
            }
            if texture_count < 3 {
                texture_count += 1;
            } else {
                texture_count = 0;
            }
            count += 1;
            if count == 300 {
                break;
            }
        }

        for enemy in enemies.iter_mut() {
            enemy.sprite.update();
            enemy.draw();
        }
        Enemies {
            enemy_pool: enemies,
            last_enemy_spawn_time: get_time(),
            enemy_spawn_rate: 2.0,
        }
    }

    pub fn set_enemy_idle(&mut self, player: &Player) {
        self.active = false;
        self.position = Vec2::new(-100.0, -100.0);
        self.health = (player.weapon.damage * player.stats.critical_damage) / 100.0 * 125.0;
        self.damage = player.stats.health / 100.0 * 30.0;
        self.last_attack_time = 0.0;
    }

    pub fn set_enemy_active(&mut self) {
        self.active = true;
        self.position = Enemy::spawn_position(true);
    }

    fn spawn_position(active: bool) -> Vec2 {
        if active {
            utilities::random_spawn_position(screen_width(), screen_height())
        } else {
            vec2(-100.0, -100.0)
        }
    }
}