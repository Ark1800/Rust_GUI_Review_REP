use macroquad::prelude::*;
use crate::modules::still_image::StillImage;

pub struct Player {
    view: StillImage,
    move_speed: f32,
}

impl Player {
    pub async fn new(image_path: &str, x: f32, y: f32) -> Self {
        let view = StillImage::new(
            image_path,
            30.0,  // width 
            30.0,  // height
            x,     // x position
            y,     // y position
            true,   // Enable stretching
            1.0,    // Normal zoom (100%)
        ).await;

        Player {
            view,
            move_speed: 200.0, // Movement speed in pixels per second
        }
    }
    // Direction to move in
    pub fn move_player_x(&mut self) -> f32 {
        let mut move_dir_x = 0.0;
        // Keyboard input
        if is_key_down(KeyCode::D) {
            move_dir_x += 1.0;
        }
        if is_key_down(KeyCode::A) {
            move_dir_x -= 1.0;
        }
        // Apply movement based on frame time
        let movement = move_dir_x * self.move_speed * get_frame_time();
        self.view.set_x(self.view.get_x() + movement);
        movement
        }

    pub fn move_player_y(&mut self) -> f32 {
           let mut move_dir_y = 0.0;
        // Keyboard input
        if is_key_down(KeyCode::S) {
            move_dir_y += 1.0;
        }
        if is_key_down(KeyCode::W) {
            move_dir_y -= 1.0;
        }
        // Apply movement based on frame time
        let movement = move_dir_y * self.move_speed * get_frame_time();
        self.view.set_y(self.view.get_y() + movement);
        movement
    }

    pub fn draw(&self) {
        self.view.draw();
    }

    pub fn view_player(&self) -> &StillImage {
        &self.view
    }

    pub fn set_oldpos(&mut self, x: f32, y: f32, i: i32) {
        if i == 0 {
            self.view.set_x(x);
        } else if i == 1 {
            self.view.set_y(y);
        }
    }
}