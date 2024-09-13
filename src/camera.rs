use macroquad::camera::{set_camera, Camera2D};
use macroquad::math::{vec2, Vec2};
use macroquad::prelude::*;
use crate::player::Player;

pub(crate) struct Camera {}

impl Camera {
    pub(crate) fn init(player: &Player) {
        // Clamping the camera target position to stay within the map boundaries
        let clamped_x = player.position.x.clamp(screen_width() / 3.0, screen_width() / 1.5);
        let clamped_y = player.position.y.clamp(screen_height() / 3.0, screen_height() / 1.5);

        let camera = Camera2D {
            target: vec2(clamped_x, clamped_y),
            zoom: vec2(2.2 / screen_width(), 2.2 / screen_height()),
            offset: Vec2::new(0.0, 0.0),
            ..Default::default()
        };
        set_camera(&camera);
    }
}