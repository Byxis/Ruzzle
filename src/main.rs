use raylib::math::glam::Vec3;
use raylib::prelude::*;

mod components;
mod crab;
mod crab_animator;
use crate::components::collider::Collider;
use crate::components::map::Map;
use crate::crab::Crab;

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

    let mut crab = Crab::new(
        &mut rl,
        &thread,
        "rsc/crab.glb",
        Vector3::new(0.0, 0.0, 0.0),
        0.0,
    );

    let mut map = Map::new(&mut rl, &thread, "rsc/map.glb");
    map.set_position(Vector3::new(0.0, -0.2, 0.0));

    map.add_collider(Collider::with_box_from_size(16.0, 0.2, 16.0));

    map.add_collider(Collider::with_box_from_size_offset(
        1.0,
        5.0,
        1.0,
        Vec3::new(4.5, 3.5, 4.5),
    ));

    rl.set_target_fps(60);

    while !rl.window_should_close() {
        let mut t = crab.calculate_next_transform(&mut rl, &camera, &thread);

        t.position = map.resolve_collisions(&crab.collider, t.position);
        crab.teleport(t);

        let mut d = rl.begin_drawing(&thread);
        d.clear_background(Color::BLACK);

        {
            let mut d3d = d.begin_mode3D(camera);

            d3d.draw_grid(10, 1.0);
            crab.draw(&mut d3d);
            crab.collider.draw(&mut d3d, crab.transform);

            if !d3d.is_key_down(KeyboardKey::KEY_Q) {
                map.draw(&mut d3d);
            }
            map.draw_collider(&mut d3d);
        }

        let coordonnees = format!(
            "({:.2}, {:.2}, {:.2})",
            crab.transform.position.x, crab.transform.position.y, crab.transform.position.z
        );
        d.draw_text(&coordonnees, 10, 40, 20, Color::DARKGRAY);
        d.draw_fps(10, 10);
    }
}
