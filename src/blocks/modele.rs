use raylib::prelude::*;

// Permet d'activer l'autorisation de la comparaison entre valeurs de BlockType
#[derive(PartialEq)]
pub enum BlockType {
    Fixe,
    RotationV,
    RotationH,
    All,
}

// Block type avec sa fonction d'implementation
pub struct BlockPrefab {
    pub position: crate::Vector3,
    pub size: crate::Vector3,
    pub color: crate::Color,
    pub base_color: crate::Color,
    pub block_type: BlockType,
    pub current_orientation: Quaternion,
    pub target_orientation: Quaternion,
    pub is_rotating: bool,
    pub rotation_progress: f32,
}

impl BlockPrefab {
    pub fn new(x: f32, y: f32, z: f32, color: crate::Color, block_type: BlockType) -> Self {
        Self {
            position: crate::Vector3::new(x, y, z),
            size: crate::Vector3::new(1.0, 1.0, 1.0),
            color: color,
            base_color: color,
            block_type: block_type,
            current_orientation: Quaternion::identity(),
            target_orientation: Quaternion::identity(),
            rotation_progress: 1.0,
            is_rotating: false,
        }
    }

    // Fonction qui dessine le cube en fonction de ses paramètres
    pub fn draw(&self, d: &mut crate::RaylibMode3D<crate::RaylibDrawHandle>) {
        // On calcule l'orientation intermédiaire pour l'animation
        let animated_orientation = self
            .current_orientation
            .slerp(self.target_orientation, self.rotation_progress);

        // Conversion en matrice 4x4
        let mat = animated_orientation.to_matrix();

        let matrix_array: [f32; 16] = [
            mat.m0, mat.m4, mat.m8, mat.m12, // Colonne 1
            mat.m1, mat.m5, mat.m9, mat.m13, // Colonne 2
            mat.m2, mat.m6, mat.m10, mat.m14, // Colonne 3
            mat.m3, mat.m7, mat.m11, mat.m15, // Colonne 4
        ];

        unsafe {
            raylib::ffi::rlPushMatrix();
            raylib::ffi::rlTranslatef(self.position.x, self.position.y, self.position.z);

            raylib::ffi::rlMultMatrixf(matrix_array.as_ptr());

            d.draw_cube(
                Vector3::zero(),
                self.size.x,
                self.size.y,
                self.size.z,
                self.color,
            );
            d.draw_cube_wires(
                Vector3::zero(),
                self.size.x,
                self.size.y,
                self.size.z,
                Color::BLACK,
            );

            raylib::ffi::rlPopMatrix();
        }
    }

    // Fonction qui vérifie si la souris est sur le cube (et donc le rend jaune pur l'instant)
    // Il prend en entrée le raylibHandle et la caméra
    // Retourne vrai si la souris est sur le cube, false sinon
    pub fn is_mouse_over(&self, rl: &RaylibHandle, camera: &Camera3D) -> bool {
        let ray = rl.get_screen_to_world_ray(rl.get_mouse_position(), camera);

        let half_size = self.size * 0.5;
        let bbox = BoundingBox::new(
            Vector3::new(
                self.position.x - half_size.x,
                self.position.y - half_size.y,
                self.position.z - half_size.z,
            ),
            Vector3::new(
                self.position.x + half_size.x,
                self.position.y + half_size.y,
                self.position.z + half_size.z,
            ),
        );

        let collision = bbox.get_ray_collision_box(ray);
        collision.hit
    }

    pub fn update_animation(&mut self, dt: f32) {
        if !self.is_rotating {
            return;
        }

        self.rotation_progress += 3.0 * dt; // Vitesse de l'animation

        if self.rotation_progress >= 1.0 {
            self.rotation_progress = 1.0;
            self.current_orientation = self.target_orientation;
            self.is_rotating = false;
        }
    }
}
