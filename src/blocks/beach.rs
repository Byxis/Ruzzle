use raylib::prelude::*;

// Permet d'activer l'autorisation de la comparaison entre valeurs de BlockType
#[derive(PartialEq)]
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
    pub color: crate::Color,
    pub base_color: crate::Color,
    pub block_type: BlockType,
    pub current_orientation: Quaternion,
    pub target_orientation: Quaternion,
    pub is_rotating: bool,
    pub rotation_progress: f32,
    pub start_pos: Vector3, // Position A
    pub end_pos: Vector3,   // Position B
    pub is_dragging: bool,
    pub drag_timer: f32,
}

impl BlockPrefab {
    pub fn new(
        pos: Vector3,
        end_pos: Option<Vector3>,
        color: crate::Color,
        size: Option<Vector3>,
        block_type: BlockType,
    ) -> Self {
        Self {
            position: pos,
            start_pos: pos,
            end_pos: end_pos.unwrap_or(crate::Vector3::new(pos.x, pos.y, pos.z)), // Permet de mettre une valeur si None
            size: size.unwrap_or(crate::Vector3::new(1.0, 1.0, 1.0)),
            color: color,
            base_color: color,
            block_type: block_type,
            current_orientation: Quaternion::identity(),
            target_orientation: Quaternion::identity(),
            rotation_progress: 1.0,
            is_rotating: false,
            is_dragging: false,
            drag_timer: 0.0,
        }
    }

    // Fonction qui dessine le cube en fonction de ses paramètres
    pub fn draw(&self, d: &mut crate::RaylibMode3D<crate::RaylibDrawHandle>) {
        // On calcule l'orientation pour l'animation
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
                Vector3::ZERO,
                self.size.x,
                self.size.y,
                self.size.z,
                self.color,
            );
            d.draw_cube_wires(
                Vector3::ZERO,
                self.size.x,
                self.size.y,
                self.size.z,
                Color::BLACK,
            );

            raylib::ffi::rlPopMatrix();
        }
    }

    // Affiche les aides visuelles
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

        // Dessiner le cube fantôme à la position de fin (semi-transparent)
        d.draw_cube(
            self.end_pos,
            self.size.x,
            self.size.y,
            self.size.z,
            self.base_color.alpha(0.3),
        );
        d.draw_cube_wires(
            self.end_pos,
            self.size.x,
            self.size.y,
            self.size.z,
            dot_color,
        );

        // Dessiner les petits points de trajectoire
        let segments = 10;
        for i in 0..=segments {
            let t = i as f32 / segments as f32;
            let dot_pos = self.start_pos.lerp(self.end_pos, t);
            d.draw_sphere(dot_pos, 0.05, Color::WHITE);
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
