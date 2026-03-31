use crate::config::Config;
use crate::menu::menu::Button;
use crate::menu::utils::{draw_back_button, draw_text_center};
use raylib::prelude::Color;
use raylib::prelude::RaylibDrawHandle;
use raylib::prelude::*;

/// Draws the settings menu screen.
///
/// Displays game configuration options and settings. Players can adjust various
/// game parameters such as graphics, audio, or other preferences through this screen.
/// A "Retour" (back) button allows returning to the main menu without saving changes,
/// or navigation to apply and save settings.
///
/// # Arguments
/// * d : &mut RaylibDrawHandle, borrows it to draw graphical elements
/// * config : &Config, used for screen dimensions and font sizes
/// * back_button : &Button, the back button to return to the main menu
/// * tex_back : &Option<Texture2D>, optional texture for the back button; if None, uses fallback styling
pub fn draw_settings(
    d: &mut RaylibDrawHandle,
    config: &Config,
    back_button: &Button,
    texture: Option<&Texture2D>,
) {
    draw_text_center(
        d,
        "Options",
        config.screen_width,
        (config.screen_height / 7) as i32,
        config.font_size_h1,
        Color::WHITE,
    );
    draw_back_button(
        d,
        back_button.rectangle,
        texture,
        &back_button.label,
        config.font_size_h2 / 3,
    );
}
