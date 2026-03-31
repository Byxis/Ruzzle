use std::{
    collections::HashMap,
    net::{SocketAddr, UdpSocket},
    thread,
    time::{Duration, Instant, SystemTime},
};

use renet::{ClientId, ConnectionConfig, DefaultChannel, RenetServer, ServerEvent};
use renet_netcode::{
    NETCODE_USER_DATA_BYTES, NetcodeServerTransport, ServerAuthentication, ServerConfig,
};

use crate::room::{RoomManager, RoomMessage, treat_message};

// Helper struct to pass an username in the user data
struct Username(String);

impl Username {
    fn from_user_data(user_data: &[u8; NETCODE_USER_DATA_BYTES]) -> Self {
        let mut buffer = [0u8; 8];
        buffer.copy_from_slice(&user_data[0..8]);
        let mut len = u64::from_le_bytes(buffer) as usize;
        len = len.min(NETCODE_USER_DATA_BYTES - 8);
        let data = user_data[8..len + 8].to_vec();
        let username = String::from_utf8(data).unwrap();
        Self(username)
    }
}

const PROTOCOL_ID: u64 = 7;

fn manage_event(
    event: ServerEvent,
    server: &mut RenetServer,
    transport: &NetcodeServerTransport,
    usernames: &mut HashMap<ClientId, String>,
) {
    match event {
        ServerEvent::ClientConnected { client_id } => {
            let user_data = transport.user_data(client_id).unwrap();
            let username = Username::from_user_data(&user_data);
            server.broadcast_message_except(
                client_id,
                DefaultChannel::ReliableOrdered,
                format!("User \"{}\" connected", username.0),
            );
            usernames.insert(client_id, username.0);
            println!("Client {} connected.", client_id)
        }
        ServerEvent::ClientDisconnected { client_id, reason } => {
            println!("Client {} disconnected: {}", client_id, reason);
            if let Some(username) = usernames.remove(&client_id) {
                server.broadcast_message_except(
                    client_id,
                    DefaultChannel::ReliableOrdered,
                    format!("User \"{}\" disconnected", username),
                );
            }
        }
    }
}

pub fn server(public_addr: SocketAddr) {
    let connection_config = ConnectionConfig::default();
    let mut server: RenetServer = RenetServer::new(connection_config);

    let current_time = SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .unwrap();
    let server_config = ServerConfig {
        current_time,
        max_clients: 64,
        protocol_id: PROTOCOL_ID,
        public_addresses: vec![public_addr],
        authentication: ServerAuthentication::Unsecure,
    };
    let socket: UdpSocket = UdpSocket::bind(public_addr).unwrap();

    let mut transport = NetcodeServerTransport::new(server_config, socket).unwrap();

    let mut usernames: HashMap<ClientId, String> = HashMap::new();
    // let mut received_messages = vec![];
    let mut last_updated = Instant::now();

    let mut room_manager = RoomManager::new();

    loop {
        let now = Instant::now();
        let duration = now - last_updated;
        last_updated = now;

        server.update(duration);
        transport.update(duration, &mut server).unwrap();

        while let Some(event) = server.get_event() {
            manage_event(event, &mut server, &transport, &mut usernames);
        }

        for client_id in server.clients_id() {
            while let Some(message) =
                server.receive_message(client_id, DefaultChannel::ReliableOrdered)
            {
                let mes: RoomMessage = bincode::deserialize(&message).unwrap();
                let response: RoomMessage = treat_message(mes, &mut room_manager);

                let players: Option<Vec<ClientId>> = room_manager.get_room_clients(client_id);
                for player_id in players.unwrap_or_else(Vec::new) {
                    server.send_message(
                        player_id,
                        DefaultChannel::ReliableOrdered,
                        bincode::serialize(&response).unwrap(),
                    );
                }
            }
        }

        transport.send_packets(&mut server);
        thread::sleep(Duration::from_millis(50));
    }
}
