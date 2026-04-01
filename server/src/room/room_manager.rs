use crate::room::player::{Player, Position};
use crate::room::room::Room;
use renet::ClientId;
use std::collections::HashMap;

pub struct RoomManager {
    rooms: HashMap<u64, Room>,
    client_to_room: HashMap<ClientId, u64>, // Track which room each client is in
    next_room_id: u64,
}

impl RoomManager {
    pub fn new() -> Self {
        Self {
            rooms: HashMap::new(),
            client_to_room: HashMap::new(),
            next_room_id: 1,
        }
    }

    /// Create a new room and return its ID
    pub fn create_room(&mut self) -> u64 {
        let room_id = self.next_room_id;
        self.next_room_id += 1;
        self.rooms.insert(room_id, Room::new(room_id));
        room_id
    }

    /// Add a player to a room
    pub fn add_player_to_room(
        &mut self,
        client_id: ClientId,
        player: Player,
        room_id: u64,
    ) -> Result<(), String> {
        let room = self
            .rooms
            .get_mut(&room_id)
            .ok_or("Room not found".to_string())?;

        room.add_player(player)?;
        self.client_to_room.insert(client_id, room_id);
        Ok(())
    }

    /// Remove player from room and clean up empty rooms
    pub fn remove_player(&mut self, client_id: ClientId) -> Result<u64, String> {
        let room_id = self
            .client_to_room
            .remove(&client_id)
            .ok_or("Client not in any room".to_string())?;

        let room = self
            .rooms
            .get_mut(&room_id)
            .ok_or("Room not found".to_string())?;

        room.remove_player(client_id as i32)?;

        // Clean up empty rooms
        if room.is_empty() {
            self.rooms.remove(&room_id);
        }

        Ok(room_id)
    }

    /// Get all clients in the same room
    pub fn get_room_clients(&self, client_id: ClientId) -> Option<Vec<ClientId>> {
        let room_id = self.client_to_room.get(&client_id)?;
        println!("Client {} is in room {}", client_id, room_id);
        Some(
            self.client_to_room
                .iter()
                .filter(|(cid, rid)| *rid == room_id && *cid != &client_id)
                .map(|(cid, _)| *cid)
                .collect(),
        )
    }

    /// Get room by ID
    pub fn get_room(&self, room_id: u64) -> Option<&Room> {
        self.rooms.get(&room_id)
    }

    /// Get mutable room by ID
    pub fn get_room_mut(&mut self, room_id: u64) -> Option<&mut Room> {
        self.rooms.get_mut(&room_id)
    }

    /// Get room ID for a client
    pub fn get_client_room(&self, client_id: ClientId) -> Option<u64> {
        self.client_to_room.get(&client_id).copied()
    }

    pub fn update_player_position(
        &mut self,
        client_id: ClientId,
        new_position: Position,
    ) -> Result<(), String> {
        let room_id = self
            .get_client_room(client_id)
            .ok_or("Client not in any room".to_string())?;

        let room = self
            .get_room_mut(room_id)
            .ok_or("Room not found".to_string())?;

        if let Some(player) = &mut room.player1 {
            if player.id == client_id as u64 {
                player.update_position(new_position);
                return Ok(());
            }
        }
        if let Some(player) = &mut room.player2 {
            if player.id == client_id as u64 {
                player.update_position(new_position);
                return Ok(());
            }
        }

        Err("Player not found in room".to_string())
    }
}
