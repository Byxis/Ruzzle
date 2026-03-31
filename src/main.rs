use raylib::math::glam::Vec3;
use raylib::prelude::*;

mod components;
use crate::components::collider::Collider;
use crate::components::map::Map;
use crate::components::transform::Transform3D;

mod crab;
use crate::crab::crab::Crab;

mod menu;
use crate::menu::menu::MenuManager;

mod sound_manager;
use crate::sound_manager::sound_manager::{BackgroundMusic, SoundManager};

mod blocks;
mod levels;

const SCREEN_WIDTH: i32 = 1280;
const SCREEN_HEIGHT: i32 = 720;

mod config;
use config::Config;

mod shader;

fn main() {
    let config = Config::new(SCREEN_WIDTH, SCREEN_HEIGHT);
    let (mut rl, thread) = raylib::init()
        .size(config.screen_width, config.screen_height)
        .title("Ruzzle")
        .build();

    let mut menu_manager = MenuManager::new(config, &mut rl, &thread);
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

    let audio = RaylibAudio::init_audio_device().expect("Failed to initialize audio device");
    let mut sound_manager = SoundManager::new(&audio);
    sound_manager.set_background_music(&audio, BackgroundMusic::CrabRave);
    sound_manager.start_background_music();

    let spawn_point = Transform3D::new(Vector3::new(0.0, 5.0, 0.0), 0.0);
    let mut map = Map::new(&mut rl, &thread, "rsc/map.glb");

    map.set_position(Vector3::new(0.0, -0.2, 0.0));
    map.set_spawn_point(spawn_point);

    map.add_collider(Collider::with_box_from_size(16.0, 0.2, 16.0));
    map.add_collider(Collider::with_box_from_size_offset(
        1.0,
        5.0,
        1.0,
        Vec3::new(4.5, 3.5, 4.5),
    ));
    map.add_collider(Collider::with_box_from_size_offset(
        7.0,
        2.0,
        7.0,
        Vec3::new(-4.5, 0.0, -4.5),
    ));

    let mut crab = Crab::new(&mut rl, &thread, "rsc/crab.glb");
    crab.teleport(map.spawn_point);

    rl.set_target_fps(60);

    menu_manager
        .shader_manager
        .apply_cel_shade_to_model(&mut map.model);
    menu_manager
        .shader_manager
        .apply_cel_shade_to_model(&mut crab.crab_animator.model);

    while !rl.window_should_close() {
        sound_manager.update_music_stream();
        menu_manager.update(&mut rl, &thread, &map, &mut crab, &camera);

        {
            let mut d = rl.begin_drawing(&thread);
            d.clear_background(Color::BLACK);

            menu_manager.draw(&mut d, &thread, &map, &mut crab, &camera);
        }
    }
}
