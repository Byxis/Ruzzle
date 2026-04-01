use serde::{Deserialize, Serialize};

#[derive(Clone)]
pub struct Player {
    pub id: u64,
    pub name: String,
    pub position: Position,
}

#[derive(Serialize, Deserialize, Clone, Copy, Debug)]
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

impl Player {
    pub fn new(id: u64, name: String) -> Self {
        Self {
            id,
            name,
            position: Position::new(0.0, 0.0, 0.0, 0.0),
        }
    }

    pub fn update_position(&mut self, new_position: Position) {
        self.position = new_position;
    }

    pub fn get_position(&self) -> Position {
        self.position
    }
}
