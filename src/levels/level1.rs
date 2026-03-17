use raylib::prelude::*;

use crate::blocks;
use crate::blocks::beach::BlockType;

pub struct Level1 {
    pub cubes: Vec<blocks::beach::BlockPrefab>,
    pub camera: Camera3D,
    pub selected_cube: Option<usize>,
}

impl Level1 {
    pub fn new() -> Self {
        Self {
            camera: Camera3D::perspective(
                Vector3::new(0.0, 10.0, 10.0),
                Vector3::new(0.0, 0.0, 0.0),
                Vector3::new(0.0, 1.0, 0.0),
                45.0,
            ),
            cubes: vec![
                blocks::beach::BlockPrefab::new(1.0, -4.0, 0.0, Color::RED, BlockType::All),
                blocks::beach::BlockPrefab::new(5.0, 2.0, 0.0, Color::BLUE, BlockType::Fixe),
                blocks::beach::BlockPrefab::new(1.0, 3.0, 0.0, Color::GREEN, BlockType::RotationH),
            ],
            selected_cube: None,
        }
    }

    // Fonction qui dessine le niveau
    pub fn draw(&mut self, rl: &mut RaylibHandle, thread: &RaylibThread) {
        let mut d = rl.begin_drawing(thread);
        d.clear_background(Color::RAYWHITE);

        let is_clicked = d.is_mouse_button_pressed(MouseButton::MOUSE_BUTTON_LEFT);
        let camera = self.camera;

        for (i, cube) in self.cubes.iter_mut().enumerate() {
            if cube.is_mouse_over(&d, &camera) {
                cube.color = Color::YELLOW;
                if is_clicked {
                    self.selected_cube = Some(i);
                    println!("Bloc cliqué !");
                }
            } else {
                if Some(i) == self.selected_cube {
                    cube.color = Color::ORANGE;
                } else {
                    cube.color = cube.base_color;
                }
            }
        }

        let mut d3d = d.begin_mode3D(&self.camera);
        for cube in &self.cubes {
            cube.draw(&mut d3d);
        }
        d3d.draw_grid(10, 1.0);
    }

    // Fonction appelée lors de la boucle pour gérer les mouvements des blocks durant le niveau
    pub fn update(&mut self, rl: &RaylibHandle) {
        let dt = rl.get_frame_time();

        // Met à jour l'animation des cubes
        for cube in self.cubes.iter_mut() {
            cube.update_animation(dt);
        }

        // Gére l'entrée utilisateur pour le cube sélectionné
        if let Some(index) = self.selected_cube {
            let cube = &mut self.cubes[index];

            // On ne permet de cliquer que si le cube ne tourne pas déjà
            if !cube.is_rotating {
                let mut rotation_to_apply = None;

                if rl.is_key_pressed(KeyboardKey::KEY_RIGHT) {
                    // Rotation de 90° autour de l'axe Y du MONDE
                    rotation_to_apply = Some(Quaternion::from_axis_angle(
                        Vector3::new(0.0, 1.0, 0.0),
                        90.0f32.to_radians(),
                    ));
                }
                if rl.is_key_pressed(KeyboardKey::KEY_LEFT) {
                    rotation_to_apply = Some(Quaternion::from_axis_angle(
                        Vector3::new(0.0, 1.0, 0.0),
                        -90.0f32.to_radians(),
                    ));
                }
                if rl.is_key_pressed(KeyboardKey::KEY_UP) {
                    // Rotation de 90° autour de l'axe X du MONDE
                    rotation_to_apply = Some(Quaternion::from_axis_angle(
                        Vector3::new(1.0, 0.0, 0.0),
                        -90.0f32.to_radians(),
                    ));
                }
                if rl.is_key_pressed(KeyboardKey::KEY_DOWN) {
                    rotation_to_apply = Some(Quaternion::from_axis_angle(
                        Vector3::new(1.0, 0.0, 0.0),
                        90.0f32.to_radians(),
                    ));
                }

                if let Some(rot) = rotation_to_apply {
                    cube.is_rotating = true;
                    cube.rotation_progress = 0.0;
                    // IMPORTANT : rot * current_orientation = Rotation MONDE
                    // current_orientation * rot = Rotation LOCALE
                    cube.target_orientation = rot * cube.current_orientation;
                }
            }
        }
    }
}
