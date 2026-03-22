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

    // méthode pour appeler un niveau et le mettre en place
    let mut level = levels::level1::Level1::new();
    while !rl.window_should_close() {
        level.update(&rl);
        level.draw(&mut rl, &thread);
    }
}
