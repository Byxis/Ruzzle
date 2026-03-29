use raylib::prelude::*;

use crate::blocks;
use crate::blocks::modele::BlockType;
use crate::blocks::modele::GroupBlock;
use crate::blocks::prefab::beach::*;

pub struct Level1 {
    pub groups: Vec<GroupBlock>,
    pub camera: Camera3D,
    pub selected_group: Option<usize>,
}

impl Level1 {
    pub fn new() -> Self {
        let mut groups = Vec::new();

        let mon_pont = create_sand_pillar(Vector3::new(2.0, 0.0, -2.0))
            .with_end_pos(Vector3::new(-2.0, 0.0, -2.0));
        groups.push(mon_pont);

        Self {
            camera: Camera3D::perspective(
                Vector3::new(0.0, 10.0, 10.0),
                Vector3::new(0.0, 0.0, 0.0),
                Vector3::new(0.0, 1.0, 0.0),
                45.0,
            ),
            groups,
            selected_group: None,
        }
    }

    // Fonction qui dessine le niveau
    pub fn draw(&mut self, rl: &mut RaylibHandle, thread: &RaylibThread) {
        let mut d = rl.begin_drawing(thread);
        d.clear_background(Color::RAYWHITE);

        let is_clicked = d.is_mouse_button_pressed(MouseButton::MOUSE_BUTTON_LEFT);
        let camera = self.camera;

        // Gestion unifiée de la sélection
        for (i, group) in self.groups.iter_mut().enumerate() {
            if group.is_mouse_over(&d, &camera) {
                group.set_temporary_color(Color::YELLOW); // Méthode à ajouter dans GroupBlock
                if is_clicked {
                    self.selected_group = Some(i);
                }
            } else {
                if Some(i) == self.selected_group {
                    group.set_temporary_color(Color::ORANGE);
                } else {
                    group.reset_color(); // Reprend les couleurs de base des enfants
                }
            }
        }

        let mut d3d = d.begin_mode3D(&self.camera);
        for group in &self.groups {
            group.draw(&mut d3d);
            if group.is_dragging {
                group.draw_drag_guides(&mut d3d);
            }
        }
        d3d.draw_grid(10, 1.0);
    }

    // Fonction appelée lors de la boucle pour gérer les mouvements des blocks durant le niveau
    pub fn update(&mut self, rl: &RaylibHandle) {
        let dt = rl.get_frame_time();
        let mouse_pos = rl.get_mouse_position();

        // Met à jour l'animation des groups
        for group in self.groups.iter_mut() {
            group.update_animation(dt);
        }

        // Gére l'entrée utilisateur pour le group sélectionné
        if let Some(index) = self.selected_group {
            let group = &mut self.groups[index];

            // On ne permet de cliquer que si le group ne tourne pas déjà
            if !group.is_rotating {
                let mut rotation_to_apply = None;

                if group.block_type == BlockType::Drag {
                    if rl.is_mouse_button_down(MouseButton::MOUSE_BUTTON_LEFT) {
                        // On active le drag si on maintient le click
                        if !group.is_dragging {
                            group.drag_timer += dt;
                            if group.drag_timer > 0.15 {
                                group.is_dragging = true;
                            }
                        }

                        // Si mode drag
                        if group.is_dragging {
                            let mouse_ray =
                                rl.get_screen_to_world_ray(rl.get_mouse_position(), &self.camera);

                            let axis = group.end_pos - group.start_pos;
                            let axis_len_sq = axis.dot(axis);

                            if axis_len_sq > 0.0 {
                                let mut plane_normal = self.camera.target - self.camera.position;
                                plane_normal.normalize();

                                let denom = mouse_ray.direction.dot(plane_normal);

                                if denom.abs() > 0.0001 {
                                    let t_plane = (group.start_pos - mouse_ray.position)
                                        .dot(plane_normal)
                                        / denom;
                                    let world_mouse_pos =
                                        mouse_ray.position + (mouse_ray.direction * t_plane);

                                    let v = world_mouse_pos - group.start_pos;
                                    let t_axis = v.dot(axis) / axis_len_sq;

                                    let t_clamped = t_axis.clamp(0.0, 1.0);
                                    let direction = group.end_pos - group.start_pos;
                                    group.position = group.start_pos + (direction * t_clamped);
                                }
                            }
                        }
                    } else {
                        // Changement des positions start et end après un snap
                        if group.is_dragging {
                            let axis = group.end_pos - group.start_pos;
                            let current_v = group.position - group.start_pos;
                            let axis_len_sq = axis.dot(axis);

                            let progress = if axis_len_sq > 0.0 {
                                current_v.dot(axis) / axis_len_sq
                            } else {
                                0.0
                            };

                            if progress > 0.5 {
                                group.position = group.end_pos;
                                let old_start = group.start_pos;
                                group.start_pos = group.end_pos;
                                group.end_pos = old_start;
                            } else {
                                group.position = group.start_pos;
                            }
                        }
                        self.selected_group = None;
                        group.is_dragging = false;
                        group.drag_timer = 0.0;
                    }
                }

                // Si c'est un ngroup qui peut tourner horizontalement
                if group.block_type == BlockType::All || group.block_type == BlockType::RotationH {
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

                // Si c'est un group qui peut tourner verticalement
                if group.block_type == BlockType::All || group.block_type == BlockType::RotationV {
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
                    group.is_rotating = true;
                    group.rotation_progress = 0.0;
                    group.target_orientation = rot * group.orientation;
                }
            }
        }
    }
}
