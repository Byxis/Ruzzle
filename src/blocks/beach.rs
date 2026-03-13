use raylib::prelude::*;

enum BlockType {
    Fix,
    Rotation,
    All,
}

// Block type avec sa fonction d'implementation
pub struct BlockPrefab {
    pub position: crate::Vector3,
    pub size: crate::Vector3,
    pub color: crate::Color,
    pub base_color: crate::Color,
    pub block_type: BlockType,
}

impl BlockPrefab {
    pub fn new(x: f32, y: f32, z: f32, color: crate::Color) -> Self {
        Self {
            position: crate::Vector3::new(x, y, z),
            size: crate::Vector3::new(1.0, 1.0, 1.0),
            color: color,
            base_color: color,
            block_type: BlockType::All,
        }
    }

    // Fonction qui dessine le cube en fonction de ses paramètres
    pub fn draw(&self, d: &mut crate::RaylibMode3D<crate::RaylibDrawHandle>) {
        d.draw_cube(
            self.position,
            self.size.x,
            self.size.y,
            self.size.z,
            self.color,
        );
        d.draw_cube_wires(
            self.position,
            self.size.x,
            self.size.y,
            self.size.z,
            crate::Color::BLACK,
        );
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
}
