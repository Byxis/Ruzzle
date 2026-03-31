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

mod shader;
use crate::shader::daylight::DayCycleManager;
use crate::shader::shader::ShaderManager;

fn main() {
    let config = Config::new();
    let (mut rl, thread) = raylib::init()
        .size(config.screen_width, config.screen_height)
        .title("Ruzzle")
        .build();

    let mut shader_manager = ShaderManager::new(&mut rl, &thread);
    let day_cycle = DayCycleManager::new();
    // day_cycle.set_test_hour(Some(2.0));

    let mut render_target = rl
        .load_render_texture(
            &thread,
            config.screen_width as u32,
            config.screen_height as u32,
        )
        .unwrap();

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

    // Apply cel_shade shader to all model materials
    shader_manager.apply_cel_shade_to_model(&mut map.model);
    shader_manager.apply_cel_shade_to_model(&mut crab.crab_animator.model);

    while !rl.window_should_close() {
        shader_manager.set_sunlight_color(day_cycle.get_light_color());
        shader_manager.set_ambient_color(day_cycle.get_ambient_color());
        shader_manager.update_background_colors(
            day_cycle.get_background_top(),
            day_cycle.get_background_bottom(),
        );

        // Updating the game
        menu_manager.update(&mut rl, &thread, &map, &mut crab, &camera);

        // Update post-process resolution uniform
        shader_manager.update_postprocess_resolution(
            menu_manager.config.screen_width as f32,
            menu_manager.config.screen_height as f32,
        );

        {
            let mut d = rl.begin_drawing(&thread);
            d.clear_background(Color::BLACK);

            if menu_manager.is_in_game() {
                // Draw 3D scene into RenderTexture
                {
                    let mut td = d.begin_texture_mode(&thread, &mut render_target);
                    td.clear_background(Color::new(0, 0, 0, 0));
                    menu_manager.draw_game_scene(
                        &mut td,
                        &mut crab,
                        &map,
                        &camera,
                        &mut shader_manager.cel_shade_shader,
                    );
                }

                // Draw the RenderTexture to screen with post-process shader
                {
                    let mut sd = d.begin_shader_mode(&mut shader_manager.postprocess_shader);

                    sd.draw_texture_rec(
                        render_target.texture(),
                        Rectangle::new(
                            0.0,
                            0.0,
                            render_target.texture().width as f32,
                            -(render_target.texture().height as f32), // Y-flip required because OpenGL textures are upside-down
                        ),
                        Vector2::new(0.0, 0.0),
                        Color::WHITE,
                    );
                }

                // Draw HUD on top
                menu_manager.draw_game_hud(&mut d, &crab);
            } else {
                menu_manager.draw(
                    &mut d,
                    &map,
                    &mut crab,
                    &camera,
                    &mut shader_manager.cel_shade_shader,
                );
            }
        }
    }
}
