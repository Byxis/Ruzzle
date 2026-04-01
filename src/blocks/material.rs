use raylib::prelude::*;

/// Defines the visual appearance of a block, including its base color and texture.
#[derive(Clone, Copy)]
pub struct BlockMaterial {
    pub color: Color,
    pub texture_id: Option<i32>,
}

impl BlockMaterial {
    /// Creates a material with a yellow tint and the sand texture (ID: 0).
    pub fn sand() -> Self {
        Self {
            color: Color::YELLOW,
            texture_id: Some(0),
        }
    }

    /// Creates a material with a green tint and the grass texture (ID: 1).
    pub fn grass() -> Self {
        Self {
            color: Color::GREEN,
            texture_id: Some(1),
        }
    }
}
