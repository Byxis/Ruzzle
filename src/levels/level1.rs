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
                blocks::beach::BlockPrefab::new(
                    Vector3 {
                        x: 1.0,
                        y: -4.0,
                        z: 0.0,
                    },
                    Some(Vector3 {
                        x: 5.0,
                        y: -4.0,
                        z: 0.0,
                    }),
                    Color::RED,
                    None,
                    BlockType::Drag,
                ),
                blocks::beach::BlockPrefab::new(
                    Vector3 {
                        x: 4.0,
                        y: 3.0,
                        z: 0.0,
                    },
                    None,
                    Color::BLUE,
                    Some(Vector3 {
                        x: 2.0,
                        y: 2.0,
                        z: 2.0,
                    }),
                    BlockType::All,
                ),
                blocks::beach::BlockPrefab::new(
                    Vector3 {
                        x: 1.0,
                        y: 3.0,
                        z: 0.0,
                    },
                    Some(Vector3 {
                        x: 3.0,
                        y: 3.0,
                        z: 3.0,
                    }),
                    Color::GREEN,
                    None,
                    BlockType::RotationH,
                ),
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
            cube.draw_drag_guides(&mut d3d);
        }
        d3d.draw_grid(10, 1.0);
    }

    // Fonction appelée lors de la boucle pour gérer les mouvements des blocks durant le niveau
    pub fn update(&mut self, rl: &RaylibHandle) {
        let dt = rl.get_frame_time();
        let mouse_pos = rl.get_mouse_position();

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

                if cube.block_type == BlockType::Drag {
                    if rl.is_mouse_button_down(MouseButton::MOUSE_BUTTON_LEFT) {
                        // On active le drag si on maintient le click
                        if !cube.is_dragging {
                            cube.drag_timer += dt;
                            if cube.drag_timer > 0.15 {
                                cube.is_dragging = true;
                            }
                        }

                        // Si mode drag
                        if cube.is_dragging {
                            let mouse_ray =
                                rl.get_screen_to_world_ray(rl.get_mouse_position(), &self.camera);

                            let axis = cube.end_pos - cube.start_pos;
                            let axis_len_sq = axis.dot(axis);

                            if axis_len_sq > 0.0 {
                                let mut plane_normal = self.camera.target - self.camera.position;
                                plane_normal.normalize();

                                let denom = mouse_ray.direction.dot(plane_normal);

                                if denom.abs() > 0.0001 {
                                    let t_plane = (cube.start_pos - mouse_ray.position)
                                        .dot(plane_normal)
                                        / denom;
                                    let world_mouse_pos =
                                        mouse_ray.position + (mouse_ray.direction * t_plane);

                                    let v = world_mouse_pos - cube.start_pos;
                                    let t_axis = v.dot(axis) / axis_len_sq;

                                    let t_clamped = t_axis.clamp(0.0, 1.0);
                                    let direction = cube.end_pos - cube.start_pos;
                                    cube.position = cube.start_pos + (direction * t_clamped);
                                }
                            }
                        }
                    } else {
                        // Changement des positions start et end après un snap
                        if cube.is_dragging {
                            let axis = cube.end_pos - cube.start_pos;
                            let current_v = cube.position - cube.start_pos;
                            let axis_len_sq = axis.dot(axis);

                            let progress = if axis_len_sq > 0.0 {
                                current_v.dot(axis) / axis_len_sq
                            } else {
                                0.0
                            };

                            if progress > 0.5 {
                                cube.position = cube.end_pos;
                                let old_start = cube.start_pos;
                                cube.start_pos = cube.end_pos;
                                cube.end_pos = old_start;
                            } else {
                                cube.position = cube.start_pos;
                            }
                        }
                        self.selected_cube = None;
                        cube.is_dragging = false;
                        cube.drag_timer = 0.0;
                    }
                }

                // Si c'est un ncube qui peut tourner horizontalement
                if cube.block_type == BlockType::All || cube.block_type == BlockType::RotationH {
                    if rl.is_key_pressed(KeyboardKey::KEY_RIGHT) {
                        // Rotation de 90° autour de l'axe Y
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
                }

                // Si c'est un cube qui peut tourner verticalement
                if cube.block_type == BlockType::All || cube.block_type == BlockType::RotationV {
                    if rl.is_key_pressed(KeyboardKey::KEY_UP) {
                        // Rotation de 90° autour de l'axe X
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
                }

                if let Some(rot) = rotation_to_apply {
                    cube.is_rotating = true;
                    cube.rotation_progress = 0.0;
                    cube.target_orientation = rot * cube.current_orientation;
                }
            }
        }
    }
}
