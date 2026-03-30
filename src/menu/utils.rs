use raylib::prelude::RaylibDrawHandle;
use raylib::prelude::*;

/// Draws text centered horizontally at a given position.
/// 
/// This function calculates the text width and positions it so that it appears
/// centered around the x coordinate provided (typically screen_width for full centering).
///
/// # Arguments
/// * d : &mut RaylibDrawHandle, borrows it to draw text
/// * text : &str, the text to display
/// * x : i32, the horizontal center reference (use screen_width / 2 for full centering)
/// * y : i32, the vertical position (top-left of text baseline)
/// * font_size : i32, necessary to compute the text width for centering
/// * color : Color, the color of the text
pub fn draw_text_center(
    d: &mut RaylibDrawHandle,
    text: &str,
    x: i32,
    y: i32,
    font_size: i32,
    color: Color,
) {
    let text_length = d.measure_text(text, font_size);
    d.draw_text(text, (x / 2) - (text_length / 2), y, font_size, color);
}



/// Draws a button texture with tinting based on hover state.
///
/// Uses draw_texture_pro to render the texture at the specified rectangle position
/// and size, with optional brightness adjustment when hovered.
///
/// # Arguments
/// * d : &mut RaylibDrawHandle, borrows it to draw the texture
/// * texture : &Texture2D, the texture to display
/// * rect : Rectangle, position and size of the button
/// * hovered : bool, if true, brightens the texture; if false, uses normal white tint
pub fn draw_texture_button(
    d: &mut RaylibDrawHandle,
    texture: &Texture2D,
    rect: Rectangle,
    hovered: bool,
) {
    // Source = toute la texture
    let src = Rectangle::new(0.0, 0.0, texture.width as f32, texture.height as f32);

    // Destination = ton rectangle de bouton
    let dst = rect;

    // Origine = coin haut-gauche (rotation 0)
    let origin = Vector2::new(0.0, 0.0);

    // Teinte: on peut éclaircir si hovered
    let tint = if hovered {
        Color::new(255, 255, 255, 220)
    } else {
        Color::WHITE
    };

    d.draw_texture_pro(texture, src, dst, origin, 0.0, tint);
}



/// Draws a rectangular button with text, with color and styling based on hover state.
///
/// The button displays different colors when hovered (typically brighter) vs normal state,
/// and the text color also changes (white when hovered, black when not).
/// Text is auto-centered within the button rectangle.
///
/// # Arguments
/// * d : &mut RaylibDrawHandle, borrows it to draw the rectangle and text
/// * button : Rectangle, the position and size of the button
/// * text : &str, the label displayed inside the button
/// * color_hovered : Color, the button background color when mouse hovers
/// * color : Color, the button background color when not hovered
/// * hovered : bool, whether the mouse is currently hovering this button
/// * font_size : i32, the font size for the button label
/// 
pub fn draw_button(
    d: &mut RaylibDrawHandle,
    button: Rectangle,
    text: &str,
    color_hovered: Color,
    color: Color,
    hovered: bool,
    font_size: i32,
) {
    let text_width_play = d.measure_text(text, font_size);
    if hovered {
        d.draw_rectangle_rec(button, color_hovered);
        d.draw_text(
            text,
            (button.x + (button.width - text_width_play as f32) / 1.8) as i32,
            (button.y + (button.height - (button.height / 2.0)) / 2.0) as i32,
            font_size,
            Color::WHITE,
        );
    } else {
        d.draw_rectangle_rec(button, color);
        d.draw_text(
            text,
            (button.x + (button.width - text_width_play as f32) / 2.0) as i32,
            (button.y + (button.height - (button.height / 2.0)) / 2.0) as i32,
            font_size,
            Color::BLACK,
        );
    }
}



/// Draws a texture while maintaining its aspect ratio within the screen bounds.
///
/// Scales the texture down proportionally to fit inside the screen without distortion,
/// and centers it. Useful for background images or logos that should never be stretched.
///
/// # Arguments
/// * d : &mut RaylibDrawHandle, borrows it to draw the texture
/// * tex : &Texture2D, the texture to display
/// * screen_w : i32, the screen width in pixels
/// * screen_h : i32, the screen height in pixels
pub fn draw_texture_contain(
    d: &mut RaylibDrawHandle,
    tex: &Texture2D,
    screen_w: i32,
    screen_h: i32,
) {
    let sw = screen_w as f32;
    let sh = screen_h as f32;
    let tw = tex.width as f32;
    let th = tex.height as f32;

    let scale = (sw / tw).min(sh / th);
    let dw = tw * scale;
    let dh = th * scale;
    let dx = (sw - dw) * 0.5;
    let dy = (sh - dh) * 0.5;

    let src = Rectangle::new(0.0, 0.0, tw, th);
    let dst = Rectangle::new(dx, dy, dw, dh);

    d.draw_texture_pro(tex, src, dst, Vector2::new(0.0, 0.0), 0.0, Color::WHITE);
}



/// Draws a "back" button that can display either a texture or fallback to a styled rectangle+text.
///
/// This is a convenience function for menus with a return/back button. It automatically
/// detects if the mouse is hovering and applies hover styling. If a texture is provided,
/// it displays the texture with hover effect; otherwise it renders a colored rectangle
/// with text label (similar to draw_button).
///
/// # Arguments
/// * d : &mut RaylibDrawHandle, borrows it to draw the button
/// * rect : Rectangle, the position and size of the back button
/// * texture : &Option<Texture2D>, optional texture; if None, falls back to rectangle+text
/// * label : &str, the text to display (used only if texture is None)
/// * font_size : i32, the font size for the label (used only if texture is None)


pub fn draw_back_button(
    d: &mut RaylibDrawHandle,
    rect: Rectangle,
    texture: &Option<Texture2D>,
    label: &str,
    font_size: i32,
) {
    let mouse = d.get_mouse_position();
    let hovered = rect.check_collision_point_rec(mouse);

    if let Some(tex) = texture {
        draw_texture_button(d, tex, rect, hovered);
    } else {
        draw_button(d,rect,label,Color::DARKORANGE,
            Color::DARKGRAY,hovered, font_size
        );
    }
}
