use crate::config::Config;
use crate::menu::menu::Button;
use crate::menu::utils::{draw_interactive_button, draw_text_center, draw_quote_bubble};
use raylib::math::Vector2;
use raylib::prelude::Color;
use raylib::prelude::RaylibDrawHandle;

/// Draws the main selection menu screen.
///
/// Displays the "RUZZLE" title and interactive buttons for all game modes:
/// Game, Level Selection, Settings, Multiplayer, and Credit.
/// Each button automatically detects hover state and applies styling accordingly.
/// The player can click any button to navigate to the corresponding menu or start the game.
///
/// # Arguments
/// * d : &mut RaylibDrawHandle, borrows it to draw graphical elements
/// * config : &Config, used for screen dimensions and font sizes
/// * buttons : &[Button], the list of interactive buttons (Game, LevelSelection, Settings, etc.)
/// * quote : &str, the quote to display (empty string if show_quote is false)
pub fn draw_select(d: &mut RaylibDrawHandle, config: &Config, buttons: &[Button], quote : &str) {
    draw_text_center(
        d,
        "RUZZLE",
        config.screen_width,
        (config.screen_height / 10) as i32,
        config.font_size_h1,
        Color::WHITE,
    );

    for button in buttons {
        draw_interactive_button(
            d,
            button.rectangle,
            None,
            &button.label,
            config.font_size_h2,
        );
    }

        // Afficher la citation seulement si elle n'est pas vide
    if !quote.is_empty() {
        let bubble_width = config.screen_width as f32 * 0.4;
        let pos_x = (config.screen_width as f32 - bubble_width) / 2.5;
        let pos_y = config.screen_height as f32 * 0.85;
        draw_quote_bubble(
            d,
            quote,
            Vector2::new(pos_x, pos_y),
            config.font_size_h2 / 2,
            bubble_width,
        );
    }
}
