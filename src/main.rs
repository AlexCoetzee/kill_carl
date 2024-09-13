mod player;
mod enemy;
mod background;
mod utilities;
mod sprite;
mod camera;
mod menu;
mod money;
mod debugging;
mod shop;
mod waves;

use macroquad::prelude::*;
use macroquad::audio::*;
use player::Player;
use player::Attack;
use player::DamageText;
use enemy::Enemy;
use camera::Camera;
use menu::Menu;
use money::Money;
use shop::Item;
use shop::Shop;
use waves::{Waves, ShopDetails};
use crate::shop::textures;


#[macroquad::main("Kill Carl!")]
async fn main() {
    set_fullscreen(true);

    /*################## Textures ###############*/
    let enemy_textures = Enemy::enemy_textures().await;
    let shop_textures = textures::get_textures().await;
    let background_texture = load_texture("assets/background/background.png").await.unwrap();
    let bullet_texture = load_texture("assets/bullets/bullet.png").await.unwrap();
    /*###########################################*/

    /*################## Sound ###############*/
    let shoot_sound = load_sound("assets/sound/shoot.wav").await.unwrap();
    /*###########################################*/

    /*################## Loading Screen ###############*/
    let mut menu = Menu::new().await;
    menu.menu().await;
    /*###########################################*/

    let mut shop = Shop::new();
    let mut player = Player::new().await;
    let mut enemies = Enemy::spawn_enemy_pool(enemy_textures).await;
    let mut shop_items = Item::shop(player.stats.level.level);
    let mut shop_details = ShopDetails {
        shop_items,
        shop_textures,
        shop,
    };
    let mut waves = Waves::new(1, 20.0, &mut player, &mut enemies, &mut menu, &mut shop_details);

    let mut bullets: Vec<Attack> = Vec::new();
    let mut coins: Vec<Money> = vec![];
    let mut damage_text = DamageText::new();


    loop {
        if is_key_down(KeyCode::Escape) {
            waves.menu.start_btn.text = "RESUME".to_string();
            waves.menu.menu().await;
        }

        if is_key_down(KeyCode::F1) {
            waves.menu.start_btn.text = "RESUME".to_string();
            Item::shop_menu(&mut waves.shop.shop_items, &mut waves.player, &waves.shop.shop_textures, &mut waves.shop.shop).await;
        }

        waves.update().await;

        Camera::init(&waves.player);

        background::draw(&background_texture);

        waves.player.movement().await;
        waves.player.sprite.update();
        waves.player.draw();

        Money::update(&mut coins, &mut waves.player, waves.shop.shop_textures.get_texture("money_notes").unwrap().clone());

        Attack::attack(get_time(), &mut waves.player, &mut bullets, &mut waves.enemies.enemy_pool, &mut coins, &mut damage_text);
        Enemy::enemy_collection(get_time(), &mut waves.enemies, &waves.player).await;
        Attack::draw_weapon_system(&mut waves.enemies.enemy_pool, &mut waves.player, &mut bullets, &mut bullet_texture.clone(), &shoot_sound);

        Enemy::update(&mut waves.enemies.enemy_pool, &mut waves.player);

        damage_text.update(get_frame_time());
        damage_text.draw();

        set_default_camera();

        waves.draw_game_status();
        waves.player.draw_stats();
        next_frame().await;
    }
}