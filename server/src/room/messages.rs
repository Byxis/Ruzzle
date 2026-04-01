use room::{Player, Position, RoomManager};
use serde::{Deserialize, Serialize};

use crate::room;

/// Messages that are exchanged between the client and the server.
#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum RoomMessage {
    // Client -> Server
    CreateRoom,
    JoinRoom { room_id: u64 },
    UpdatePosition { position: Position },

    // Server -> Client
    RoomCreated { room_id: u64 },
    RoomJoined { room_id: u64 },
    RoomJoinFailed { reason: String },
    PlayerLeft { player_name: String },
}

/// function to treat the messages received by the server and return a response message if needed
pub fn treat_message(msg: RoomMessage, player: u64, room_manager: &mut RoomManager) -> RoomMessage {
    match msg {
        RoomMessage::CreateRoom => {
            println!("Creating room for player: {}", player);
            let room_id = room_manager.create_room();
            let _ = room_manager.add_player_to_room(
                player,
                Player {
                    id: player,
                    name: format!("Player{}", player),
                    position: Position::new(0.0, 0.0, 0.0, 0.0),
                },
                room_id,
            );
            RoomMessage::RoomCreated { room_id }
        }
        RoomMessage::JoinRoom { room_id } => {
            println!("Player {} is trying to join room {}", player, room_id);
            match room_manager.add_player_to_room(
                player,
                Player {
                    id: player,
                    name: format!("Player{}", player),
                    position: Position::new(0.0, 0.0, 0.0, 0.0),
                },
                room_id,
            ) {
                Ok(_) => RoomMessage::RoomJoined { room_id },
                Err(e) => RoomMessage::RoomJoinFailed {
                    reason: e.to_string(),
                },
            }
        }
        RoomMessage::UpdatePosition { position } => {
            // println!("Updating position for player: {} to ({}, {}, {})", player, position.x, position.y, position.z);
            let _ = room_manager.update_player_position(player, position);
            RoomMessage::UpdatePosition { position }
        }
        _ => {
            println!("Received unhandled message: {:?}", msg);
            msg
        }
    }
}
