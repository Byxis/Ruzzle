use room::{Player, Position, PositionUpdate, RoomManager};
use serde::{Deserialize, Serialize};

use crate::room;

/// Messages that are exchanged between the client and the server.
#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum RoomMessage {
    // Client -> Server
    CreateRoom {
        player_id: u64,
    },
    JoinRoom {
        room_id: u64,
        player_id: u64,
    },
    UpdatePosition {
        position: PositionUpdate,
    },

    // Server -> Client
    RoomCreated {
        room_id: u64,
    },
    RoomJoined {
        room_id: u64,
        other_player: Option<String>,
    },
    RoomJoinFailed {
        reason: String,
    },
    PlayerLeft {
        player_name: String,
    },
}

pub fn treat_message(msg: RoomMessage, room_manager: &mut RoomManager) -> RoomMessage {
    match msg {
        RoomMessage::CreateRoom { player_id } => {
            println!("Creating room for player: {}", player_id);
            let room_id = room_manager.create_room();
            let _ = room_manager.add_player_to_room(
                player_id,
                Player {
                    id: player_id,
                    name: format!("Player{}", player_id),
                    position: Position::new(0.0, 0.0, 0.0, 0.0),
                },
                room_id,
            );
            RoomMessage::RoomCreated { room_id }
        }
        RoomMessage::JoinRoom { room_id, player_id } => {
            println!("Player {} is trying to join room {}", player_id, room_id);
            match room_manager.add_player_to_room(
                player_id,
                Player {
                    id: player_id,
                    name: format!("Player{}", player_id),
                    position: Position::new(0.0, 0.0, 0.0, 0.0),
                },
                room_id,
            ) {
                Ok(_) => RoomMessage::RoomJoined {
                    room_id,
                    other_player: Some(format!("Player{}", player_id)),
                },
                Err(e) => RoomMessage::RoomJoinFailed {
                    reason: e.to_string(),
                },
            }
        }
        RoomMessage::UpdatePosition { position } => {
            println!("Updating position for player: {}", position.client_id);
            let _ = room_manager.update_player_position(position.client_id, position.position);
            RoomMessage::UpdatePosition { position }
        }
        _ => {
            println!("Received unhandled message: {:?}", msg);
            msg
        }
    }
}
