use serde::{Deserialize, Serialize};

/// Struct to easily send the position of a player between the client and the server
/// Inspired by the transform component already present.
#[derive(Serialize, Deserialize, Copy, Clone, Debug, PartialEq)]
pub struct Position {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub rotation: f32,
}

impl Position {
    pub fn new(x: f32, y: f32, z: f32, rotation: f32) -> Self {
        Self { x, y, z, rotation }
    }
}
