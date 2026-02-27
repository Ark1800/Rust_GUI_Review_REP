use macroquad::prelude::*;
use crate::modules::still_image::StillImage;

pub struct mwall {
    view: StillImage,
    move_speed: f32,
    movement: Vec2,
}

impl mwall {
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

        mwall {
            view,
            move_speed: 200.0, // Movement speed in pixels per second
            movement: vec2(0.0, 0.0),
        }
    }
}