use crate::components::transform::Transform3D;
use crate::multiplayer::messages::Message;
use crate::Crab;
use raylib::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum RoomStatus {
    WaitingRoom,
    InGame(i32), // the number represents the level
    Loading,
}

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

pub struct Room<'a> {
    pub room_id: u64,
    pub local_player: Option<&'a mut Crab>,
    pub other_player: Option<&'a mut Crab>,
    pub status: RoomStatus,
    pub tx: Option<std::sync::mpsc::Sender<Position>>,
}

impl<'a> Room<'a> {
    pub fn new(room_id: u64, local_player: &'a mut Crab, tx: std::sync::mpsc::Sender<Position>) -> Self {
        Self {
            room_id,
            local_player: Some(local_player),
            other_player: None,
            status: RoomStatus::WaitingRoom,
            tx: Some(tx),
        }
    }

    pub fn add_other_player(&mut self, crab: &'a mut Crab) {
        self.other_player = Some(crab);
        self.status = RoomStatus::InGame(1);
    }

    pub fn update_other_player_position(&mut self, position: Position) {
        if let Some(ref mut other) = self.other_player {
            other.teleport(Transform3D::new(
                Vector3::new(position.x, position.y, position.z),
                position.rotation,
            ));
        }
    }

    pub fn crab_to_position(&mut self) -> Position {
        let pos = self.local_player.as_ref().unwrap().transform.position;
        Position::new(pos.x, pos.y, pos.z, self.local_player.as_ref().unwrap().transform.rotation)
    }

    pub fn treat_message(&mut self, msg: Message) {
        match msg {
            Message::UpdatePosition { position } => {
                println!("Updating position for other player: {:?}", position);
                self.update_other_player_position(position);
            },
            Message::RoomCreated { room_id } => {
                self.room_id = room_id;
                self.status = RoomStatus::WaitingRoom;
            },
            Message::RoomJoined { room_id } => {
                self.room_id = room_id;
                self.status = RoomStatus::WaitingRoom;
            },
            Message::RoomJoinFailed { reason } => {
                println!("Error: {}", reason);
            },
            Message::PlayerLeft { player_name } => {
                println!("Player left: {}", player_name);
            },
            _ => {}
        }
    }
}