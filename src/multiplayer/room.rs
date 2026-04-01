use crate::multiplayer::messages::Message;
use crate::multiplayer::Position;
use renet::DefaultChannel;

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum RoomStatus {
    WaitingRoom,
    InGame(i32), // the number represents the level
    // Loading,
}

/// Struct to store all the information about the current multiplayer session
/// Not used that much for now, but could be useful in the future
pub struct Room {
    pub room_id: u64,
    // pub local_player: Option<&'a mut Crab>,
    // pub other_player: Option<&'a mut Crab>,
    pub status: RoomStatus,
    pub tx: Option<std::sync::mpsc::Sender<Position>>,
    pub room_joined: bool,
}

impl Room {
    pub fn new(room_id: u64, tx: std::sync::mpsc::Sender<Position>) -> Self {
        Self {
            room_id,
            // local_player: Some(local_player),
            // other_player: None,
            status: RoomStatus::WaitingRoom,
            tx: Some(tx),
            room_joined: false,
        }
    }

    // Function to initialise or join a room in the server
    pub fn init_room(&mut self, is_host: bool, client: &mut renet::RenetClient) {
        if self.room_joined {
            return;
        }
        match is_host {
            true => {
                let msg = Message::CreateRoom;
                let serialized = bincode::serialize(&msg).unwrap();
                client.send_message(DefaultChannel::ReliableOrdered, serialized);
                self.room_joined = true;
            }
            false => {
                let msg = Message::JoinRoom {
                    room_id: self.room_id,
                };
                let serialized = bincode::serialize(&msg).unwrap();
                client.send_message(DefaultChannel::ReliableOrdered, serialized);
                self.room_joined = true;
            }
        }
    }

    // Function to treat the messages received from the server
    pub fn treat_message(&mut self, msg: Message) {
        match msg {
            Message::UpdatePosition { position } => {
                println!("Updating position for other player: {:?}", position);
                if let Some(ref tx) = self.tx {
                    let _ = tx.send(position); // Send remote player position to main thread
                }
            }
            Message::RoomCreated { room_id } => {
                self.room_id = room_id;
                self.status = RoomStatus::InGame(0);
            }
            Message::RoomJoined { room_id } => {
                self.room_id = room_id;
                self.status = RoomStatus::InGame(0);
            }
            Message::RoomJoinFailed { reason } => {
                println!("Error: {}", reason);
            }
            Message::PlayerLeft { player_name } => {
                println!("Player left: {}", player_name);
            }
            _ => {}
        }
    }
}
