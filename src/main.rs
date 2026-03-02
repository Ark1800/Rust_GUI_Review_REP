/*
By: <Your Name Here>
Date: 2026-02-12
Program Details: <Program Description Here>
*/

mod modules;
mod screen1;
mod screen2;
use macroquad::prelude::*;
use crate::modules::preload_image::TextureManager;
use crate::modules::preload_image::LoadingScreenOptions; // If you want to customize the loading screen

/// Set up window settings before the app runs
fn window_conf() -> Conf {
    Conf {
        window_title: "Rust_GUI_Review".to_string(),
        window_width: 1024,
        window_height: 768,
        fullscreen: false,
        high_dpi: true,
        window_resizable: true,
        sample_count: 4, // MSAA: makes shapes look smoother
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    let all_assets = vec!["assets/king.png","assets/maze.png","assets/end.png","assets/locked_door.png","assets/key.png",];
    let tm = TextureManager::new();
    // Using custom loading screen appearance
    let loading_options = LoadingScreenOptions {
       title: Some("Maze Game".to_string()),
       background_color: DARKBLUE,
       bar_fill_color: GOLD,
       // Use default values for other options
       ..Default::default()
    };
    tm.preload_with_loading_screen(&all_assets, Some(loading_options)).await;
    let mut current_screen = "screen1".to_string();
    let mut last_switch = get_time() - 0.02;

    loop {
        if get_time() - last_switch > 0.01 {
            current_screen = match current_screen.as_str() {
                "screen1" => screen1::run().await,
                "screen2" => screen2::run().await,
                _ => break,
            };
            last_switch = get_time();
        }
        next_frame().await;
    }
}
