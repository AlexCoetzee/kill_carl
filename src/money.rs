use macroquad::prelude::*;
use crate::player::Player;

pub(crate) struct Money {
    pub position: Vec2,
    pub size: Vec2,
    pub value: u32,
    pub collected: bool,
}

impl Money {
    pub async fn new(position: Vec2, value: u32) -> Self {
        Money {
            position,
            size: vec2(20.0, 20.0),
            value,
            collected: false,
        }
    }

    pub fn draw(&self, money_texture: Texture2D) {
        draw_texture_ex(
            &money_texture,
            self.position.x - self.size.x * 0.5,
            self.position.y - self.size.y * 0.5,
            WHITE,
            DrawTextureParams {
                dest_size: Some(self.size),
                ..Default::default()
            },
        );
    }

    pub fn update(coins: &mut Vec<Money>, player: &mut Player, money_texture: Texture2D) {
        // Update and draw coins
        for coin in &mut coins.iter_mut() {
            player.collect_coin(coin, 400.0); // Attraction radius of 100

            if !coin.collected {
                coin.draw(money_texture.clone());
            }
        }

        // Remove collected coins
        coins.retain(|coin| !coin.collected);
    }
}