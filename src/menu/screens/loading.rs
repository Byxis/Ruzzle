use raylib::prelude::*;
use crate::config::Config;
use raylib::prelude::Color;
use raylib::prelude::RaylibDrawHandle;




/// Draws the loading screen.
///
/// Displays a loading animation or progress indication while the game is initializing
/// resources. This screen is shown for a fixed duration (artificial delay) or until
/// all assets are fully loaded. A logo or loading message is typically displayed
/// to keep the player informed and entertained during the wait.
///
/// # Arguments
/// * d : &mut RaylibDrawHandle, borrows it to draw graphical elements
/// * config : &Config, used for screen dimensions and font sizes
/// * bg_logo : &Option<Texture2D>, optional texture for the loading screen background or logo
pub fn draw_loading(
    d: &mut RaylibDrawHandle,
    config: &Config,
    texture_logo : &Option<Texture2D>
) {
    if let Some(texture) = texture_logo {
            let x = config.screen_width / 2 - texture.width / 2;
            let y = config.screen_height / 2 - texture.height / 2;
            d.draw_texture(texture, x, y, Color::WHITE);
        }
    }
