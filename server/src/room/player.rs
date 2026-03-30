use serde::{Serialize, Deserialize};

#[derive(Clone)]
pub struct Player {
    pub id: u64,
    pub name: String,
    pub position: PositionUpdate,
}

#[derive(Serialize, Deserialize, Clone, Copy)]
pub struct PositionUpdate {
    pub client_id: u64,
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub rotation: f32,
}

impl PositionUpdate {
    pub fn from_transform(client_id: u64, transform: PositionUpdate) -> Self {
        Self {
            client_id,
            x: transform.x,
            y: transform.y,
            z: transform.z,
            rotation: transform.rotation,
        }
    }
}

impl Player {
    pub fn new(id: u64, name: String) -> Self {
        Self {
            id,
            name,
            position: PositionUpdate::from_transform(id, PositionUpdate { client_id: id, x: 0.0, y: 0.0, z: 0.0, rotation: 0.0 }),
        }
    }

    pub fn update_position(&mut self, new_position: PositionUpdate) {
        self.position = PositionUpdate::from_transform(self.id, new_position);
    }

    pub fn get_position(&self) -> PositionUpdate {
        self.position
    }
}