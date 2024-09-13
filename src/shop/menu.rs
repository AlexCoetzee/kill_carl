use macroquad::prelude::*;
use crate::player::Player;
use crate::shop::*;
use crate::shop::logic::ApplyShopItem;
use crate::utilities;

pub(crate) async fn draw_shop(shop_items: &mut Vec<Item>, player: &mut Player, shop_textures: &ShopTextures, shop: &mut Shop) {
    loop {
        clear_background(DARKGRAY);
        player_money(&player, &shop_textures.get_texture("money_notes").unwrap());

        if reroll_button(Vec2::new(10.0, 10.0), &shop_textures.get_texture("money_notes").unwrap(), &player, &shop) {
            *shop_items = items::get_four_items_from_list(player.stats.level.level);
            player.stats.money -= shop.reroll_cost;
            shop.increment_reroll_cost();
        }

        listings(shop_items, &shop_textures, player);
        player_attributes_panel(&player);

        if start_next_wave() {
            break;
        }

        next_frame().await;
    }
}

fn listings(displayed_items: &mut Vec<Item>, shop_textures: &ShopTextures, player: &mut Player) {
    let mut remove_index = None;
    let card_gap = 15.0;
    let card_width = screen_width() / 5.0;
    for (i, item) in displayed_items.iter().enumerate() {
        let card_x = 40.0 + i as f32 *  (card_width + card_gap) - card_gap;
        let card_y = screen_height() / 2.0 - 80.0;
        listing(&item, Vec2::new(card_x, card_y), &shop_textures.get_texture(&item.texture_name).unwrap(), card_width, &player);
        if buy_button(Vec2::new(card_x + 300.0, card_y + 130.0), &shop_textures.get_texture("buy_button").unwrap(), &player, item.price) {
            purchase_item(&item, player);
            remove_index = Some(i);
        }
    }

    if let Some(index) = remove_index {
        displayed_items.remove(index);
    }
}

pub fn listing(item: &Item, position: Vec2, texture: &Texture2D, card_width: f32, player: &Player) {
    let card_height = 200.0;
    let padding = 10.0;

    let texture_size = Vec2::new(75.0, 75.0);
    let text_x = position.x + texture_size.x + padding + 30.0;

    let is_hovered = utilities::hovering_over(position, Vec2::new(card_width, card_height));
    let can_purchase = can_purchase_item(item.price, &player);
    let mut background_color = if is_hovered {  LIGHTGRAY } else { WHITE };
    background_color = if is_hovered && !can_purchase { Color::new(1.0, 0.678, 0.643, 1.00) } else { background_color };

    // Draw card background with a border
    draw_rectangle(position.x - 5.0, position.y - 5.0, card_width + 10.0, card_height + 10.0, BLACK);
    draw_rectangle(position.x, position.y, card_width, card_height, background_color);

    draw_texture_ex(
        &texture,
        position.x + padding,
        position.y + (card_height - texture_size.y) / 2.0,
        WHITE,
        DrawTextureParams {
            dest_size: Some(texture_size),
            ..Default::default()
        },
    );

    // Draw item details
    draw_text(&item.name, text_x, position.y + 40.0, 24.0, BLACK);
    draw_text(&item.description, text_x, position.y + 70.0, 18.0, GRAY);
    draw_text(&format!("Price: {} coins", item.price), text_x, position.y + 100.0, 18.0, DARKGRAY);
    draw_text(&format!("Level: {}", item.level_requirement), text_x, position.y + 120.0, 18.0, DARKGRAY);
}

pub fn player_attributes_panel(player: &Player) {
    let x  = screen_width() - 260.0;
    let y = 10.0;
    let panel_width = 250.0;
    let panel_height = 500.0;
    let padding = 20.0;
    let mut y_offset = y + padding;

    // Draw the panel background with a border
    draw_rectangle(x - 5.0, y - 5.0, panel_width + 10.0, panel_height + 10.0, BLACK);
    draw_rectangle(x, y, panel_width, panel_height, WHITE);

    // Draw the attributes
    draw_text("Player Attributes", x + padding, y_offset, 30.0, BLACK);

    y_offset += 40.0;
    draw_text(&format!("Health: {}", player.stats.health), x + padding, y_offset, 20.0, BLACK);
    y_offset += 30.0;
    draw_text(&format!("Level: {}", player.stats.level.level), x + padding, y_offset, 20.0, BLACK);
    y_offset += 30.0;
    draw_text(&format!("Movement Speed: {:.1}", player.stats.movement_speed), x + padding, y_offset, 20.0, BLACK);
    y_offset += 30.0;
    draw_text(&format!("Defense: {}", player.stats.defense), x + padding, y_offset, 20.0, BLACK);
    y_offset += 30.0;
    draw_text(&format!("Critical Chance: {:.1}%", player.stats.critical_chance * 100.0), x + padding, y_offset, 20.0, BLACK);
    y_offset += 30.0;
    draw_text(&format!("Critical Damage: {:.1}%", player.stats.critical_damage * 100.0), x + padding, y_offset, 20.0, BLACK);
    y_offset += 30.0;
    draw_text(&format!("Attack Speed: {:.2}",  player.attack_interval()), x + padding, y_offset, 20.0, BLACK);
    y_offset += 30.0;
    draw_text(&format!("Weapon Damage: {}", player.weapon.damage), x + padding, y_offset, 20.0, BLACK);
    y_offset += 30.0;
    draw_text(&format!("Weapon Count: {}", player.weapon.count), x + padding, y_offset, 20.0, BLACK);
    y_offset += 30.0;
    draw_text(&format!("Weapon Splash Radius: {}", player.weapon.damage_radius), x + padding, y_offset, 20.0, BLACK);
    y_offset += 30.0;
    draw_text(&format!("Weapon range: {}", player.weapon.range), x + padding, y_offset, 20.0, BLACK);
    y_offset += 30.0;
    draw_text(&format!("AoE Targets: {}", player.weapon.aoe_count), x + padding, y_offset, 20.0, BLACK);
}

fn reroll_button(position: Vec2, texture: &Texture2D, player: &Player, shop: &Shop) -> bool {
    let mut text: String = String::from("REROLL -");
    text.push_str(&shop.reroll_cost.to_string());
    let can_purchase = can_purchase_item(shop.reroll_cost, &player);
    let button_width = 200.0;
    let button_height = 40.0;
    let is_hovered = utilities::hovering_over(position, Vec2::new(button_width, button_height));
    let text_measurements = measure_text(&text, None, 40, 1.0);
    let mut button_background = if is_hovered { GRAY } else { WHITE };
    if is_hovered && !can_purchase {
        button_background = Color::new(1.0, 0.678, 0.643, 1.00);
    }
    draw_rectangle(position.x, position.y, button_width + 40.0, button_height, button_background);
    let text_pos = utilities::center_text(&text, 40, Vec2::new(button_width, button_height), position);
    draw_text(
        &text,
        text_pos.x,
        text_pos.y,
        40.0,
        BLACK,
    );
    draw_texture_ex(
        &texture,
        text_pos.x + text_measurements.width + 10.0,
        text_pos.y - 27.5,
        WHITE,
        DrawTextureParams {
            dest_size: Some(Vec2::new(35.0, 35.0)),
            ..Default::default()
        },
    );
    is_hovered && is_mouse_button_pressed(MouseButton::Left) && can_purchase
}

fn buy_button(position: Vec2, texture: &Texture2D, player: &Player, cost: u32) -> bool {
    let mut texture_width = 65.0;
    let mut texture_height = 65.0;
    let can_purchase = can_purchase_item(cost, &player);
    let is_hovered = utilities::hovering_over(position, Vec2::new(texture_width, texture_height));
    if is_hovered {
        texture_width = 80.0;
        texture_height = 80.0;
    }
    draw_texture_ex(
        &texture,
        position.x,
        position.y,
        WHITE,
        DrawTextureParams {
            dest_size: Some(Vec2::new(texture_width, texture_height)),
            ..Default::default()
        },
    );
    is_hovered && is_mouse_button_pressed(MouseButton::Left) && can_purchase
}

pub fn player_money(player: &Player, texture: &Texture2D) {
    let text = player.stats.money.to_string();
    let text_measurements = measure_text(&text, None, 40, 1.0);
    let box_width = text_measurements.width + 100.0;
    let box_height = text_measurements.height + 20.0;
    let x = screen_width() / 2.0 - (box_width / 2.0);
    let y = 10.0;

    draw_rectangle(x, y, box_width, box_height, WHITE);
    let text_pos = Vec2::new(
        x + (box_width / 2.0) - (text_measurements.width / 2.0) - 20.0,
        y + (box_height / 2.0)  + (text_measurements.height / 2.0)
    );
    draw_text(
        &text,
        text_pos.x,
        text_pos.y,
        40.0,
        BLACK,
    );
    draw_texture_ex(
        &texture,
        text_pos.x + 10.0 + text_measurements.width,
        text_pos.y - 27.5,
        WHITE,
        DrawTextureParams {
            dest_size: Some(Vec2::new(35.0, 35.0)),
            ..Default::default()
        },
    );
}

fn can_purchase_item(cost: u32, player: &Player) -> bool {
    player.stats.money >= cost
}

fn purchase_item(item: &Item, player: &mut Player) {
    let item: Item = item.clone();
    ApplyShopItem::apply_item(&ApplyShopItem { item }, player);
}

fn start_next_wave() -> bool {
    let mut button_dim = Vec2::new(300.0, 60.0);
    let text = "START NEXT WAVE";
    let position = Vec2::new(screen_width() - 310.0, screen_height() - 70.0);
    let is_hovered = utilities::hovering_over(position, Vec2::new(button_dim.x, button_dim.y));
    let button_color = if is_hovered { GRAY } else { LIGHTGRAY };
    draw_rectangle(position.x, position.y, button_dim.x, button_dim.y, button_color);
    let text_pos = utilities::center_text(&text, 40, Vec2::new(button_dim.x, button_dim.y), position);
    draw_text(
        &text,
        text_pos.x,
        text_pos.y,
        40.0,
        BLACK,
    );
    is_hovered && is_mouse_button_pressed(MouseButton::Left)
}