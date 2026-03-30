use raylib::{ffi::CSSPalette, prelude::*};

#[derive(Clone, Copy)]
pub struct BlockMaterial {
    pub color: Color,
    pub texture_id: Option<i32>,
}

impl BlockMaterial {
    pub fn sand() -> Self {
        Self {
            color: Color::YELLOW,
            texture_id: Some(0),
        }
    }

    pub fn grass() -> Self {
        Self {
            color: Color::GREEN,
            texture_id: Some(1),
        }
    }
}
