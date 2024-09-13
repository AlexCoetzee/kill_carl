use macroquad::math::vec2;
use macroquad::prelude::*;

#[allow(dead_code)]
pub(crate) fn debug(mut debug_x: f32, mut debug_y: f32) -> (f32, f32) {
    let button_position_increase_y = vec2(100.0, 200.0);
    let button_position_increase_x = vec2(100.0, 100.0);
    let button_position_decrease_y = vec2(250.0, 200.0);
    let button_position_decrease_x = vec2(250.0, 100.0);


    let button_size = vec2(150.0, 50.0);
    let button_color = RED;

    // Draw the button
    draw_rectangle(button_position_increase_x.x, button_position_increase_x.y, button_size.x, button_size.y, button_color);
    draw_rectangle(button_position_increase_y.x, button_position_increase_y.y, button_size.x, button_size.y, button_color);

    draw_rectangle(button_position_decrease_x.x, button_position_decrease_x.y, button_size.x, button_size.y, button_color);
    draw_rectangle(button_position_decrease_y.x, button_position_decrease_y.y, button_size.x, button_size.y, button_color);

    // Draw the button label
    draw_text("Increase X!", button_position_increase_x.x + 20.0, button_position_increase_x.y + 35.0, 30.0, WHITE);
    draw_text("Increase Y!", button_position_increase_y.x + 20.0, button_position_increase_y.y + 35.0, 30.0, WHITE);

    draw_text("Decrease X!", button_position_decrease_x.x + 20.0, button_position_decrease_x.y + 35.0, 30.0, WHITE);
    draw_text("Decrease Y!", button_position_decrease_y.x + 20.0, button_position_decrease_y.y + 35.0, 30.0, WHITE);

    // Check for mouse clicks
    if is_mouse_button_pressed(MouseButton::Left) {
        let mouse_position: Vec2 = mouse_position().into();

        // Check if the click is inside the button bounds
        if mouse_position.x >= button_position_increase_x.x && mouse_position.x <= button_position_increase_x.x + button_size.x &&
            mouse_position.y >= button_position_increase_x.y && mouse_position.y <= button_position_increase_x.y + button_size.y {
            // Increment the counter
            debug_x += 1.0;
        }

        // Check if the click is inside the button bounds
        if mouse_position.x >= button_position_increase_y.x && mouse_position.x <= button_position_increase_y.x + button_size.x &&
            mouse_position.y >= button_position_increase_y.y && mouse_position.y <= button_position_increase_y.y + button_size.y {
            // Increment the counter
            debug_y += 1.0;
        }

        // Check if the click is inside the button bounds
        if mouse_position.x >= button_position_decrease_x.x && mouse_position.x <= button_position_decrease_x.x + button_size.x &&
            mouse_position.y >= button_position_decrease_x.y && mouse_position.y <= button_position_decrease_x.y + button_size.y {
            // Increment the counter
            debug_x -= 1.0;
        }

        // Check if the click is inside the button bounds
        if mouse_position.x >= button_position_decrease_y.x && mouse_position.x <= button_position_decrease_y.x + button_size.x &&
            mouse_position.y >= button_position_decrease_y.y && mouse_position.y <= button_position_decrease_y.y + button_size.y {
            // Increment the counter
            debug_y -= 1.0;
        }

        println!("X: {}, Y: {}", debug_x, debug_y);
    }
    (debug_x, debug_y)
}