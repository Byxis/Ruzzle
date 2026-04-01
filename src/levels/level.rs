use crate::blocks::material::BlockMaterial;
use crate::blocks::modele::{BlockType, GroupBlock};
use crate::blocks::prefab::beach::{create_level1, create_level2};
use crate::components::collider::Collider;
use crate::menu::menu::Assets;
use raylib::math::glam::vec3;
use raylib::prelude::*;

/// Represents a 3D level with a group of blocks.
///
/// # Examples
///
/// ```
/// use raylib::prelude::*;
/// use crate::levels::level::Level;
///
/// self.current_level = Some(Level::new((i + 1) as i8));
/// ```
pub struct Level {
    pub groups: Vec<GroupBlock>,
    pub camera: Camera3D,
    pub selected_group: Option<usize>,
}

impl Level {
    /// Initialize a level
    /// To add a new level, just add the index and the group of blocks wanted
    pub fn new(index: i8) -> Self {
        let mut groups = Vec::new();

        match index {
            1 => groups.push(create_level1(Vector3::new(0.0, 0.0, 0.0))),
            2 => groups.push(create_level2(Vector3::new(0.0, 0.0, 0.0))),
            _ => {}
        }

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

    /// Updates the 3D placement of all the blocks of the level
    /// When it's rotating, dragging, or still
    pub fn update(&mut self, rl: &RaylibHandle) {
        let dt = rl.get_frame_time();

        // Update visual animations for all groups
        for group in self.groups.iter_mut() {
            group.update_animation(dt);
        }

        // Handle interaction logic if a group is selected
        if let Some(index) = self.selected_group {
            if let Some(group) = self.groups.get_mut(index) {
                if !group.is_rotating {
                    // Process movement and rotation inputs
                    Self::update_drag(&mut self.selected_group, group, dt, rl, self.camera);
                    Self::update_rotation(group, rl);
                }
            }
        }
    }

    /// Handles keyboard input to trigger 90-degree rotations on valid axes.
    pub fn update_rotation(group: &mut GroupBlock, rl: &RaylibHandle) {
        let mut rotation_to_apply: Option<Quaternion> = None;

        let is_h_type =
            group.block_type == BlockType::All || group.block_type == BlockType::RotationH;
        let is_v_type =
            group.block_type == BlockType::All || group.block_type == BlockType::RotationV;

        // Horizontal rotation (Y-axis)
        if is_h_type {
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

        // Vertical rotation (X-axis)
        if is_v_type {
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

        // Apply the new target orientation if an input was detected
        if let Some(rot) = rotation_to_apply {
            group.is_rotating = true;
            group.rotation_progress = 0.0;
            group.target_orientation = rot * group.orientation;
        }
    }

    pub fn update_drag(
        selected_group_idx: &mut Option<usize>,
        group: &mut GroupBlock,
        dt: f32,
        rl: &RaylibHandle,
        camera: Camera3D,
    ) {
        if group.block_type == BlockType::Drag {
            if rl.is_mouse_button_down(MouseButton::MOUSE_BUTTON_LEFT) {
                if !group.is_dragging {
                    group.drag_timer += dt;
                    if group.drag_timer > 0.15 {
                        group.is_dragging = true;
                    }
                }

                if group.is_dragging {
                    let mouse_ray = rl.get_screen_to_world_ray(rl.get_mouse_position(), &camera);
                    let axis = group.end_pos - group.start_pos;
                    let axis_len_sq = axis.dot(axis);

                    if axis_len_sq > 0.0 {
                        let plane_normal = (camera.target - camera.position).normalize();
                        let denom = mouse_ray.direction.dot(plane_normal);

                        if denom.abs() > 0.0001 {
                            let t_plane =
                                (group.start_pos - mouse_ray.position).dot(plane_normal) / denom;
                            let world_mouse_pos =
                                mouse_ray.position + (mouse_ray.direction * t_plane);
                            let v = world_mouse_pos - group.start_pos;
                            let t_axis = v.dot(axis) / axis_len_sq;

                            group.position = group.start_pos + (axis * t_axis.clamp(0.0, 1.0));
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
                        std::mem::swap(&mut group.start_pos, &mut group.end_pos);
                    } else {
                        group.position = group.start_pos;
                    }
                }
                group.is_dragging = false;
                group.drag_timer = 0.0;
                *selected_group_idx = None;
            }
        }
    }

    /// Checks if a given collider at a specific position is in collision with any block in the level
    /// Returns `true` if the map collides with the given collider at the given position.
    pub fn collides_with(&self, other: &Collider, pos: Vector3) -> bool {
        self.groups.iter().any(|g| {
            g.children.iter().any(|child| {
                let world_pos = g.position + child.position;
                child.collider.collides_with(world_pos, other, pos)
            })
        })
    }
    /// Resolves collisions for the given collider at the given position, returning the new position
    pub fn resolve_collisions(&self, collider: &Collider, mut pos: Vector3) -> Vector3 {
        for group in &self.groups {
            for child in &group.children {
                let world_pos = group.position + child.position;
                if let Some(push) = collider.get_penetration_vector(pos, &child.collider, world_pos)
                {
                    pos += push;
                }
            }
        }
        pos
    }

    /// Returns `true` if the given collider is grounded (touching a map collider below).
    pub fn is_grounded(&self, collider: &Collider, pos: Vector3) -> bool {
        self.collides_with(collider, pos - Vector3::new(0.0, 0.05, 0.0))
    }

    /// Draws the map using the given 3D drawing context.
    pub fn draw(&mut self, rl: &mut RaylibDrawHandle, assets: &Assets) {
        let is_clicked = rl.is_mouse_button_pressed(MouseButton::MOUSE_BUTTON_LEFT);
        let camera = self.camera;

        let mut new_selected = self.selected_group;

        //let mut d = rl.begin_drawing(thread);
        //d.clear_background(Color::RAYWHITE);

        for (i, group) in self.groups.iter_mut().enumerate() {
            if group.is_mouse_over(&rl, &camera) {
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

        self.selected_group = new_selected;

        {
            let mut d3d = rl.begin_mode3D(&camera);
            for group in self.groups.iter() {
                group.draw(&mut d3d, assets);
                if group.is_dragging {
                    group.draw_drag_guides(&mut d3d);
                }
            }
        }
    }
}
