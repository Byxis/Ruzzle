use crate::multiplayer::Position;
use serde::{Deserialize, Serialize};

/// Enum to represent the messages sended between the client and the server
#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum Message {
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
