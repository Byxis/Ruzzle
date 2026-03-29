use raylib::prelude::*;

#[derive(Clone, Copy)]
pub struct BlockMaterial {
    pub color: Color,
    pub texture_id: Option<i32>, // ID de la texture chargée en mémoire
    pub roughness: f32,          // Pour des effets de lumière plus tard
}

impl BlockMaterial {
    pub fn sand() -> Self {
        Self {
            color: Color::new(194, 178, 128, 255), // Couleur sable
            texture_id: Some(0), // Supposons que 0 est l'ID de ta texture de sable
            roughness: 0.8,
        }
    }

    pub fn metal() -> Self {
        Self {
            color: Color::LIGHTGRAY,
            texture_id: Some(1),
            roughness: 0.2,
        }
    }
}
