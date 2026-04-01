use crate::components::collider::CollisionShape;
use crate::components::map::Map;
use crate::crab::crab::Crab;
use crate::levels::level::Level;
use crate::menu::menu::{self, Assets};
use crate::menu::utils::draw_text_center;
use crate::Config;
use raylib::prelude::RaylibDrawHandle;
use raylib::prelude::*;

/// Draws the active gameplay screen.
///
/// Renders the 3D game world with the crab player, map terrain, and all interactive elements.
/// This is called every frame during active gameplay and uses the camera to render
/// the scene from the appropriate viewpoint. Map collisions and object rendering are
/// handled by the map's internal draw methods.
///
/// # Arguments
/// * d : &mut RaylibDrawHandle, borrows it to draw graphical elements
/// * crab : &mut Crab, the player entity (crabito) to be rendered
/// * map : &Map, the game world containing terrain and interactive objects
/// * camera : &Camera3D, the 3D camera defining the view of the scene
/// * config : &Config, used for screen dimensions and configuration settings
pub fn draw_game(
    d: &mut RaylibDrawHandle,
    config: &Config,
    crab: &mut Crab,
    map: &Map,
    camera: &Camera3D,
    level: &mut Level,
    assets: &Assets,
    level_id: i8,
    shader: &mut Shader,
) {
    {
        level.handle_input_from_draw(d, camera);

        {
            let mut d3d = d.begin_mode3D(camera);

            crab.draw(&mut d3d);
            level.draw(&mut d3d, assets);

            {
                d3d.begin_shader_mode(shader);
                //map.draw(&mut s);
            }

            if let Some(ep) = level.endpoint_world() {
                d3d.draw_sphere(ep, 0.3, Color::GREEN);
            }

            for group in &level.groups {
                for child in &group.children {
                    if let CollisionShape::Box { half_size } = child.collider.shape {
                        d3d.draw_cube_wires_v(child.collider.offset, half_size * 2.0, Color::RED);
                    }
                }
            }
        }
        match level_id {
            1 => {
                draw_text_center(
                    d,
                    "You can drag group of cubes to a certain position to help you",
                    config.screen_width,
                    (config.screen_height / 7) * 6 as i32,
                    config.font_size_h2,
                    Color::WHITE,
                );
            }
            2 => {
                draw_text_center(
                    d,
                    "The problems",
                    config.screen_width,
                    (config.screen_height / 7) * 6 as i32,
                    config.font_size_h2,
                    Color::WHITE,
                );
            }
            3 => {
                draw_text_center(
                    d,
                    "Cubes can be rotated with the arrows of the keyboard...",
                    config.screen_width,
                    (config.screen_height / 7) * 6 as i32,
                    config.font_size_h2,
                    Color::WHITE,
                );
            }
            4 => {
                draw_text_center(
                    d,
                    "Patient, you must be - Yoda ",
                    config.screen_width,
                    (config.screen_height / 7) * 6 as i32,
                    config.font_size_h2,
                    Color::WHITE,
                );
            }
            _ => {}
        }
    }
}
