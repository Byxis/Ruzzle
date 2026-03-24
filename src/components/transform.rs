use raylib::prelude::*;

/// Represents a 3D transform with a position and rotation.
///
/// The `IDENTITY` constant provides a transform with zero position and rotation.
///
/// # Examples
///
/// ```
/// use raylib::prelude::*;
/// use ruzzle::components::Transform3D;
///
/// let transform = Transform3D::new(Vector3::new(1.0, 2.0, 3.0), 45.0);
/// let identity = Transform3D::IDENTITY;
/// ```
#[derive(Clone, Copy)]
pub struct Transform3D {
    pub position: Vector3,
    pub rotation: f32,
}

impl Transform3D {
    /// The identity transform with zero position and rotation.
    pub const IDENTITY: Self = Self {
        position: Vector3::ZERO,
        rotation: 0.0,
    };

    /// Creates a new transform with the given position and rotation.
    pub fn new(position: Vector3, rotation: f32) -> Self {
        Self { position, rotation }
    }
}
