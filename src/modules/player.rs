use macroquad::prelude::*;
use crate::modules::still_image::StillImage;
use crate::modules::collision::check_collision;

pub struct Player {
    view: StillImage,
    move_speed: f32,
    movement: Vec2,
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
            movement: vec2(0.0, 0.0),
        }
    }
    //movement functions
    pub fn handle_keypresses(&mut self) {
        let mut move_dir = vec2(0.0, 0.0);

        if is_key_down(KeyCode::D) {
            move_dir.x += 1.0;
        }
        if is_key_down(KeyCode::A) {
            move_dir.x -= 1.0;
        }
        if is_key_down(KeyCode::S) {
            move_dir.y += 1.0;
        }
        if is_key_down(KeyCode::W) {
            move_dir.y -= 1.0;
        }

        if move_dir.length() > 0.0 {
            move_dir = move_dir.normalize();
        }

        let movement = move_dir * self.move_speed * get_frame_time();
        self.movement = movement;
    }

    pub fn move_and_check_x(&mut self, img2: &StillImage) -> bool {
        let mut collided = false; // Placeholder for collision check
        self.view.set_x(self.view.get_x() + self.movement.x);
        if check_collision(self.view_player(), img2, 1) {
            collided = true;
        }
        collided
    }

    pub fn move_and_check_y(&mut self, img2: &StillImage) -> bool {
        let mut collided = false; // Placeholder for collision check
        self.view.set_y(self.view.get_y() + self.movement.y);
        if check_collision(self.view_player(), img2, 1) {
            collided = true;
        }
        collided
    }
    
    //general functions
    pub fn get_oldpos(&self) -> Vec2 {
        vec2(self.view.get_x(), self.view.get_y())
    }

    pub fn set_x(&mut self, x: f32) {
        self.view.set_x(x);
    }

    pub fn set_y(&mut self, y: f32) {
        self.view.set_y(y);
    }

    pub fn draw(&self) {
        self.view.draw();
    }

    pub fn view_player(&self) -> &StillImage {
        &self.view
    }

}