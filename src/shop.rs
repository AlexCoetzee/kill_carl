use macroquad::prelude::*;
use std::collections::HashMap;

mod items;
mod menu;
pub(crate) mod textures;
mod logic;

use crate::player::Player;

pub(crate) struct Shop {
    pub reroll_count: u32,
    pub reroll_cost: u32,
}

impl Shop {
    pub fn new() -> Self {
        Shop {
            reroll_cost: 2, // Starting value
            reroll_count: 0,
        }
    }

    pub fn increment_reroll_cost(&mut self) -> u32 {
        self.reroll_count += 1;

        // Apply the increment based on the press count
        if self.reroll_count % 2 == 1 {
            self.reroll_cost += 3; // Odd-index increment
        } else {
            self.reroll_cost += 2; // Even-index increment
        }
        self.reroll_cost
    }
}

#[derive(Clone, Debug)]
pub(crate) struct Item {
    pub texture_name: String,
    pub name: String,
    pub price: u32,
    pub description: String,
    pub level_requirement: u32,
    pub attributes: Attributes
}

#[derive(Clone, Debug)]
pub struct Attributes {
    pub(crate) attribute_type: String,
    pub(crate) quantity: u32,
    pub(crate) duration: u32,
}

pub struct ShopTextures {
    textures: HashMap<String, Texture2D>,
}

impl Item {
    pub fn shop(level: u32) -> Vec<Item> {
        items::get_four_items_from_list(level)
    }

    pub async fn shop_menu(items: &mut Vec<Item>, player: &mut Player, shop_textures: &ShopTextures, shop: &mut Shop) {
        menu::draw_shop(items, player, shop_textures, shop).await;
    }
}

impl ShopTextures {
    pub fn new() -> Self {
        ShopTextures {
            textures: HashMap::new(),
        }
    }

    // Method to add a texture with a name
    pub async fn add_texture(&mut self, name: &str, path: &str) {
        let texture = load_texture(path).await.unwrap();
        self.textures.insert(name.to_string(), texture);
    }

    // Method to get a texture by name
    pub fn get_texture(&self, name: &str) -> Option<&Texture2D> {
        self.textures.get(name)
    }
}