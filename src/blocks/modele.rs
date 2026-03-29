use crate::blocks;
use crate::blocks::material::{self, BlockMaterial};
use raylib::prelude::*;

// Permet d'activer l'autorisation de la comparaison entre valeurs de BlockType
#[derive(PartialEq, Clone, Copy)]
pub enum BlockType {
    Fixe,
    RotationV,
    RotationH,
    Drag,
    All,
}

// Block type avec sa fonction d'implementation
pub struct BlockPrefab {
    pub position: crate::Vector3,
    pub size: crate::Vector3,
    pub block_type: BlockType,
    pub material: material::BlockMaterial,
    pub temp_color: Option<Color>,
}

impl BlockPrefab {
    pub fn new(
        pos: Vector3,
        size: Option<Vector3>,
        block_type: BlockType,
        material: BlockMaterial,
    ) -> Self {
        Self {
            position: pos,
            size: size.unwrap_or(Vector3::new(1.0, 1.0, 1.0)),
            block_type,
            material: material,
            temp_color: None,
        }
    }

    pub fn get_current_color(&self) -> Color {
        self.material.color
    }
}

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
}

impl GroupBlock {
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
            // Initialisation du drag
            start_pos: pos,
            end_pos: pos,
            is_dragging: false,
            drag_timer: 0.0,
        }
    }

    pub fn single(pos: Vector3, block_type: BlockType, material: BlockMaterial) -> Self {
        let child = BlockPrefab::new(Vector3::ZERO, None, block_type.clone(), material);
        Self::new(pos, vec![child], block_type)
    }

    // Applique une couleur à tous les enfants (pour le feedback visuel)
    pub fn set_temporary_color(&mut self, color: Color) {
        for child in self.children.iter_mut() {
            child.temp_color = Some(color);
        }
    }

    // Rend aux enfants leur base_color
    pub fn reset_color(&mut self) {
        for child in self.children.iter_mut() {
            child.temp_color = None;
        }
    }

    pub fn draw(&self, d: &mut RaylibMode3D<RaylibDrawHandle>) {
        // Calcul de l'orientation animée du groupe
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
            // On déplace tout le groupe à sa position mondiale
            raylib::ffi::rlTranslatef(self.position.x, self.position.y, self.position.z);
            // On applique la rotation du groupe
            raylib::ffi::rlMultMatrixf(matrix_array.as_ptr());

            for child in &self.children {
                let color_to_draw = child.temp_color.unwrap_or(child.material.color);
                // On dessine l'enfant à sa position RELATIVE
                d.draw_cube(
                    child.position,
                    child.size.x,
                    child.size.y,
                    child.size.z,
                    color_to_draw,
                );
                d.draw_cube_wires(
                    child.position,
                    child.size.x,
                    child.size.y,
                    child.size.z,
                    Color::BLACK,
                );
            }
            raylib::ffi::rlPopMatrix();
        }
    }
    pub fn is_mouse_over(&self, rl: &RaylibHandle, camera: &Camera3D) -> bool {
        let ray = rl.get_screen_to_world_ray(rl.get_mouse_position(), camera);

        for child in &self.children {
            // Note: On doit tester la position MONDE du cube : groupe.pos + enfant.pos
            // (Attention: Si rotation, il faut transformer child.position par l'orientation)
            let world_child_pos = self.position + child.position;

            let half_size = child.size * 0.5;
            let bbox = BoundingBox::new(world_child_pos - half_size, world_child_pos + half_size);
            if bbox.get_ray_collision_box(ray).hit {
                return true;
            }
        }
        false
    }

    pub fn update_animation(&mut self, dt: f32) {
        if self.is_rotating {
            self.rotation_progress += 3.0 * dt; // Vitesse de rotation
            if self.rotation_progress >= 1.0 {
                self.rotation_progress = 1.0;
                self.orientation = self.target_orientation;
                self.is_rotating = false;
            }
        }
    }

    pub fn draw_drag_guides(&self, d: &mut RaylibMode3D<RaylibDrawHandle>) {
        if !self.is_dragging {
            return;
        }
        let axis = self.end_pos - self.start_pos;
        let current_v = self.position - self.start_pos;
        let progress = current_v.dot(axis) / axis.dot(axis);

        let dot_color = if progress > 0.5 {
            Color::LIME
        } else {
            Color::WHITE
        };

        unsafe {
            raylib::ffi::rlPushMatrix();
            raylib::ffi::rlTranslatef(self.end_pos.x, self.end_pos.y, self.end_pos.z);
            // On applique la rotation actuelle pour le fantôme
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

        // Dessiner les petits points de trajectoire
        let segments = 10;
        for i in 0..=segments {
            let t = i as f32 / segments as f32;
            let dot_pos = self.start_pos.lerp(self.end_pos, t);
            d.draw_sphere(dot_pos, 0.05, Color::WHITE);
        }
    }

    pub fn with_end_pos(mut self, end: Vector3) -> Self {
        self.end_pos = end;
        self
    }
}
