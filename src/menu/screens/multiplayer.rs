use raylib::prelude::*;
use crate::config::Config;
use crate::menu::menu::Button;
use crate::menu::utils::{draw_text_center, draw_back_button};
use raylib::prelude::Color;
use raylib::prelude::RaylibDrawHandle;


/// Draws the multiplayer menu screen.
///
/// Displays information or options related to multiplayer gameplay mode.
/// Includes a "Retour" (back) button to return to the main menu.
/// This screen allows players to access or configure multiplayer features.
///
/// # Arguments
/// * d : &mut RaylibDrawHandle, borrows it to draw graphical elements
/// * config : &Config, used for screen dimensions and font sizes
/// * back_button : &Button, the back button to return to the main menu
/// * tex_back : &Option<Texture2D>, optional texture for the back button; if None, uses fallback styling
pub fn draw_multiplayer(
    d: &mut RaylibDrawHandle,
    config: &Config,
    back_button : &Button,
    texture : &Option<Texture2D>
) {
 draw_text_center(
            d,
            "Multijoueur",
            config.screen_width,
            (config.screen_height / 7) as i32,
            config.font_size_h2,
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
