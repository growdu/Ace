use std::sync::Arc;
use parking_lot::RwLock;
use std::collections::HashMap;
use crate::game::GameRoom;

pub struct AppState {
    pub users: RwLock<HashMap<String, User>>,
    pub rooms: RwLock<HashMap<String, Room>>,
    pub sessions: RwLock<HashMap<String, String>>,
    pub game_rooms: RwLock<HashMap<String, GameRoom>>,
}

impl AppState {
    pub fn new() -> Self {
        AppState {
            users: RwLock::new(HashMap::new()),
            rooms: RwLock::new(HashMap::new()),
            sessions: RwLock::new(HashMap::new()),
            game_rooms: RwLock::new(HashMap::new()),
        }
    }
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct User {
    pub id: String,
    pub username: String,
    pub password_hash: String,
    pub score: i32,
    pub wins: i32,
    pub losses: i32,
    pub created_at: i64,
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct Room {
    pub id: String,
    pub owner_id: String,
    pub players: Vec<RoomPlayer>,
    pub status: RoomStatus,
    pub created_at: i64,
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct RoomPlayer {
    pub user_id: String,
    pub username: String,
    pub is_ready: bool,
    pub is_robot: bool,
}

#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub enum RoomStatus {
    Waiting,
    Full,
    Playing,
    Closed,
}

pub type SharedState = Arc<AppState>;

pub fn create_state() -> SharedState {
    Arc::new(AppState::new())
}