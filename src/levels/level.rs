use crate::Assets;
use raylib::prelude::*;

use crate::blocks::modele::BlockType;
use crate::blocks::modele::GroupBlock;

pub trait Level {
    fn groups_mut(&mut self) -> &mut Vec<GroupBlock>;
    fn camera(&self) -> &Camera3D;
    fn selected_group(&self) -> Option<usize>;
    fn selected_group_mut(&mut self) -> &mut Option<usize>;

    fn update(&mut self, rl: &RaylibHandle) {
        let dt = rl.get_frame_time();

        // variable locale pour éviter le &&mut
        let groups = self.groups_mut();
        for group in groups.iter_mut() {
            group.update_animation(dt);
        }

        if let Some(index) = self.selected_group() {
            // on récupère ce dont on a besoin AVANT d'emprunter groups_mut
            let camera = *self.camera();

            let group = &mut self.groups_mut()[index];

            if !group.is_rotating {
                let mut rotation_to_apply = None;

                if group.block_type == BlockType::Drag {
                    if rl.is_mouse_button_down(MouseButton::MOUSE_BUTTON_LEFT) {
                        if !group.is_dragging {
                            group.drag_timer += dt;
                            if group.drag_timer > 0.15 {
                                group.is_dragging = true;
                            }
                        }

                        if group.is_dragging {
                            let mouse_ray =
                                rl.get_screen_to_world_ray(rl.get_mouse_position(), &camera);

                            let axis = group.end_pos - group.start_pos;
                            let axis_len_sq = axis.dot(axis);

                            if axis_len_sq > 0.0 {
                                let mut plane_normal = camera.target - camera.position;
                                plane_normal = plane_normal.normalize();

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
                                    group.position = group.start_pos
                                        + ((group.end_pos - group.start_pos) * t_clamped);
                                }
                            }
                        }
                    } else {
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
                        group.is_dragging = false;
                        group.drag_timer = 0.0;

                        *self.selected_group_mut() = None;
                    }
                }

                // re-borrow après le bloc drag
                let group = &mut self.groups_mut()[index];

                if group.block_type == BlockType::All || group.block_type == BlockType::RotationH {
                    if rl.is_key_pressed(KeyboardKey::KEY_RIGHT) {
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

                if group.block_type == BlockType::All || group.block_type == BlockType::RotationV {
                    if rl.is_key_pressed(KeyboardKey::KEY_UP) {
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

    fn draw(&mut self, rl: &mut RaylibHandle, thread: &RaylibThread, assets: &Assets) {
        let mut d = rl.begin_drawing(thread);
        d.clear_background(Color::RAYWHITE);

        let is_clicked = d.is_mouse_button_pressed(MouseButton::MOUSE_BUTTON_LEFT);
        let camera = *self.camera();

        let mut new_selected = self.selected_group();
        for (i, group) in self.groups_mut().iter_mut().enumerate() {
            if group.is_mouse_over(&d, &camera) {
                group.set_temporary_color(Color::YELLOW);
                if is_clicked {
                    new_selected = Some(i);
                }
            } else if Some(i) == new_selected {
                group.set_temporary_color(Color::ORANGE);
            } else {
                group.reset_color();
            }
        }

        *self.selected_group_mut() = new_selected;
        let mut d3d = d.begin_mode3D(&camera);
        for group in self.groups_mut().iter() {
            group.draw(&mut d3d, assets);
            if group.is_dragging {
                group.draw_drag_guides(&mut d3d);
            }
        }
        d3d.draw_grid(10, 1.0);
    }
}
