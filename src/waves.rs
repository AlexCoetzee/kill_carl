use crate::enemy::{Enemies};
use crate::player::Player;
use crate::menu::Menu;
use macroquad::prelude::*;
use crate::shop::{Item, Shop, ShopTextures};
use crate::utilities;

pub(crate) struct Waves<'a> {
    pub waves: Wave,
    pub player: &'a mut Player,
    pub enemies: &'a mut Enemies,
    pub menu: &'a mut Menu,
    pub shop: &'a mut ShopDetails,
}

pub struct Wave {
    pub wave: u32,
    pub spawn_start: u32,
    pub spawn_increment: u32,
    pub duration: Timer,
}

pub struct ShopDetails {
    pub shop_items: Vec<Item>,
    pub shop_textures: ShopTextures,
    pub shop: Shop,
}

impl<'a> Waves<'a> {
    pub fn new(wave: u32, duration: f64, player: &'a mut Player, enemies: &'a mut Enemies, menu: &'a mut Menu, shop: &'a mut ShopDetails) -> Self {
        Waves {
            waves: Wave {
                wave,
                duration: Timer::new(duration),
                spawn_start: 20,
                spawn_increment: 5,
            },
            player,
            enemies,
            menu,
            shop
        }
    }

    pub async fn update(&mut self) {
        if self.waves.duration.has_elapsed() {
            self.waves.wave += 1;
            self.waves.duration.stop();
            self.reset_enemies();
            Item::shop_menu(&mut self.shop.shop_items, &mut self.player, &self.shop.shop_textures, &mut self.shop.shop).await;
            self.start_wave();
            self.waves.duration.start();
        } else {
            self.game_over().await;
            if !self.waves.duration.running {
                self.waves.duration.start()
            }
        }
    }

    fn reset_enemies(&mut self) {
        for enemy in  self.enemies.enemy_pool.iter_mut() {
            enemy.set_enemy_idle(self.player);
        }
    }

    fn start_wave(&mut self) {
        self.player.stats.health = self.player.stats.max_health;
        self.player.stats.money += 100;
        let mut spawn_count = 0;
        for enemy in self.enemies.enemy_pool.iter_mut() {
            if !enemy.elite && spawn_count < self.waves.spawn_start {
                enemy.set_enemy_active();
                spawn_count += 1;
            }
        }
        self.waves.spawn_start += self.waves.spawn_increment;
    }

    pub fn draw_game_status(&self) {
        let text = format!("Wave: {} - Time: {:.2}", self.waves.wave, self.waves.duration.time_left());
        let position = vec2(screen_width() / 2.0 - 200.0, 10.0);
        let text_pos = utilities::center_text(&text, 40, Vec2::new(400.0, 50.0), position);
        draw_rectangle(position.x, position.y, 400.0, 50.0, BLACK);
        draw_text(&text, text_pos.x, text_pos.y, 40.0, WHITE);
    }

    async fn game_over(&mut self) {
        if self.player.stats.health <= 0.0 {
            loop {
                self.player.position = vec2(-10000.0, -10000.0);
                draw_rectangle(0.0, 0.0, screen_width(), screen_height(), Color::new(0.0, 0.0, 0.0, 0.7));
                let text = "GAME OVER".to_string();
                let text_measurements = measure_text(&text, None, 100, 1.0);
                let text_pos = Vec2::new(screen_width() / 2.0 - (text_measurements.width / 2.0), screen_height() / 2.0 - (text_measurements.height / 2.0));
                draw_text(&text, text_pos.x, text_pos.y, 100.0, WHITE);

                let is_hover_quit = utilities::hovering_over(Vec2::new((screen_width() / 2.0) - 100.0, (screen_height() / 2.0) + 100.0), Vec2::new(200.0, 50.0));
                let quit_text = utilities::center_text("QUIT", 30, Vec2::new(200.0, 50.0), Vec2::new((screen_width() / 2.0) - 100.0, (screen_height() / 2.0) + 100.0));
                let btn_quit_color = if is_hover_quit { GRAY } else { BLUE };
                draw_rectangle((screen_width() / 2.0) - 100.0, (screen_height() / 2.0) + 100.0, 200.0, 50.0, btn_quit_color);
                draw_text("QUIT", quit_text.x, quit_text.y, 30.0, WHITE);

                if is_hover_quit && is_mouse_button_pressed(MouseButton::Left) {
                    std::process::exit(0);
                }

                next_frame().await;
            }
        }
    }
}

struct Timer {
    interval: f64,
    last_time: f64,
    running: bool,
}

impl Timer {
    fn new(interval: f64) -> Self {
        Timer {
            interval,
            last_time: get_time(),
            running: false,
        }
    }

    fn start(&mut self) {
        self.last_time = get_time();
        self.running = true;
    }

    fn stop(&mut self) {
        self.running = false;
    }

    fn has_elapsed(&mut self) -> bool {
        if !self.running {
            return false;  // If the timer is not running, don't check
        }

        let current_time = get_time();
        if current_time - self.last_time >= self.interval {
            self.running = false;  // Stop the timer when it elapses
            true
        } else {
            false
        }
    }

    fn time_left(&self) -> f64 {
        if !self.running {
            return 0.0;  // If the timer isn't running, return 0
        }

        let current_time = get_time();
        let time_passed = current_time - self.last_time;
        (self.interval - time_passed).max(0.0)  // Don't let the time go negative
    }
}