use crate::blocks;
use crate::blocks::material::{self, BlockMaterial};
use crate::components::collider::Collider;
use crate::components::collider::CollisionShape;
use crate::menu::menu::Assets;
use raylib::ffi;
use raylib::prelude::*;

#[derive(PartialEq, Clone, Copy)]
pub enum BlockType {
    Fixe,
    RotationV,
    RotationH,
    Drag,
    All,
}

/// Represents a single block unit with its properties and physics collider.
pub struct BlockPrefab {
    pub position: crate::Vector3,
    pub size: crate::Vector3,
    pub block_type: BlockType,
    pub material: material::BlockMaterial,
    pub temp_color: Option<Color>,
    pub collider: Collider,
}

impl BlockPrefab {
    /// Creates a new `BlockPrefab` with the specified position, type, and material.
    /// Defaults to a 1x1x1 size if none is provided.
    pub fn new(
        pos: Vector3,
        size: Option<Vector3>,
        block_type: BlockType,
        material: BlockMaterial,
    ) -> Self {
        let actual_size = size.unwrap_or(Vector3::new(1.0, 1.0, 1.0));
        Self {
            position: pos,
            size: actual_size,
            block_type,
            material: material,
            temp_color: None,
            collider: Collider::with_box_from_size(actual_size.x, actual_size.y, actual_size.z),
        }
    }
}

/// A logical group of one or more blocks that can rotate or move as a single entity.
pub struct GroupBlock {
    pub position: Vector3,
    pub orientation: Quaternion,
    pub children: Vec<blocks::modele::BlockPrefab>,
    pub block_type: BlockType,

    pub target_orientation: Quaternion,
    pub is_rotating: bool,
    pub rotation_progress: f32,

    pub start_pos: Vector3,
    pub end_pos: Vector3,
    pub is_dragging: bool,
    pub drag_timer: f32,

    pub endpoint_local: Option<Vector3>,
    pub model: Option<Model>,
    pub model_offset: Vector3,
    pub model_orientation: Quaternion,
}

impl GroupBlock {
    /// Creates a new `GroupBlock` from a list of existing block prefabs.
    pub fn new(
        pos: Vector3,
        children: Vec<blocks::modele::BlockPrefab>,
        block_type: BlockType,
    ) -> Self {
        Self {
            position: pos,
            orientation: Quaternion::identity(),
            target_orientation: Quaternion::identity(),
            children,
            block_type,

            is_rotating: false,
            rotation_progress: 1.0,

            start_pos: pos,
            end_pos: pos,
            is_dragging: false,
            drag_timer: 0.0,

            endpoint_local: None,
            model: None,
            model_offset: Vector3::ZERO,
            model_orientation: Quaternion::identity(),
        }
    }

    /// Helper to create a `GroupBlock` containing only one block at the given position.
    pub fn single(pos: Vector3, block_type: BlockType, material: BlockMaterial) -> Self {
        let child = BlockPrefab::new(Vector3::ZERO, None, block_type.clone(), material);
        let mut group = Self::new(pos, vec![child], block_type);
        group.sync_colliders();
        group
    }

    /// Applies a temporary highlight color to all blocks in the group.
    pub fn set_temporary_color(&mut self, color: Color) {
        for child in self.children.iter_mut() {
            child.temp_color = Some(color);
        }
    }

    /// Removes the temporary highlight and restores the blocks' original material colors.
    pub fn reset_color(&mut self) {
        for child in self.children.iter_mut() {
            child.temp_color = None;
        }
    }

    /// Renders all blocks in the group, applying current rotation and translation matrices.
    pub fn draw(&self, d: &mut impl RaylibDraw3D, assets: &Assets) {
        let animated_orientation = self
            .orientation
            .slerp(self.target_orientation, self.rotation_progress);
        let mat = animated_orientation.to_matrix();

        let matrix_array: [f32; 16] = [
            mat.m0, mat.m4, mat.m8, mat.m12, mat.m1, mat.m5, mat.m9, mat.m13, mat.m2, mat.m6,
            mat.m10, mat.m14, mat.m3, mat.m7, mat.m11, mat.m15,
        ];

        unsafe {
            raylib::ffi::rlPushMatrix();
            raylib::ffi::rlTranslatef(self.position.x, self.position.y, self.position.z);

            raylib::ffi::rlMultMatrixf(matrix_array.as_ptr());

            for child in &self.children {
                let color_to_draw = child.temp_color.unwrap_or(child.material.color);

                let tex = child
                    .material
                    .texture_id
                    .and_then(|id| assets.textures.get(id as usize))
                    .unwrap_or(&assets.textures[0]);

                draw_cube_with_texture(
                    tex,
                    child.position,
                    child.size.x,
                    child.size.y,
                    child.size.z,
                    color_to_draw,
                );

                /*
                d.draw_cube_wires(
                    child.position,
                    child.size.x,
                    child.size.y,
                    child.size.z,
                    Color::BLACK,
                );
                */
            }
            raylib::ffi::rlPopMatrix();
        }
        if let Some(model) = &self.model {
            let x = self.model_offset.x;
            let y = self.model_offset.y;
            let z = self.model_offset.z;

            let rx = mat.m0 * x + mat.m1 * y + mat.m2 * z;
            let ry = mat.m4 * x + mat.m5 * y + mat.m6 * z;
            let rz = mat.m8 * x + mat.m9 * y + mat.m10 * z;

            let world_pos = Vector3::new(
                self.position.x + rx,
                self.position.y + ry,
                self.position.z + rz,
            );

            let animated_model_orientation = self.model_orientation
                * self
                    .orientation
                    .slerp(self.target_orientation, self.rotation_progress);

            let (axis, angle) = animated_model_orientation.to_axis_angle();

            d.draw_model_ex(
                model,
                world_pos,
                axis,
                angle.to_degrees(),
                Vector3::ONE,
                Color::WHITE,
            );
        }
    }

    /// Calculates the world-space position of the group's endpoint
    /// Returns `None` if no local endpoint is defined
    pub fn endpoint_world(&self) -> Option<Vector3> {
        self.endpoint_local.map(|local| {
            let mat = self.orientation.to_matrix();
            let x = local.x;
            let y = local.y;
            let z = local.z;

            let rx = mat.m0 * x + mat.m1 * y + mat.m2 * z;
            let ry = mat.m4 * x + mat.m5 * y + mat.m6 * z;
            let rz = mat.m8 * x + mat.m9 * y + mat.m10 * z;

            Vector3::new(
                self.position.x + rx,
                self.position.y + ry,
                self.position.z + rz,
            )
        })
    }

    /// Checks if the mouse cursor is currently hovering over any block in this group.
    pub fn is_mouse_over(&self, rl: &RaylibHandle, camera: &Camera3D) -> bool {
        let ray = rl.get_screen_to_world_ray(rl.get_mouse_position(), camera);
        let mat = self
            .orientation
            .slerp(self.target_orientation, self.rotation_progress)
            .to_matrix();

        for child in &self.children {
            // Transformation manuelle du point local en point monde
            let x = child.position.x;
            let y = child.position.y;
            let z = child.position.z;

            let rx = mat.m0 * x + mat.m4 * y + mat.m8 * z + mat.m12;
            let ry = mat.m1 * x + mat.m5 * y + mat.m9 * z + mat.m13;
            let rz = mat.m2 * x + mat.m6 * y + mat.m10 * z + mat.m14;

            let world_child_pos = self.position + Vector3::new(rx, ry, rz);

            let half_size = child.size * 0.5;
            let bbox = BoundingBox::new(world_child_pos - half_size, world_child_pos + half_size);

            if bbox.get_ray_collision_box(ray).hit {
                return true;
            }
        }
        false
    }

    /// Updates the group's rotation animation downed on the delta time.
    pub fn update_animation(&mut self, dt: f32) {
        if self.is_rotating {
            self.rotation_progress += 3.0 * dt;
            if self.rotation_progress >= 1.0 {
                self.rotation_progress = 1.0;
                self.orientation = self.target_orientation;
                self.is_rotating = false;
                self.bake_rotation();
            }
            self.sync_colliders();
        }
    }

    ///Permanently applies the current orientation to the children's local offsets
    pub fn bake_rotation(&mut self) {
        let mat = self.orientation.to_matrix();

        for child in self.children.iter_mut() {
            let x = child.position.x;
            let y = child.position.y;
            let z = child.position.z;

            child.position = Vector3::new(
                mat.m0 * x + mat.m4 * y + mat.m8 * z,
                mat.m1 * x + mat.m5 * y + mat.m9 * z,
                mat.m2 * x + mat.m6 * y + mat.m10 * z,
            );

            child.position.x = (child.position.x * 100.0).round() / 100.0;
            child.position.y = (child.position.y * 100.0).round() / 100.0;
            child.position.z = (child.position.z * 100.0).round() / 100.0;
        }

        if let Some(local) = self.endpoint_local {
            let baked = Vector3::new(
                mat.m0 * local.x + mat.m4 * local.y + mat.m8 * local.z,
                mat.m1 * local.x + mat.m5 * local.y + mat.m9 * local.z,
                mat.m2 * local.x + mat.m6 * local.y + mat.m10 * local.z,
            );
            self.endpoint_local = Some(Vector3::new(
                (baked.x * 100.0).round() / 100.0,
                (baked.y * 100.0).round() / 100.0,
                (baked.z * 100.0).round() / 100.0,
            ));
        }

        let x = self.model_offset.x;
        let y = self.model_offset.y;
        let z = self.model_offset.z;
        let baked_x = mat.m0 * x + mat.m1 * y + mat.m2 * z;
        let baked_y = mat.m4 * x + mat.m5 * y + mat.m6 * z;
        let baked_z = mat.m8 * x + mat.m9 * y + mat.m10 * z;

        self.model_offset = Vector3::new(
            (baked_x * 100.0).round() / 100.0,
            (baked_y * 100.0).round() / 100.0,
            (baked_z * 100.0).round() / 100.0,
        );

        self.model_orientation = self.orientation * self.model_orientation;
        self.orientation = Quaternion::identity();
        self.target_orientation = Quaternion::identity();

        self.sync_colliders();
    }

    /// Synchronizes the position of all block colliders with their current world positions.
    pub fn sync_colliders(&mut self) {
        let current_rot = if self.is_rotating {
            self.orientation
                .slerp(self.target_orientation, self.rotation_progress)
        } else {
            self.orientation
        };

        let mat = current_rot.to_matrix();

        for child in self.children.iter_mut() {
            let x = child.position.x;
            let y = child.position.y;
            let z = child.position.z;

            let rx = mat.m0 * x + mat.m4 * y + mat.m8 * z;
            let ry = mat.m1 * x + mat.m5 * y + mat.m9 * z;
            let rz = mat.m2 * x + mat.m6 * y + mat.m10 * z;

            child.collider.offset = Vector3::new(
                self.position.x + rx,
                self.position.y + ry,
                self.position.z + rz,
            );

            let hs = child.size * 0.5;

            let new_hs = Vector3::new(
                (mat.m0 * hs.x).abs() + (mat.m4 * hs.y).abs() + (mat.m8 * hs.z).abs(),
                (mat.m1 * hs.x).abs() + (mat.m5 * hs.y).abs() + (mat.m9 * hs.z).abs(),
                (mat.m2 * hs.x).abs() + (mat.m6 * hs.y).abs() + (mat.m10 * hs.z).abs(),
            );

            child.collider.shape = CollisionShape::Box { half_size: new_hs };
        }
    }

    /// Renders visual indicators for the movement path and destination when a group is being dragged.
    pub fn draw_drag_guides(&self, d: &mut impl RaylibDraw3D) {
        if !self.is_dragging {
            return;
        }
        let axis = self.end_pos - self.start_pos;
        let current_v = self.position - self.start_pos;
        let progress = current_v.dot(axis) / axis.dot(axis);

        if progress > 0.5 {
            Color::LIME
        } else {
            Color::WHITE
        };

        unsafe {
            raylib::ffi::rlPushMatrix();
            raylib::ffi::rlTranslatef(self.end_pos.x, self.end_pos.y, self.end_pos.z);
            let mat = self.orientation.to_matrix();

            let matrix_array: [f32; 16] = [
                mat.m0, mat.m4, mat.m8, mat.m12, mat.m1, mat.m5, mat.m9, mat.m13, mat.m2, mat.m6,
                mat.m10, mat.m14, mat.m3, mat.m7, mat.m11, mat.m15,
            ];

            raylib::ffi::rlMultMatrixf(matrix_array.as_ptr());

            for child in &self.children {
                d.draw_cube_wires(
                    child.position,
                    child.size.x,
                    child.size.y,
                    child.size.z,
                    Color::LIME.alpha(0.4),
                );
            }
            raylib::ffi::rlPopMatrix();
        }

        let segments = 10;
        for i in 0..=segments {
            let t = i as f32 / segments as f32;
            let dot_pos = self.start_pos.lerp(self.end_pos, t);
            d.draw_sphere(dot_pos, 0.05, Color::WHITE);
        }
    }

    /// Sets the destination position for groups that support dragging.
    pub fn with_end_pos(mut self, end: Vector3) -> Self {
        self.end_pos = end;
        self
    }
}

///Renders a 3D cube with a texture applied to each of its six faces
fn draw_cube_with_texture(
    tex: &Texture2D,
    position: Vector3,
    width: f32,
    height: f32,
    length: f32,
    color: Color,
) {
    let (x, y, z) = (position.x, position.y, position.z);
    let (w, h, l) = (width / 2.0, height / 2.0, length / 2.0);

    let front_down_left = (x - w, y - h, z + l);
    let front_down_right = (x + w, y - h, z + l);
    let front_up_left = (x - w, y + h, z + l);
    let front_up_right = (x + w, y + h, z + l);

    let arr_down_left = (x - w, y - h, z - l);
    let arr_down_right = (x + w, y - h, z - l);
    let arr_up_left = (x - w, y + h, z - l);
    let arr_up_right = (x + w, y + h, z - l);

    unsafe {
        ffi::rlSetTexture(tex.id);
        ffi::rlBegin(ffi::RL_QUADS as i32);
        ffi::rlColor4ub(color.r, color.g, color.b, color.a);

        draw_face(
            front_down_left,
            front_down_right,
            front_up_right,
            front_up_left,
        ); // front
        draw_face(arr_down_right, arr_down_left, arr_up_left, arr_up_right); // back
        draw_face(arr_up_left, front_up_left, front_up_right, arr_up_right); // up
        draw_face(
            arr_down_right,
            front_down_right,
            front_down_left,
            arr_down_left,
        ); // down
        draw_face(
            front_down_right,
            arr_down_right,
            arr_up_right,
            front_up_right,
        ); // right
        draw_face(arr_down_left, front_down_left, front_up_left, arr_up_left); // left

        ffi::rlEnd();
        ffi::rlSetTexture(0);
    }
}

/// Draws a single quad face using vertex positions and standard UV coordinates.
unsafe fn draw_face(
    down_left: (f32, f32, f32),
    down_right: (f32, f32, f32),
    up_right: (f32, f32, f32),
    up_left: (f32, f32, f32),
) {
    ffi::rlTexCoord2f(0.0, 1.0);
    ffi::rlVertex3f(down_left.0, down_left.1, down_left.2);
    ffi::rlTexCoord2f(1.0, 1.0);
    ffi::rlVertex3f(down_right.0, down_right.1, down_right.2);
    ffi::rlTexCoord2f(1.0, 0.0);
    ffi::rlVertex3f(up_right.0, up_right.1, up_right.2);
    ffi::rlTexCoord2f(0.0, 0.0);
    ffi::rlVertex3f(up_left.0, up_left.1, up_left.2);
}
