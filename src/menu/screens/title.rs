use crate::config::Config;
use crate::menu::utils::draw_text_center;
use raylib::prelude::Color;
use raylib::prelude::RaylibDrawHandle;



/// Draws the title menu screen.
///
/// Displays the "Ruzzle" game title and an instruction prompt centered on screen.
/// This is the first screen shown when launching the game, waiting for the player
/// to press Enter to proceed to the main menu.
///
/// # Arguments
/// * d : &mut RaylibDrawHandle, borrows it to draw graphical elements
/// * config : &Config, used for screen dimensions and font sizes
pub fn draw_title(d: &mut RaylibDrawHandle, config: &Config) {
    draw_text_center(
        d,
        "Ruzzle",
        config.screen_width,
        (config.screen_height as i32) / 2 - (config.screen_height / 10) as i32,
        config.font_size_h1,
        Color::WHITE,
    );
    draw_text_center(
        d,
        "Appuyez sur Entrée",
        config.screen_width,
        (config.screen_height as i32) / 2,
        config.font_size_h1,
        Color::WHITE,
    );
}
