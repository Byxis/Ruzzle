use raylib::{ffi::IsKeyPressed, prelude::*};


mod camera_controls;

use camera_controls::handle_camera;
use camera_controls::display_coordinates;
use camera_controls::handle_coords_toggle;


fn main() {
    let screen_width = 800;
    let screen_height = 450;

    let (mut rl, thread) = raylib::init()
        .size(screen_width, screen_height)
        .title("Ruzzle")
        .build();

    let mut camera = Camera3D::perspective(
        Vector3::new(0.0, 10.0, 10.0),
        Vector3::new(0.0, 0.0, 0.0),
        Vector3::new(0.0, 1.0, 0.0),
        45.0,
    );

    let cube_position = Vector3::new(0.0, 0.0, 0.0);

    let mut show_coords = true;


    rl.set_target_fps(60);

    while !rl.window_should_close() {
        handle_coords_toggle(&rl, &mut show_coords);
        
        handle_camera(&rl, &mut camera);
        let mut d = rl.begin_drawing(&thread);

        d.clear_background(Color::RAYWHITE);
        {
            let mut d3d = d.begin_mode3D(camera);

            d3d.draw_cube(cube_position, 2.0, 2.0, 2.0, Color::RED);
            d3d.draw_cube_wires(cube_position, 2.0, 2.0, 2.0, Color::MAROON);
            d3d.draw_grid(10, 1.0);
        }


        display_coordinates(&mut d, &mut camera, show_coords);
        d.draw_fps(10, 10);
    }
}
