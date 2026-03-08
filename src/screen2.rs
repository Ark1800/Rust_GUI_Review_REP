use macroquad::prelude::*;
use crate::modules::mwall::Mwall;
use crate::modules::still_image::StillImage;
use crate::modules::label::Label;
use crate::modules::text_button::TextButton;
use crate::modules::messagebox::{MessageBox, MessageBoxResult};
use crate::modules::scale::use_virtual_resolution;
use crate::modules::player::Player;

pub async fn run() -> String {
    // Define virtual resolution constants
    const VIRTUAL_WIDTH: f32 = 1024.0;
    const VIRTUAL_HEIGHT: f32 = 768.0;
    //images
    let mut player = Player::new("assets/king.png",  30.0, 30.0).await;
    let mut mwall = Mwall::new("assets/end.png", 10.0, 130.0, 900.0, 290.0).await;
    let maze = StillImage::new(
    "assets/maze.png",
    VIRTUAL_WIDTH,  // width
    VIRTUAL_HEIGHT,  // height
    0.0,  // x position
    0.0,   // y position
    true,   // Enable stretching
    1.0,    // Normal zoom (100%)
    ).await;
    let end = StillImage::new(
    "assets/end.png",
    130.0,  // width
    100.0,  // height
    0.0,  // x position
    765.0,   // y position
    true,   // Enable stretching
    1.0,    // Normal zoom (100%)
    ).await;
    let wall = StillImage::new(
    "assets/end.png",
    130.0,  // width
    100.0,  // height
    50.0,  // x position
    -100.0,   // y position
    true,   // Enable stretching
    1.0,    // Normal zoom (100%)
    ).await;
    //locked_door
    let locked_door = StillImage::new(
    "assets/locked_door.png",
    75.0,  // width
    75.0,  // height
    550.0,  // x position
    690.0,   // y position
    true,   // Enable stretching
    1.0,    // Normal zoom (100%)
    ).await;
    //key
    let key = StillImage::new(
    "assets/key.png",
    80.0,  // width
    80.0,  // height
    710.0,  // x position
    30.0,   // y position
    true,   // Enable stretching
    1.0,    // Normal zoom (100%)
    ).await;
    let btn_return = TextButton::new(
    800.0,
    700.0,
    200.0,
    60.0,
    "Return to Menu",
    BLUE,
    GREEN,
    30
    );
    // Speed of movement in pixels per second
    let starttime = get_time();
    //labels
    let lbl_time_str = Label::new("Time:", 900.0, 40.0, 30);
    let mut lbl_time_num = Label::new("0", 965.0, 40.0, 30);
    //message box
    let mut end_box = MessageBox::info("Level Clear!", "You Beat The Maze!");
    let wallfirstpos = mwall.get_firstpos();
    let mut key_collected = false;
    loop {
        // Apply virtual resolution every frame
        use_virtual_resolution(VIRTUAL_WIDTH, VIRTUAL_HEIGHT);
        clear_background(DARKGRAY);
        // Only process input if message box is NOT visible
        if !end_box.is_visible() {
            //PLAYER MOVEMENT AND COLLISION CHECKING
            Mwall::move_updown(&mut mwall, wallfirstpos);
            let oldpos = player.get_oldpos(); // Store old position before movement
            player.handle_keypresses();
            //x move and collision
            player.move_x();
            if player.check_x_collision(&mwall.view_mwall()) || player.check_x_collision(&maze) || player.check_x_collision(&wall) {
                player.set_x(oldpos.x); // Revert to old x position on collision
            }
            if player.check_x_collision(&locked_door) && !key_collected {
                player.set_x(oldpos.x); // Revert to old x position on collision
            }
            //y move and collision
            player.move_y();
            if player.check_y_collision(&maze) || player.check_y_collision(&wall) {
                player.set_y(oldpos.y); // Revert to old y position on collision
            }
            if player.check_y_collision(&locked_door) && !key_collected {
                player.set_y(oldpos.y); // Revert to old y position on collision
            }
            if player.check_y_collision(&mwall.view_mwall()) {
                player.set_y(oldpos.y); // Revert to old y position on collision
                player.set_x(oldpos.x-15.0); // Revert to old x position on collision
            }
            //any collision
            if player.check_x_collision(&end) || player.check_y_collision(&end) {
                end_box.show();
            }
            if player.check_x_collision(&key) || player.check_y_collision(&key) {
                key_collected = true;
            }
        }
        if btn_return.click() {
            return "screen1".to_string();
        }
        let currenttime = format!("{:.1}", get_time()-starttime);
        lbl_time_num.set_text(currenttime);
        maze.draw();
        player.draw();
        mwall.draw();
        end.draw();
        wall.draw();
        if !key_collected {
            key.draw();
            locked_door.draw();
        }
        lbl_time_num.draw();
        lbl_time_str.draw();
        end_box.centered();  // Center the message box on screen
        // message box handling
        if let Some(result) = end_box.draw() {
            match result {
                MessageBoxResult::ButtonPressed(_) | MessageBoxResult::Closed => {
                    return "screen1".to_string();
                }
            }
        }
    next_frame().await;
    }
}