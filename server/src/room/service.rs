use serde::{Deserialize, Serialize};
use super::super::state::{Room, RoomPlayer, RoomStatus};

#[derive(Debug, Deserialize)]
pub struct CreateRoomRequest {
    pub user_id: String,
    pub username: String,
}

#[derive(Debug, Deserialize)]
pub struct JoinRoomRequest {
    pub room_id: String,
    pub user_id: String,
    pub username: String,
}

#[derive(Debug, Serialize)]
pub struct RoomResponse {
    pub id: String,
    pub owner_id: String,
    pub players: Vec<RoomPlayer>,
    pub status: String,
    pub created_at: i64,
}

impl From<Room> for RoomResponse {
    fn from(room: Room) -> Self {
        RoomResponse {
            id: room.id,
            owner_id: room.owner_id,
            players: room.players,
            status: format!("{:?}", room.status),
            created_at: room.created_at,
        }
    }
}