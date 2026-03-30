use crate::room::player::Player;

pub enum RoomStatus {
    WaitingRoom,
    InGame(i32), // the number represents the level
    Loading
}

pub struct Room {
    pub id: u64,
    pub player1: Option<Player>,
    pub player2: Option<Player>,
    pub status: RoomStatus,
}

impl Room {
    pub fn new(id: u64) -> Self {
        Self {
            id,
            player1: None,
            player2: None,
            status: RoomStatus::WaitingRoom,
        }
    }

    pub fn add_player(&mut self, player: Player) -> Result<(), String> {
        if self.player1.is_none() {
            self.player1 = Some(player);
            Ok(())
        } else if self.player2.is_none() {
            self.player2 = Some(player);
            Ok(())
        } else {
            Err("Room is full".to_string())
        }
    }

    pub fn remove_player(&mut self, player: i32) -> Result<(), String> {
        if let Some(p) = &self.player1 {
            if p.id == player as u64 {
                self.player1 = None;
                return Ok(());
            }
        }
        if let Some(p) = &self.player2 {
            if p.id == player as u64 {
                self.player2 = None;
                return Ok(());
            }
        }
        Err("Player not found in room".to_string())
    }

    pub fn is_full(&self) -> bool {
        self.player1.is_some() && self.player2.is_some()
    }

    pub fn is_empty(&self) -> bool {
        self.player1.is_none() && self.player2.is_none()
    }

    pub fn player_count(&self) -> usize {
        let mut count = 0;
        if self.player1.is_some() { count += 1; }
        if self.player2.is_some() { count += 1; }
        count
    }

    pub fn load_new_level(&mut self, level_id: i32) {
        match self.status {
            | RoomStatus::WaitingRoom => {
                self.status = RoomStatus::InGame(level_id)
            },
            | RoomStatus::InGame(_) => {
                self.status = RoomStatus::InGame(level_id)
            },
            | RoomStatus::Loading => {
                self.status = RoomStatus::InGame(level_id)
            }
        }
    }
}