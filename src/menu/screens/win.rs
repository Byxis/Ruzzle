use crate::config::Config;
use crate::menu::menu::Button;
use crate::menu::utils::{draw_back_button, draw_text_center};
use raylib::prelude::Color;
use raylib::prelude::RaylibDrawHandle;
use raylib::prelude::*;

pub fn draw_win(
    d: &mut RaylibDrawHandle,
    config: &Config,
    back_button: &Button,
    texture: Option<&Texture2D>,
    buttons: &[Button],
) {
    draw_text_center(
        d,
        "Niveau terminé !",
        config.screen_width,
        (config.screen_height as f32 * 0.2) as i32,
        config.font_size_h1,
        Color::WHITE,
    );
    draw_text_center(
        d,
        "Bravo !",
        config.screen_width,
        (config.screen_height as f32 * 0.35) as i32,
        config.font_size_h1,
        Color::YELLOW,
    );

    let mouse_pos = d.get_mouse_position();
    for button in buttons {
        let is_hovered = button.rectangle.check_collision_point_rec(mouse_pos);
        let bg_color = if is_hovered {
            Color::new(255, 255, 255, 40)
        } else {
            Color::new(0, 0, 0, 120)
        };
        let text_color = if is_hovered {
            Color::YELLOW
        } else {
            Color::WHITE
        };

        d.draw_rectangle_rec(button.rectangle, bg_color);
        d.draw_rectangle_lines_ex(button.rectangle, 2.0, text_color);
        draw_text_center(
            d,
            &button.label,
            config.screen_width,
            button.rectangle.y as i32 + button.rectangle.height as i32 / 2
                - config.font_size_h1 / 2,
            config.font_size_h1,
            text_color,
        );
    }
}
