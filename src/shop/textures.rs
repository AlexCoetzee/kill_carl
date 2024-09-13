use crate::shop::ShopTextures;

pub(crate) async fn get_textures() -> ShopTextures {
    let mut texture_map = ShopTextures::new();
    texture_map.add_texture("love_potion", "assets/shop/love_potion.png").await;
    texture_map.add_texture("buy_button", "assets/shop/buy_button.png").await;
    texture_map.add_texture("money_notes", "assets/money/money_notes.png").await;
    texture_map
}