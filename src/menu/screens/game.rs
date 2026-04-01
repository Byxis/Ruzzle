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
    // shader: &mut Shader,
) {
    let mut d3d = d.begin_mode3D(camera);
    // let mut s = d3d.begin_shader_mode(shader);
    d3d.draw_grid(10, 1.0);
    crab.draw(&mut d3d);
    map.draw(&mut d3d);
    level.draw(&mut d3d, assets);
}
