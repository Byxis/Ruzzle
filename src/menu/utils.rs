use raylib::prelude::RaylibDrawHandle;
use raylib::prelude::*;



///given a text, draws it centered based on the coordinate
/// * x :i32 x coordinates, if you want at the middle of the screen screen resolution /2 i   (can be anything but it will not be centered if not the =current screen resolution)
/// * y : i32 y coordinate
/// fontsize : i32 necessary to compute the center of the displayed text
///  color : Color of the text
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
