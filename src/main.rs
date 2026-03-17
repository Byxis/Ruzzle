use raylib::prelude::*;

mod blocks;
mod levels;

fn main() {
    let screen_width = 800;
    let screen_height = 450;

    let (mut rl, thread) = raylib::init()
        .size(screen_width, screen_height)
        .title("Ruzzle")
        .build();

    rl.set_target_fps(60);
    let mut level = levels::level1::Level1::new();
    while !rl.window_should_close() {
        level.draw(&mut rl, &thread);
        if rl.is_key_down(KeyboardKey::KEY_Q) {
            println!("Rotate Left");
        }
        if rl.is_key_down(KeyboardKey::KEY_E) {
            println!("Rotate Right")
        }
    }
}
