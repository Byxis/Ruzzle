use crate::components::map::Map;
use crate::crab::crab::Crab;
use crate::levels::level::Level;
use crate::menu::menu::Assets;
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
    crab: &mut Crab,
    map: &Map,
    camera: &Camera3D,
    level: &mut Level,
    assets: &Assets,
) {
    {
        level.handle_input_from_draw(d, camera); // ← sélection avec la bonne caméra

        {
            let mut d3d = d.begin_mode3D(camera); // ← un seul begin_mode3D
            crab.draw(&mut d3d);
            level.draw(&mut d3d, assets);
            if let Some(ep) = level.endpoint_world() {
                d3d.draw_sphere(ep, 0.3, Color::GREEN);
            }
        }
    }
}
