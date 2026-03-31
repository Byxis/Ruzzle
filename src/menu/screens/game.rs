use raylib::prelude::*;
use crate::crab::crab::Crab;
use crate::components::map::Map;
use raylib::prelude::RaylibDrawHandle;


  pub  fn draw_game(
    d: &mut RaylibDrawHandle,
    crab : &mut Crab,
    map :  &Map,
    camera : &Camera3D
) {
        let mut d3d = d.begin_mode3D(camera);
            d3d.draw_grid(10, 1.0);
            crab.draw(&mut d3d);
            map.draw(&mut d3d);

    }