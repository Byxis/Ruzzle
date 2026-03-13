use raylib::prelude::*;

use crate::blocks;

pub struct Level2 {
    pub cubes: Vec<blocks::beach::BlockPrefab>,
    pub camera: Camera3D,
}

impl Level2 {
    pub fn new() -> Self {
        Self {
            camera: Camera3D::perspective(
                Vector3::new(0.0, 10.0, 10.0),
                Vector3::new(0.0, 0.0, 0.0),
                Vector3::new(0.0, 1.0, 0.0),
                45.0,
            ),
            cubes: vec![
                blocks::beach::BlockPrefab::new(5.0, 4.0, 0.0, Color::RED),
                blocks::beach::BlockPrefab::new(3.0, 3.0, 0.0, Color::BLUE),
                blocks::beach::BlockPrefab::new(4.0, 1.0, 0.0, Color::GREEN),
            ],
        }
    }

    // Fonction qui dessine le niveau
    pub fn draw(&mut self, rl: &mut RaylibHandle, thread: &RaylibThread) {
        let mut d = rl.begin_drawing(thread);
        d.clear_background(Color::RAYWHITE);

        let is_clicked = d.is_mouse_button_pressed(MouseButton::MOUSE_BUTTON_LEFT);
        let camera = self.camera;

        for cube in self.cubes.iter_mut() {
            if cube.is_mouse_over(&d, &camera) {
                cube.color = Color::YELLOW;
                if is_clicked {
                    println!("Bloc cliqué !");
                }
            } else {
                cube.color = cube.base_color;
            }
        }

        let mut d3d = d.begin_mode3D(&self.camera);
        for cube in &self.cubes {
            cube.draw(&mut d3d);
        }
        d3d.draw_grid(10, 1.0);
    }
}
