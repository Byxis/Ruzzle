use raylib::math::glam::Vec3;
use raylib::prelude::*;
use std::sync::mpsc;
use std::thread;

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

mod multiplayer;
use multiplayer::{client, Position};

fn main() {
    let args: Vec<String> = std::env::args().collect();

    let config = Config::new();
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

    let mut crab2 = Crab::new(&mut rl, &thread, "rsc/crab_2.glb");
    crab2.teleport(map.spawn_point);

    // Create channels for network communication
    let (tx_to_main, rx_from_net) = mpsc::channel::<Position>();
    let (tx_to_net, rx_from_main) = mpsc::channel::<Position>();

    let exec_type = &args[1];
    let mut host: bool;
    match exec_type.as_str() {
        "create" => {
            host = true;
        }
        "join" => {
            host = false;
        }
        _ => {
            host = true;
            println!("Invalid argument, first one must be \"client\" or \"server\".");
        }
    }

    // Spawn client in separate thread
    thread::spawn(move || {
        client(tx_to_main, rx_from_main, host);
    });

    rl.set_target_fps(60);

    while !rl.window_should_close() {
        // Receive remote player position updates from network thread
        if let Ok(position) = rx_from_net.try_recv() {
            crab2.teleport(Transform3D::new(
                Vector3::new(position.x, position.y, position.z),
                position.rotation,
            ));
        }

        // Get local crab position and send to network thread
        let local_position = Position::new(
            crab.transform.position.x,
            crab.transform.position.y,
            crab.transform.position.z,
            crab.transform.rotation,
        );
        let _ = tx_to_net.send(local_position);

        //Updating the game
        menu_manager.update(&mut rl, &thread, &map, &mut crab, &mut crab2, &camera);

        //Drawing the game
        let mut d = rl.begin_drawing(&thread);
        d.clear_background(Color::BLACK);

        menu_manager.draw(&mut d, &map, &mut crab, &mut crab2, &camera);
    }
}
