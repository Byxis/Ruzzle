use raylib::prelude::*;

mod crab;
mod crab_animator;
use crate::crab::Crab;

mod menu;
use crate::menu::menu::MenuManager;

mod config;
use config::Config;

mod sound_manager;
use sound_manager::sound_manager::{SoundManager,BackgroundMusic, SoundEffect};

fn main() {
    let config = Config::new();
    let (mut rl, thread) = raylib::init()
        .size(config.screen_width, config.screen_height)
        .title("Ruzzle")
        .build();

    let audio = RaylibAudio::init_audio_device().expect("Failed to initialize audio device");
    let mut sound_manager = SoundManager::new(&audio, &config); 

    sound_manager.set_background_music(BackgroundMusic::CrabRave);
    sound_manager.start_background_music();

    // let mut current_menu = Menu::Title;
    let mut menu_manager = MenuManager::new(&config, &mut rl, &thread);
    if rl.get_screen_width() != menu_manager.config.screen_width
        || rl.get_screen_height() != menu_manager.config.screen_height
    {
        unsafe {
            raylib::ffi::SetConfigFlags(raylib::ffi::ConfigFlags::FLAG_WINDOW_RESIZABLE as u32);
        }
    }

    let camera = Camera3D::perspective(
        Vector3::new(10.0, 10.0, 0.0),
        Vector3::new(0.0, 0.0, 0.5),
        Vector3::new(0.0, 1.0, 0.0),
        45.0,
    );

    let mut crab = Crab::new(
        &mut rl,
        &thread,
        "rsc/crab.glb",
        Vector3::new(0.0, 0.0, 0.0),
        0.0,
    );

    
    

    rl.set_target_fps(60);

    while !rl.window_should_close() {
        //Updating the game
        menu_manager.update(&rl, &sound_manager);
        sound_manager.update_music_stream();

        crab.update_with_camera(&mut rl, &camera, &thread);
        //Drawing the game
        let mut d = rl.begin_drawing(&thread);
        menu_manager.draw(&mut d, &crab, &camera);
    }
}
