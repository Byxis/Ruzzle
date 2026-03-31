use raylib::prelude::*;
use crate::config::{self, Config};
use crate::menu::menu::Button;
use crate::menu::utils::{draw_back_button, draw_interactive_button, draw_text_center};
use raylib::prelude::Color;
use raylib::prelude::RaylibDrawHandle;

  pub  fn draw_level_selection(
    d: &mut RaylibDrawHandle, 
    config: &Config,
    level_buttons: &[Button],
    texture : &Option<Texture2D>,
    back_button : &Button
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
            draw_interactive_button(d, button.rectangle, &None, &button.label, config.font_size_h1);
        }

        draw_back_button(
            d,
            back_button.rectangle,
            texture,
            &back_button.label,
            config.font_size_h2 / 3,
        );
    }