use macroquad::prelude::*;
use crate::modules::still_image::StillImage;

pub struct mwall {
    view: StillImage,
    move_speed: f32,
    movement: Vec2,
}

impl mwall {
    pub async fn new(image_path: &str, width: f32, height: f32, x: f32, y: f32) -> Self {
        let view = StillImage::new(
            image_path,
            width,  // width 
            height,  // height
            x,     // x position
            y,     // y position
            true,   // Enable stretching
            1.0,    // Normal zoom (100%)
        ).await;

        mwall {
            view,
            move_speed: 50.0, // Movement speed in pixels per second
            movement: vec2(0.0, 0.0),
        }
    }

    pub fn move_updown(&mut self, first_pos: Vec2) {
        self.movement.y = self.move_speed * get_frame_time();
        self.set_y(self.view.get_y() + self.movement.y);
        if self.view.get_y() >= first_pos.y + 100.0 || self.view.get_y() <= first_pos.y - 100.0 {
            self.move_speed = -self.move_speed; // Move up
        }
    }

    pub fn set_y(&mut self, new_y: f32) {
        self.view.set_y(new_y);
    }

    pub fn get_firstpos(&self) -> Vec2 {
        vec2(self.view.get_x(), self.view.get_y())
    }

    pub fn draw(&self) {
        self.view.draw();
    }

    pub fn view_player(&self) -> &StillImage {
        &self.view
    }

}
