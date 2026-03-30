use crate::config::Config;
use crate::menu::utils::draw_text_center;
use raylib::prelude::Color;
use raylib::prelude::RaylibDrawHandle;

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
