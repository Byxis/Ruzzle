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
            }
        }
        match level_id {
            1 => {
                draw_text_center(
                    d,
                    "Tu peux bouger les cubes en cliquant longtemps et en déplaçant la souris.",
                    config.screen_width,
                    (config.screen_height / 7) * 6 as i32,
                    config.font_size_h2 -2,
                    Color::WHITE,
                );
            }
            2 => {
                draw_text_center(
                    d,
                    "Les problèmes...",
                    config.screen_width,
                    (config.screen_height / 7) * 6 as i32,
                    config.font_size_h2,
                    Color::WHITE,
                );
            }
            3 => {
                draw_text_center(
                    d,
                    "Si seulement des cubes pouvaient être bougés avec les flèches du clavier ...",
                    config.screen_width,
                    (config.screen_height / 7) * 6 as i32,
                    config.font_size_h2,
                    Color::WHITE,
                );
            }
            4 => {
                draw_text_center(
                    d,
                    "Être patient, tu dois. - Yoda ",
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
