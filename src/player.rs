use macroquad::audio::{Sound, play_sound, PlaySoundParams };
use macroquad::math::Vec2;
use macroquad::prelude::*;
use crate::sprite::Sprite;
use crate::Enemy;
use crate::money::Money;
use crate::utilities;

pub(crate) struct Player {
    pub position: Vec2,
    pub size: Vec2,
    pub sprite: Sprite,
    pub stats: Stats,
    pub weapon: Weapon,
}

pub struct Stats {
    pub base_attack_speed: f32,
    pub attack_speed_modifier: f32,
    pub health: f32,
    pub max_health: f32,
    pub movement_speed: f32,
    pub money: u32,
    pub level: Level,
    pub critical_chance: f32,
    pub critical_damage: f32,
    pub defense: u32
}

pub struct Weapon {
    pub count: usize,
    pub damage: f32,
    pub last_attack_time: f64,
    pub position: Vec2,
    pub direction: Vec2,
    pub can_shoot: bool,
    pub range: f32,
    pub damage_radius: f32,
    pub speed: f32,
    pub circle_radius: f32,
    pub aoe_count: usize,
}

pub(crate) struct Attack {
    pub texture: Texture2D,
    pub position: Vec2,
    pub direction: Vec2,
    pub speed: f32,
}

pub struct Level {
    pub level: u32,
    pub experience: f32,
    pub experience_to_next_level: f32,
}

struct FloatingText {
    text: String,
    size: f32,
    position: Vec2,
    velocity: Vec2,
    lifetime: f32,
    color: Color
}

pub(crate) struct DamageText {
    floating_texts: Vec<FloatingText>,
}

impl Player {
    pub async fn new() -> Self {
        let player = Sprite::new(load_texture("assets/player/player_s_1.png").await.unwrap(), Vec2::new(50.0,50.0), 8.0, 8.0, 1.0);
        Player {
            position: utilities::random_position((25.0..screen_width(), 25.0..screen_height())),
            size: Vec2::new(50.0,50.0),
            sprite: player,
            stats: Stats {
                base_attack_speed: 1.2,
                attack_speed_modifier: 1.0,
                health: 5.0,
                max_health: 10.0,
                movement_speed: 200.0,
                money: 1000,
                level: Level::new(0, 0.0, 20.0),
                critical_chance: 0.01,
                critical_damage: 0.01,
                defense: 0
            },
            weapon: Weapon {
                count: 3,
                damage: 10.0,
                last_attack_time: 0.0,
                position: vec2(10.0, 0.0),
                direction: vec2(1.0, 0.0),
                can_shoot: false,
                range: 300.0,
                damage_radius: 50.0,
                speed: 400.0,
                circle_radius: 40.0,
                aoe_count: 3,
            },
        }
    }

    pub  fn draw(&mut self) {
        self.sprite.draw(self.position);
    }

    pub async fn movement(&mut self) {

        let mut direction = Vec2::ZERO;

        // Update direction based on input
        if is_key_down(KeyCode::D) {
            direction.x += 1.0;
        }
        if is_key_down(KeyCode::A) {
            direction.x -= 1.0;
        }
        if is_key_down(KeyCode::W) {
            direction.y -= 1.0;
        }
        if is_key_down(KeyCode::S) {
            direction.y += 1.0;
        }

        // Normalize direction to have consistent speed
        if direction.length() > 0.0 {
            direction = direction.normalize();
        }

        // Update player position based on normalized direction and speed
        self.position += direction * self.stats.movement_speed * get_frame_time();

        // Keep the player within the screen bounds
        self.position.x = self.position.x.clamp(0.0, screen_width() - self.size.x);
        self.position.y = self.position.y.clamp(0.0, screen_height() - self.size.y);
    }

    pub fn draw_stats(&mut self) {
        //Health Bar
        Player::draw_player_bars(
            self.stats.health / self.stats.max_health,
            10.0,
            screen_width() / 5.0,
            40.0,
            GRAY,
            RED,
            WHITE,
            &format!("{:.0}/{:.0}", self.stats.health, self.stats.max_health),
        );
        //Experience Bar
        Player::draw_player_bars(
            self.stats.level.experience / self.stats.level.experience_to_next_level,
            60.0,
            screen_width() / 5.0,
            40.0,
            GRAY,
            GREEN,
            WHITE,
            &format!("LVL.{}", self.stats.level.level),
        );
        Player::draw_player_additional(self);
    }

    fn draw_player_bars(percentage: f32, bar_y_offset: f32, bar_width: f32, bar_height: f32, color_1: Color, color_2: Color,color_3: Color, message: &String) {
        draw_rectangle(10.0, bar_y_offset, bar_width, bar_height, color_1);
        draw_rectangle(10.0, bar_y_offset, bar_width * percentage, bar_height, color_2);
        let dimensions = draw_text(&message, -100.0, -100.0, 30.0, BLACK);
        draw_text(
            &message,
            bar_width / 2.0 - dimensions.width / 2.0,
            (bar_height + bar_y_offset) - 15.0,
            30.0,
            color_3,
        );
    }

    fn draw_player_additional(player: &mut Player) {
        let bar_width = screen_width() / 5.0;
        let bar_height = 170.0;
        let background_color = Color::new(0.0, 0.0, 0.0, 0.7);

        draw_rectangle(10.0, 110.0, bar_width, bar_height, background_color);

        struct Messages {
            message: String,
            color: Color,
            dimensions: TextDimensions,
            y_offset: f32
        }
        let mut messages: Vec<Messages> = Vec::new();

        messages.push(Messages {
            message: format!("Money: {}", player.stats.money),
            color: WHITE,
            dimensions: draw_text(&format!("Money: {}", player.stats.money), -100.0, -100.0, 30.0, BLACK),
            y_offset: 0.0
        });
        messages.push(Messages {
            message: format!("Critical Chance: {:.2}", player.stats.critical_chance * 100.0),
            color: WHITE,
            dimensions: draw_text(&format!("Critical Chance: {:.2}%", player.stats.critical_chance * 100.0), -100.0, -100.0, 30.0, BLACK),
            y_offset: 30.0
        });
        messages.push(Messages {
            message: format!("Critical Damage: {:.2}%", player.stats.critical_damage * 100.0),
            color: WHITE,
            dimensions: draw_text(&format!("Critical Damage: {:.2}", player.stats.critical_damage * 100.0), -100.0, -100.0, 30.0, BLACK),
            y_offset: 60.0
        });
        messages.push(Messages {
            message: format!("Attack Speed: {:.2}", player.attack_interval()),
            color: WHITE,
            dimensions: draw_text(&format!("Attack Speed: {:.2}", player.attack_interval()), -100.0, -100.0, 30.0, BLACK),
            y_offset: 90.0
        });
        messages.push(Messages {
            message: format!("Speed: {}", player.stats.movement_speed),
            color: WHITE,
            dimensions: draw_text(&format!("Speed: {}", player.stats.movement_speed), -100.0, -100.0, 30.0, BLACK),
            y_offset: 120.0
        });

        for message in messages.iter() {
            draw_text(
                &message.message,
                bar_width / 2.0 - message.dimensions.width / 2.0,
                message.y_offset + bar_height - 30.0,
                30.0,
                message.color,
            );
        }
    }

    pub fn find_closest_enemy<'a>(&self, enemies: &'a Vec<Enemy>) -> Option<&'a Enemy> {
        enemies.iter().min_by(|a, b| {
            let distance_a = self.position.distance(a.position);
            let distance_b = self.position.distance(b.position);
            distance_a.partial_cmp(&distance_b).unwrap()
        })
    }

    pub fn collect_coin(&mut self, money: &mut Money, attraction_radius: f32) {
        let distance = self.position.distance(money.position);
        if distance < attraction_radius {
            let direction = (self.position - money.position).normalize();
            money.position += direction * (self.stats.movement_speed / 50.0); // Speed of attraction

            // Check if the coin has reached the player
            if distance < 5.0 {
                self.stats.money += money.value;
                money.collected = true;
            }
        }
    }

    fn level_up(&mut self, experience: f32) {
        if self.stats.level.gain_experience(experience) {
            self.stats.max_health += 2.0;
            self.stats.health = self.stats.max_health;
            self.stats.attack_speed_modifier += 0.005;
            self.stats.movement_speed += 5.0;
            self.stats.critical_chance += 0.005;
            self.stats.critical_damage += 0.005;
        }
    }

    pub fn attack_interval(&self) -> f32 {
        1.0 / (self.stats.base_attack_speed * self.stats.attack_speed_modifier)
    }
}

impl Attack {
    pub fn new(player_pos: Vec2, direction: Vec2, speed: f32, texture: Texture2D) -> Self {
        Self {
            position: player_pos,
            direction,
            speed,
            texture,
        }
    }

    pub fn draw(&self) {
        if self.direction != Vec2::ZERO {
            draw_texture_ex(
                &self.texture,
                self.position.x - 10.0,
                self.position.y - 10.0,
                WHITE,
                DrawTextureParams {
                    dest_size: Some(Vec2::new(20.0, 20.0)),
                    ..Default::default()
                },
            );
        }
    }

    pub fn update(&mut self) {
        self.position += self.direction * self.speed * get_frame_time();
    }

    pub fn is_out_of_bounds(&self, map_width: f32, map_height: f32) -> bool {
        self.position.x < 0.0 || self.position.x > map_width || self.position.y < 0.0 || self.position.y > map_height
    }

    pub fn check_collision(&self, enemy: &Enemy) -> bool {
        self.position.distance(enemy.position) < (enemy.size.length() - 30.0)
    }

    pub fn attack(current_time: f64, player: &mut Player, bullets: &mut Vec<Attack>, enemies: &mut Vec<Enemy>, coins: &mut Vec<Money>, damage_text: &mut DamageText) {
        draw_circle_lines(player.position.x, player.position.y, player.weapon.circle_radius, 1.0, LIGHTGRAY);
        if current_time - player.weapon.last_attack_time > player.attack_interval() as f64 {
            if let Some(closest_enemy) = player.find_closest_enemy(&enemies) {
                if player.position.distance(closest_enemy.position) < player.weapon.range {
                    player.weapon.can_shoot = true;
                    player.weapon.last_attack_time = current_time;
                }
            }
        } else {
            player.weapon.can_shoot = false;
        }

        for bullet in bullets.iter_mut() {
            bullet.draw();
            bullet.update();
        }

        bullets.retain(|bullet| {
            if bullet.is_out_of_bounds(screen_width(), screen_height()) {
                return false;
            }
            for enemy in enemies.iter_mut() {
                if bullet.check_collision(enemy) {
                    return Attack::apply_area_of_effect_damage(enemies, bullet.position, coins, player, damage_text);
                }
            }
            true
        });
    }

    fn apply_area_of_effect_damage(enemies: &mut Vec<Enemy>, hit_position: Vec2, money: &mut Vec<Money>, player: &mut Player, damage_text: &mut DamageText) -> bool {
        let mut enemies_damaged: usize = 0;
        for enemy in enemies.iter_mut() {
            let distance = (enemy.position - hit_position).length();
            if enemy.active {
                if distance <= player.weapon.damage_radius {
                    if enemies_damaged == player.weapon.aoe_count {
                        break;
                    }
                    Attack::apply_damage(player, enemy, damage_text);
                    Attack::enemy_died(enemy, player, money);
                    enemies_damaged += 1;
                }
            }
        }
        false
    }

    fn apply_damage(player: &mut Player, enemy: &mut Enemy, damage_text: &mut DamageText) {
        let mut damage = player.weapon.damage;
        let mut font_size = 20.0;
        let mut message = format!("{}", damage);
        let mut color = WHITE;
        if utilities::is_critical_hit(player.stats.critical_chance) {
            font_size = 30.0;
            color = RED;
            damage = player.weapon.damage * (100.0 * player.stats.critical_damage);
            message = format!("{}!", damage as u32);
        }
        enemy.health -= damage;

        damage_text.add_floating_text(
            font_size,
            message,
            enemy.position,
            color,
        );
    }

    fn enemy_died(enemy: &mut Enemy, player: &mut Player, money: &mut Vec<Money>) {
        if enemy.health <= 0.0 {
            player.level_up(enemy.experience);
            money.push(Money {
                position: enemy.position,
                size: vec2(20.0, 20.0),
                value: utilities::random_number(1..=10) as u32,
                collected: false,
            });
            enemy.set_enemy_idle(player);
        }
    }

    pub fn weapon_system(player: &mut Player, closest_enemy: &Enemy, bullets: &mut Vec<Attack>, bullet_texture: &mut Texture2D, sound: &Sound) {
        let direction = (closest_enemy.position - player.position).normalize();
        let rotation_angle = direction.y.atan2(direction.x);

        let gun_position = Vec2::new(
            player.position.x  + player.weapon.circle_radius * rotation_angle.cos(),
            player.position.y + player.weapon.circle_radius * rotation_angle.sin(),
        );

        player.weapon.position = gun_position;
        player.weapon.direction = direction;

        draw_circle(gun_position.x, gun_position.y, 5.0, RED);
        draw_circle_lines(player.position.x, player.position.y, player.weapon.circle_radius, 2.0, BLACK);
        if player.position.distance(closest_enemy.position) < player.weapon.range {
            if player.weapon.can_shoot {
                play_sound(sound, PlaySoundParams { looped: false, volume: 0.001 });
                let bullet = Attack::new(player.weapon.position, player.weapon.direction, player.weapon.speed, bullet_texture.clone());
                bullets.push(bullet);
            }
        }
    }

    pub fn draw_weapon_system(enemies: &mut Vec<Enemy>, player: &mut Player, bullets: &mut Vec<Attack>, bullet_texture: &mut Texture2D, shoot_sound: &Sound) {

        let mut sorted_enemies = enemies;
        sorted_enemies.sort_by(|a, b| {
            player.position.distance(a.position)
                .partial_cmp(&player.position.distance(b.position))
                .unwrap()
        });

        for i in 0..player.weapon.count {
            if i < sorted_enemies.len() {
                Attack::weapon_system(player, &sorted_enemies[i], bullets, &mut bullet_texture.clone(), &shoot_sound);
            }
        }
    }
}



impl Level {
    pub fn new(level: u32, experience: f32, experience_to_next_level: f32) -> Self {
        Level {
            level,
            experience,
            experience_to_next_level,
        }
    }

    pub fn gain_experience(&mut self, experience: f32) -> bool {
        self.experience += experience;
        if self.experience >= self.experience_to_next_level {
            self.level += 1;
            self.experience -= self.experience_to_next_level;
            self.experience_to_next_level *= 1.1125;
            true
        } else {
            false
        }
    }
}



impl FloatingText {
    fn update(&mut self, dt: f32) {
        self.position += self.velocity * dt;
        self.lifetime -= dt;
    }

    fn draw(&self) {
        draw_text(
            &self.text,
            self.position.x,
            self.position.y,
            self.size,
            self.color,
        );
    }

    fn is_expired(&self) -> bool {
        self.lifetime <= 0.0
    }
}



impl DamageText {
    pub fn new() -> Self {
        Self {
            floating_texts: Vec::new(),
        }
    }

    fn add_floating_text(&mut self, size: f32, text: String, position: Vec2, color: Color) {
        let floating_text = FloatingText {
            text,
            size,
            position,
            // Move upwards
            velocity: Vec2::new(0.0, -50.0),
            lifetime: 1.0,
            color
        };
        self.floating_texts.push(floating_text);
    }

    pub fn update(&mut self, dt: f32) {
        for text in &mut self.floating_texts {
            text.update(dt);
        }
        self.floating_texts.retain(|text| !text.is_expired());
    }

    pub fn draw(&self) {
        for text in &self.floating_texts {
            text.draw();
        }
    }
}