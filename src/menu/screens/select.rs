
use crate::config::Config;
use crate::menu::menu::Button;
use crate::menu::utils::{draw_interactive_button, draw_text_center};
use raylib::prelude::Color;
use raylib::prelude::RaylibDrawHandle;

pub fn draw_select(
    d: &mut RaylibDrawHandle,
    config: &Config,
    buttons: &[Button],
) {
    let color_hovered = Color::DARKORANGE;
    let color_button = Color::DARKGRAY;

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
            &None,
            &button.label,
            config.font_size_h2,
        );
    }
}
