use raylib::prelude::*;
use crate::config::Config;
use crate::menu::menu::Button;
use crate::menu::utils::{draw_text_center, draw_back_button};
use raylib::prelude::Color;
use raylib::prelude::RaylibDrawHandle;

pub fn draw_credit(
    d: &mut RaylibDrawHandle,
    config: &Config,
    back_button : &Button,
    texture : &Option<Texture2D>
) 
{
       draw_text_center(
            d,
            "Jeu réalisé par :",
            config.screen_width,
            (config.screen_height as i32) / 2 - (config.screen_height / 12) as i32,
            config.font_size_h1,
            Color::BLACK,
        );
        draw_text_center(
            d,
            "Andréa Antoniali, Max Chateau, Caroline Floquet",
            config.screen_width,
            (config.screen_height as i32) / 2,
            config.font_size_h2,
            Color::BLACK,
        );
        draw_text_center(
            d,
            "Alessandra Van Rossen Martinez, Alexis Serrano",
            config.screen_width,
            (config.screen_height as i32) / 2 + (config.screen_height / 15) as i32,
            config.font_size_h2, 
            Color::BLACK,
        );
            draw_back_button(
            d,
            back_button.rectangle,
            texture,
            &back_button.label,
            config.font_size_h2 / 3,
        );
    }
