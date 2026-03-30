use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum RoomMessage {
    // Client -> Server
    CreateRoom { player_name: String },
    JoinRoom { room_id: u64, player_name: String },
    
    // Server -> Client
    RoomCreated { room_id: u64 },
    RoomJoined { room_id: u64, other_player: Option<String> },
    RoomJoinFailed { reason: String },
    PlayerJoined { player_name: String },
    PlayerLeft { player_name: String },
}