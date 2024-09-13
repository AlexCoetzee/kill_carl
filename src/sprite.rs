use macroquad::prelude::*;

#[derive(Clone)]
pub(crate) struct Sprite {
    pub texture: Texture2D,
    pub size: Vec2,
    current_frame: u32,
    frame_timer: f32,
    pub total_number_of_frames: f32,
    pub number_of_columns: f32,
    frame_width: f32,
    frame_height: f32
}

impl Sprite {
    pub fn new(texture: Texture2D, size: Vec2, total_number_of_frames: f32, number_of_columns: f32, number_of_rows: f32) -> Self {

        // The dimensions of each frame (assuming a grid of frames in the sprite sheet)
        let frame_width = texture.width() / number_of_columns;
        let frame_height = texture.height() / number_of_rows;

        // Timer for frame animation
        let frame_timer = 0.0;
        let current_frame = 0;

        Sprite {
            texture,
            size,
            current_frame,
            frame_timer,
            total_number_of_frames,
            number_of_columns,
            frame_width,
            frame_height
        }
    }

    pub fn update(&mut self) {
        // Update the frame based on a timer
        // get_frame_time() gives the time since last frame
        self.frame_timer += get_frame_time();

        if self.frame_timer > 0.1 { // Change frame every 0.1 seconds (adjust as needed)
            self.frame_timer = 0.0;
            // Loop back to the first frame
            self.current_frame = (self.current_frame + 1) % self.total_number_of_frames as u32;
        }
    }

    pub fn draw(&self, position: Vec2) {
        let x = (self.current_frame % self.number_of_columns as u32) as f32 * self.frame_width;
        let y = (self.current_frame / self.number_of_columns as u32) as f32 * self.frame_height;
        let source = Rect::new(x, y, self.frame_width, self.frame_height);

        draw_texture_ex(
            &self.texture,
            position.x - (self.size.x * 0.5),
            position.y - (self.size.y * 0.5),
            WHITE,
            DrawTextureParams {
                source: Some(source),
                dest_size: Some(self.size),
                ..Default::default()
            },
        );

    }
}