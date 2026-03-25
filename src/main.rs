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

mod config;
use config::Config;

fn main() {
    let config = Config::new();
    let (mut rl, thread) = raylib::init()
        .size(config.screen_width, config.screen_height)
        .title("Ruzzle")
        .build();

    let mut shader = rl.load_shader(
        &thread,
        Some("rsc/shaders/pulse.vs"),
        Some("rsc/shaders/pulse.fs"),
    );
    eprintln!("Shader ID: {}", shader.id);
    let u_time_loc = shader.get_shader_location("uTime");

    // let mut current_menu = Menu::Title;
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

    for material in map.model.materials_mut() {
        material.shader = *shader.as_ref();
    }
    for material in crab.crab_animator.model.materials_mut() {
        material.shader = *shader.as_ref();
    }

    while !rl.window_should_close() {
        //Updating the game
        menu_manager.update(&mut rl, &thread, &map, &mut crab, &camera);
        shader.set_shader_value(u_time_loc, rl.get_time() as f32);

        //Drawing the game
        let mut d = rl.begin_drawing(&thread);
        d.clear_background(Color::BLACK);

        menu_manager.draw(&mut d, &map, &mut crab, &camera, &mut shader);
    }
}
