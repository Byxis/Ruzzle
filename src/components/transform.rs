use raylib::prelude::*;

#[derive(Clone, Copy)]
pub struct Transform3D {
    pub position: Vector3,
    pub rotation: f32,
}

impl Transform3D {
    pub const IDENTITY: Self = Self {
        position: Vector3::ZERO,
        rotation: 0.0,
    };

    pub fn new(position: Vector3, rotation: f32) -> Self {
        Self { position, rotation }
    }
}
