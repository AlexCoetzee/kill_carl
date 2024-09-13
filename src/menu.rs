use macroquad::prelude::*;
use crate::background;
use crate::utilities;

pub(crate) struct Menu {
    pub start_btn: Button,
    pub quit_btn: Button,
    pub background: Texture2D,
}

pub struct Button {
    pos: Vec2,
    width: f32,
    height: f32,
    pub text: String,
    border_radius: f32,
}

impl Menu {
    pub async fn new() -> Self {
        let start_btn = Button {
            pos: Vec2::new( screen_width() / 2.0 - 100.0, screen_height() / 2.0 - 25.0),
            width: 200.0,
            height: 50.0,
            text: "START".to_string(),
            border_radius: 20.0,
        };

        let quit_btn = Button {
            pos: Vec2::new( screen_width() / 2.0 - 100.0, (screen_height() / 2.0 - 25.0) + 60.0),
            width: 200.0,
            height: 50.0,
            text: "QUIT".to_string(),
            border_radius: 20.0,
        };

        Menu {
            start_btn,
            quit_btn,
            background: load_texture("assets/background/background.png").await.unwrap(),
        }
    }

    pub async fn menu(&mut self) {
        loop {
            background::draw(&self.background);
            // Draw the semi-transparent background
            let background_color = Color::new(0.0, 0.0, 0.0, 0.7);
            draw_rectangle(0.0, 0.0, screen_width(), screen_height(), background_color);
            if self.start() {
                break;
            }
            next_frame().await;
        }
    }

    pub(crate) fn start(&mut self) -> bool {
        self.start_btn.pos = Vec2::new( screen_width() / 2.0 - 100.0, (screen_height() / 2.0) - (self.start_btn.height / 2.0));
        self.quit_btn.pos = Vec2::new( screen_width() / 2.0 - 100.0, (screen_height() / 2.0) + 35.0);
        let is_hovering_start = utilities::hovering_over(self.start_btn.pos, Vec2::new(self.start_btn.width, self.start_btn.height));
        let start_text = utilities::center_text(&self.start_btn.text, 30, Vec2::new(self.start_btn.width, self.start_btn.height), self.start_btn.pos);
        let btn_start_color = if is_hovering_start { GRAY } else { BLUE };
        Menu::draw_rounded_rect(self.start_btn.pos.x, self.start_btn.pos.y, self.start_btn.width, self.start_btn.height, self.start_btn.border_radius, btn_start_color);
        draw_text(&self.start_btn.text, start_text.x, start_text.y, 30.0, WHITE);

        let is_hover_quit = utilities::hovering_over(self.quit_btn.pos, Vec2::new(self.quit_btn.width, self.quit_btn.height));
        let quit_text = utilities::center_text("QUIT", 30, Vec2::new(self.quit_btn.width, self.quit_btn.height), self.quit_btn.pos);
        let btn_quit_color = if is_hover_quit { GRAY } else { BLUE };
        Menu::draw_rounded_rect(self.quit_btn.pos.x, self.quit_btn.pos.y, self.quit_btn.width, self.quit_btn.height, self.quit_btn.border_radius, btn_quit_color);
        draw_text("QUIT", quit_text.x, quit_text.y, 30.0, WHITE);

        if is_hovering_start && is_mouse_button_pressed(MouseButton::Left) {
            true
        } else if is_hover_quit && is_mouse_button_pressed(MouseButton::Left) {
            std::process::exit(0);
        } else {
            false
        }
    }

    // Function to draw a rectangle with rounded corners
    fn draw_rounded_rect(x: f32, y: f32, width: f32, height: f32, radius: f32, color: Color) {
        // Draw the four rounded corners as circles
        draw_circle(x + radius, y + radius, radius, color);
        draw_circle(x + width - radius, y + radius, radius, color);
        draw_circle(x + width - radius, y + height - radius, radius, color);
        draw_circle(x + radius, y + height - radius, radius, color);

        // Draw the four edges as rectangles
        draw_rectangle(x + radius, y, width - 2.0 * radius, height, color);
        draw_rectangle(x, y + radius, radius, height - 2.0 * radius, color);
        draw_rectangle(x + width - radius, y + radius, radius, height - 2.0 * radius, color);
        draw_rectangle(x + radius, y + height - radius, width - 2.0 * radius, radius, color);
    }
}