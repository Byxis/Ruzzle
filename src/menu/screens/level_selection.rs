use crate::config::Config;
use crate::menu::menu::{Assets, Button};
use crate::menu::utils::{draw_back_button, draw_interactive_button, draw_text_center};
use raylib::prelude::Color;
use raylib::prelude::RaylibDrawHandle;

/// Draws the level selection screen.
///
/// Displays a list of playable levels as interactive buttons in a vertical layout.
/// Each level button can be clicked to start that level. A "Retour" (back) button
/// allows the player to return to the main menu. The currently hovered level is
/// highlighted with hover styling.
///
/// # Arguments
/// * d : &mut RaylibDrawHandle, borrows it to draw graphical elements
/// * config : &Config, used for screen dimensions and font sizes
/// * level_buttons : &[Button], the list of level selection buttons
/// * back_button : &Button, the back button to return to the main menu
/// * tex_back : &Option<Texture2D>, optional texture for the back button; if None, uses fallback styling
pub fn draw_level_selection(
    d: &mut RaylibDrawHandle,
    config: &Config,
    level_buttons: &[Button],
    assets: &Assets,
    back_button: &Button,
) {
    draw_text_center(
        d,
        "Niveaux",
        config.screen_width,
        (config.screen_height / 7) as i32,
        config.font_size_h2,
        Color::WHITE,
    );

    // Draw the buttons for the levels
    for button in level_buttons {
        draw_interactive_button(
            d,
            button.rectangle,
            None,
            &button.label,
            config.font_size_h2,
        );
    }

    draw_back_button(
        d,
        back_button.rectangle,
        assets.textures.get(3),
        &back_button.label,
        config.font_size_h2 / 3,
    );
}
