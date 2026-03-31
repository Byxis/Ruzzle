use raylib::prelude::*;
use crate::config::Config;
use raylib::prelude::Color;
use raylib::prelude::RaylibDrawHandle;

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
