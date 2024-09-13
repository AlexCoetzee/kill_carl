use macroquad::prelude::*;
use crate::player::Player;
use crate::shop::Item;

pub(crate) struct ApplyShopItem {
    pub item: Item,
}

impl ApplyShopItem {
    pub fn apply_item(&self, player: &mut Player) {
        if self.item.attributes.duration > 0 {
            self.apply_buff(player);
        } else {
            self.apply_item_attributes(player);
        }
    }

    fn apply_buff(&self, player: &mut Player) {
        /*match self.item.attributes.attribute_type.as_str() {
            "health" => player.health += self.item.attributes.quantity as f32,
            "damage" => player.damage += self.item.attributes.quantity as f32,
            "speed" => player.speed += self.item.attributes.quantity as f32,
            _ => {}
        }*/
    }

    fn apply_item_attributes(&self, player: &mut Player) {
        player.stats.money -= self.item.price;
        match self.item.attributes.attribute_type.as_str() {
            "player_health" => player.stats.health += self.item.attributes.quantity as f32,
            "defense" => player.stats.defense += self.item.attributes.quantity,
            "damage_radius" => player.weapon.damage_radius += self.item.attributes.quantity as f32,
            "damage_range" => player.weapon.range += self.item.attributes.quantity as f32,
            "weapon_damage" => player.weapon.damage += self.item.attributes.quantity as f32,
            "weapon_count" => {
                if player.weapon.count + self.item.attributes.quantity as usize > 6 {
                    player.weapon.count = 6;
                } else {
                    player.weapon.count += self.item.attributes.quantity as usize;
                }
            }
            "bullet_speed" => player.weapon.speed += self.item.attributes.quantity as f32,
            "attack_speed" => player.stats.attack_speed_modifier += self.item.attributes.quantity as f32 / 100.0,
            "movement_speed" => player.stats.movement_speed += self.item.attributes.quantity as f32,
            "critical_chance" => player.stats.critical_chance += (self.item.attributes.quantity as f32 / 100.0),
            "critical_damage" => player.stats.critical_damage += (self.item.attributes.quantity as f32 / 100.0),
            "aoe_targets" => player.weapon.aoe_count += self.item.attributes.quantity as usize,
            _ => {}
        }
    }
}