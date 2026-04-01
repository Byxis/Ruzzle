use std::{
    net::{SocketAddr, UdpSocket},
    thread,
    time::{Duration, Instant, SystemTime},
};

use renet::{ClientId, ConnectionConfig, DefaultChannel, RenetServer};
use renet_netcode::{NetcodeServerTransport, ServerAuthentication, ServerConfig};

use crate::room::{RoomManager, RoomMessage, treat_message};

const PROTOCOL_ID: u64 = 7;

/// Main function that control all the server logic.
/// Contains the initialisation of the socket, main loop of execution,
/// the treatment of the messages received and sended.
///
/// Based on the Renet example, that I adapted to Ruzzle logic.
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

    let mut last_updated = Instant::now();

    let mut room_manager = RoomManager::new();

    loop {
        let now = Instant::now();
        let duration = now - last_updated;
        last_updated = now;

        server.update(duration);
        transport.update(duration, &mut server).unwrap();

        for client_id in server.clients_id() {
            while let Some(message) =
                server.receive_message(client_id, DefaultChannel::ReliableOrdered)
            {
                let mes: RoomMessage = bincode::deserialize(&message).unwrap();
                println!("Received message from client {}: {:?}", client_id, mes);
                let response: RoomMessage = treat_message(mes, client_id, &mut room_manager);

                let players: Option<Vec<ClientId>> = room_manager.get_room_clients(client_id);
                println!(
                    "Broadcasting message to clients in the same room as {}: {:?}",
                    client_id, players
                );

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
        thread::sleep(Duration::from_millis(16));
    }
}
