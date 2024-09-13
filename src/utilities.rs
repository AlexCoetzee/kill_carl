use rand::thread_rng;
use rand::Rng;
use core::ops::*;
use macroquad::input::mouse_position;
use macroquad::math::Rect;
use macroquad::prelude::{measure_text, Vec2};
use rand::rngs::ThreadRng;
use rand::seq::SliceRandom;

pub fn random_position(range: (Range<f32>, Range<f32>)) -> Vec2 {
    let mut rng = thread_rng();
    let (x_range, y_range) = range;
    Vec2::new(rng.gen_range(x_range), rng.gen_range(y_range))
}

pub fn random_spawn_position(map_width: f32, map_height: f32) -> Vec2 {
    let mut rng = rand::thread_rng();
    let edge = rng.gen_range(0..4);
    match edge {
        0 => Vec2::new(rng.gen_range(0.0..map_width), 0.0),
        1 => Vec2::new(map_width, rng.gen_range(0.0..map_height)),
        2 => Vec2::new(rng.gen_range(0.0..map_width), map_height),
        _ => Vec2::new(0.0, rng.gen_range(0.0..map_height)),
    }
}

pub fn random_number(range: RangeInclusive<i32>) -> i32 {
    let mut rng = thread_rng();
    rng.gen_range(range)
}

pub fn is_critical_hit(crit_chance: f32) -> bool {
    let mut rng = thread_rng();
    rng.gen::<f32>() < crit_chance
}

pub fn rand_thread_rng() -> ThreadRng {
    thread_rng()
}

pub fn get_random_elements<T>(vec: &Vec<T>, count: usize) -> Vec<T>
where
    T: Clone,
{
    let mut rng = thread_rng();
    let mut elements = vec.clone();
    elements.shuffle(&mut rng);
    elements.into_iter().take(count).collect()
}

pub fn center_text(text: &str, font_size: u16, container_measurements: Vec2, container_position: Vec2) -> Vec2 {
    let text_measurements = measure_text(&text, None, font_size, 1.0);
    let text_x = container_position.x + (container_measurements.x / 2.0) - (text_measurements.width / 2.0);
    let text_y = container_position.y + (container_measurements.y / 2.0)  + (text_measurements.height / 2.0);
    Vec2::new(text_x, text_y)
}

pub fn hovering_over(position: Vec2, dimensions: Vec2) -> bool {
    let button_rect = Rect::new(position.x, position.y, dimensions.x, dimensions.y);
    button_rect.contains(mouse_position().into())
}