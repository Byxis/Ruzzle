use serde::{Deserialize, Serialize};

/// Struct to store the data of a player connected to the server
/// The id is the ClientId of the player
#[derive(Clone)]
pub struct Player {
    pub id: u64,
    pub name: String,
    pub position: Position,
}

/// Struct to store the position of a player in the game
/// 3 axis position (x,y,z) and a rotation for the direction of the player
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
