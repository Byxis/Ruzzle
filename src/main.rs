use raylib::prelude::*;

mod crab;
mod crab_animator;
use crate::crab::Crab;

mod sound_manager;
use crate::sound_manager::sound_manager::{SoundManager, BackgroundMusic, SoundEffect};

const SCREEN_WIDTH: i32 = 1280;
const SCREEN_HEIGHT: i32 = 720;

fn main() {
    let (mut rl, thread) = raylib::init()
        .size(SCREEN_WIDTH, SCREEN_HEIGHT)
        .title("Ruzzle")
        .build();

    let camera = Camera3D::perspective(
        Vector3::new(10.0, 10.0, 0.0),
        Vector3::new(0.0, 0.0, 0.5),
        Vector3::new(0.0, 1.0, 0.0),
        45.0,
    );

    let audio = RaylibAudio::init_audio_device().expect("Failed to initialize audio device");
    let mut sound_manager = SoundManager::new(&audio);

    let mut crab = Crab::new(
        &mut rl,
        &thread,
        "rsc/crab.glb",
        Vector3::new(0.0, 0.0, 0.0),
        0.0,
    );

    rl.set_target_fps(60);

    // Apply default sound parameters and start game music
    sound_manager.set_background_music(&audio, BackgroundMusic::CrabRave);
    sound_manager.start_background_music();

    // Frame loop
    while !rl.window_should_close() {
        // Update background music stream (for continuous playing)
        sound_manager.update_music_stream(); 

        // Crab position update
        crab.update_with_camera(&mut rl, &camera, &thread);

        let mut d = rl.begin_drawing(&thread);
        d.clear_background(Color::BLACK);

        {
            let mut d3d = d.begin_mode3D(camera);

            d3d.draw_grid(10, 1.0);
            crab.draw(&mut d3d);
        }

        let coordonnees = format!(
            "({:.2}, {:.2}, {:.2})",
            crab.position.x, crab.position.y, crab.position.z
        );
        d.draw_text(&coordonnees, 10, 40, 20, Color::DARKGRAY);
        d.draw_fps(10, 10);
        
    }
}
