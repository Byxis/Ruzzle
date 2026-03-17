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
    pub rotation: Vector3,
    pub target_rotation_x: f32,
    pub target_rotation_y: f32,
    pub is_rotating: bool,
}

impl BlockPrefab {
    pub fn new(x: f32, y: f32, z: f32, color: crate::Color, block_type: BlockType) -> Self {
        Self {
            position: crate::Vector3::new(x, y, z),
            size: crate::Vector3::new(1.0, 1.0, 1.0),
            color: color,
            base_color: color,
            block_type: block_type,
            rotation: Vector3::zero(),
            target_rotation_x: 0.0,
            target_rotation_y: 0.0,
            is_rotating: false,
        }
    }

    // Fonction qui dessine le cube en fonction de ses paramètres
    pub fn draw(&self, d: &mut crate::RaylibMode3D<crate::RaylibDrawHandle>) {
        unsafe {
            raylib::ffi::rlPushMatrix();

            // 1. Déplacement à la position du cube dans le monde
            raylib::ffi::rlTranslatef(self.position.x, self.position.y, self.position.z);

            // 2. ON APPLIQUE D'ABORD LA ROTATION HORIZONTALE (Monde)
            // Cela garantit que "gauche/droite" tourne toujours autour du poteau vertical du monde
            raylib::ffi::rlRotatef(self.rotation.y, 0.0, 1.0, 0.0);

            // 3. ENSUITE ON APPLIQUE LA ROTATION VERTICALE
            raylib::ffi::rlRotatef(self.rotation.x, 1.0, 0.0, 0.0);

            // 4. Dessin
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

        let speed = 400.0 * dt;
        let mut moved = false;

        // --- Gestion Axe X ---
        let diff_x = self.target_rotation_x - self.rotation.x;
        if diff_x.abs() > 0.1 {
            // Si l'écart est significatif
            if diff_x.abs() <= speed {
                self.rotation.x = self.target_rotation_x; // On arrive pile dessus
            } else {
                self.rotation.x += diff_x.signum() * speed;
                moved = true;
            }
        }

        // --- Gestion Axe Y ---
        let diff_y = self.target_rotation_y - self.rotation.y;
        if diff_y.abs() > 0.1 {
            if diff_y.abs() <= speed {
                self.rotation.y = self.target_rotation_y; // On arrive pile dessus
            } else {
                self.rotation.y += diff_y.signum() * speed;
                moved = true;
            }
        }

        // Si aucun mouvement n'a été effectué sur les deux axes, on arrête
        if !moved {
            self.is_rotating = false;
            // On s'assure que les valeurs sont parfaitement égales à la cible
            self.rotation.x = self.target_rotation_x;
            self.rotation.y = self.target_rotation_y;
        }
    }
}
