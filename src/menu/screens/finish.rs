use crate::config::Config;
use crate::menu::menu::Button;
use crate::menu::utils::{draw_back_button, draw_text_center};
use raylib::ffi::{CSSPalette, RaylibPalette};
use raylib::prelude::Color;
use raylib::prelude::RaylibDrawHandle;
use raylib::prelude::*;

pub fn draw_finish(d: &mut RaylibDrawHandle, config: &Config) {
    draw_text_center(
        d,
        "Félicitations !",
        config.screen_width,
        (config.screen_height as f32 * 0.25) as i32,
        config.font_size_h1,
        Color::BLACK,
    );
    draw_text_center(
        d,
        "Vous avez terminé tous les niveaux !",
        config.screen_width,
        (config.screen_height as f32 * 0.45) as i32,
        config.font_size_h1,
        Color::BLACK,
    );
    draw_text_center(
        d,
        "Appuyez sur ENTRÉE pour retourner au menu",
        config.screen_width,
        (config.screen_height as f32 * 0.65) as i32,
        config.font_size_h2 / 2,
        Color::BLACK,
    );
}
