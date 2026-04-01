use std::{
    net::{SocketAddr, UdpSocket},
    sync::mpsc::{Sender, Receiver},
    thread,
    time::{Duration, Instant, SystemTime},
};

use renet::{ConnectionConfig, DefaultChannel, RenetClient};
use renet_netcode::{
    ClientAuthentication, NetcodeClientTransport, NETCODE_USER_DATA_BYTES,
};
use crate::multiplayer::{Message, Position};

// Helper struct to pass an username in the user data
struct Username(String);

impl Username {
    fn to_netcode_user_data(&self) -> [u8; NETCODE_USER_DATA_BYTES] {
        let mut user_data = [0u8; NETCODE_USER_DATA_BYTES];
        if self.0.len() > NETCODE_USER_DATA_BYTES - 8 {
            panic!("Username is too big");
        }
        user_data[0..8].copy_from_slice(&(self.0.len() as u64).to_le_bytes());
        user_data[8..self.0.len() + 8].copy_from_slice(self.0.as_bytes());
        user_data
    }
}

pub fn client(tx: Sender<Position>, rx: Receiver<Position>, is_host: bool) {
    let connection_config = ConnectionConfig::default();
    let mut client = RenetClient::new(connection_config);

    let server_addr: SocketAddr = "127.0.0.1:1312".parse().unwrap();
    let username = Username("Player1".to_string());
    let protocol_id: u64 = 7;

    let socket = UdpSocket::bind("127.0.0.1:0").unwrap();
    let current_time = SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap();
    let client_id = current_time.as_millis() as u64;
    let authentication = ClientAuthentication::Unsecure {
        server_addr,
        client_id,
        user_data: Some(username.to_netcode_user_data()),
        protocol_id,
    };

    let mut transport = NetcodeClientTransport::new(current_time, authentication, socket).unwrap();

    let mut last_updated = Instant::now();

    let mut room_joined = false;

    loop {
        let now = Instant::now();
        let duration = now - last_updated;
        last_updated = now;

        client.update(duration);
        transport.update(duration, &mut client).unwrap();


        if client.is_connected() {
            if !room_joined && is_host {
                let msg = Message::CreateRoom;
                let serialized = bincode::serialize(&msg).unwrap();
                client.send_message(DefaultChannel::ReliableOrdered, serialized);
                room_joined = true;
            }
            if !room_joined && !is_host {
                let msg = Message::JoinRoom { room_id: 1 };
                let serialized = bincode::serialize(&msg).unwrap();
                client.send_message(DefaultChannel::ReliableOrdered, serialized);
                room_joined = true;
            }

            // Receive local position from main thread and send to server
            if let Ok(position) = rx.try_recv() {
                let msg: Message = Message::UpdatePosition { position };
                // println!("Sending position to server: ({}, {}, {})", position.x, position.y, position.z);
                let serialized = bincode::serialize(&msg).unwrap();
                client.send_message(DefaultChannel::ReliableOrdered, serialized);
            }

            // Receive messages from server and send to main thread
            while let Some(message_bytes) = client.receive_message(DefaultChannel::ReliableOrdered) {
                if let Ok(msg) = bincode::deserialize::<Message>(&message_bytes) {
                    if let Message::UpdatePosition { position } = msg {
                        println!("Received position from server: ({}, {}, {})", position.x, position.y, position.z);
                        let _ = tx.send(position); // Send remote player position to main thread
                    }
                }
            }
        }
        transport.send_packets(&mut client).unwrap();
        thread::sleep(Duration::from_millis(16));
    }
}