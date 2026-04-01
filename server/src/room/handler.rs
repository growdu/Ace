use axum::{
    extract::{State, Path},
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use serde::Deserialize;
use crate::state::{SharedState, Room, RoomPlayer, RoomStatus};
use super::service::{CreateRoomRequest, JoinRoomRequest, RoomResponse};

pub async fn create_room(
    State(state): State<SharedState>,
    Json(payload): Json<CreateRoomRequest>,
) -> Result<impl IntoResponse, (StatusCode, String)> {
    let room_id = uuid::Uuid::new_v4().to_string();

    let room = Room {
        id: room_id.clone(),
        owner_id: payload.user_id.clone(),
        players: vec![RoomPlayer {
            user_id: payload.user_id.clone(),
            username: payload.username,
            is_ready: true,
            is_robot: false,
        }],
        status: RoomStatus::Waiting,
        created_at: chrono::Utc::now().timestamp(),
    };

    state.rooms.write().insert(room_id.clone(), room);

    Ok(Json(serde_json::json!({
        "room_id": room_id,
        "message": "房间创建成功"
    })))
}

pub async fn join_room(
    State(state): State<SharedState>,
    Json(payload): Json<JoinRoomRequest>,
) -> Result<impl IntoResponse, (StatusCode, String)> {
    let mut rooms = state.rooms.write();

    let room = rooms.get_mut(&payload.room_id)
        .ok_or_else(|| (StatusCode::NOT_FOUND, "房间不存在".to_string()))?;

    if room.players.len() >= 4 {
        return Err((StatusCode::BAD_REQUEST, "房间已满".to_string()));
    }

    if room.status != RoomStatus::Waiting {
        return Err((StatusCode::BAD_REQUEST, "房间已开始游戏".to_string()));
    }

    for player in &room.players {
        if player.user_id == payload.user_id {
            return Err((StatusCode::CONFLICT, "已在房间中".to_string()));
        }
    }

    room.players.push(RoomPlayer {
        user_id: payload.user_id.clone(),
        username: payload.username,
        is_ready: false,
        is_robot: false,
    });

    if room.players.len() == 4 {
        room.status = RoomStatus::Full;
    }

    Ok(Json(RoomResponse::from(room.clone())))
}

pub async fn leave_room(
    State(state): State<SharedState>,
    Json(payload): Json<serde_json::Value>,
) -> Result<impl IntoResponse, (StatusCode, String)> {
    let room_id = payload["room_id"].as_str().unwrap_or("");
    let user_id = payload["user_id"].as_str().unwrap_or("");

    let mut rooms = state.rooms.write();

    let room = rooms.get_mut(room_id)
        .ok_or_else(|| (StatusCode::NOT_FOUND, "房间不存在".to_string()))?;

    let pos = room.players.iter().position(|p| p.user_id == user_id);

    if let Some(pos) = pos {
        room.players.remove(pos);

        if room.owner_id == user_id && !room.players.is_empty() {
            room.owner_id = room.players[0].user_id.clone();
        }

        if room.players.is_empty() {
            rooms.remove(room_id);
        }

        Ok(Json(serde_json::json!({
            "success": true,
            "message": "离开房间成功"
        })))
    } else {
        Err((StatusCode::NOT_FOUND, "不在房间中".to_string()))
    }
}

pub async fn get_room(
    State(state): State<SharedState>,
    Path(room_id): Path<String>,
) -> Result<impl IntoResponse, (StatusCode, String)> {
    let rooms = state.rooms.read();

    let room = rooms.get(&room_id)
        .ok_or_else(|| (StatusCode::NOT_FOUND, "房间不存在".to_string()))?;

    Ok(Json(RoomResponse::from(room.clone())))
}

pub async fn start_game(
    State(state): State<SharedState>,
    Path(room_id): Path<String>,
) -> Result<impl IntoResponse, (StatusCode, String)> {
    let mut rooms = state.rooms.write();

    let room = rooms.get_mut(&room_id)
        .ok_or_else(|| (StatusCode::NOT_FOUND, "房间不存在".to_string()))?;

    if room.players.len() != 4 {
        return Err((StatusCode::BAD_REQUEST, "需要4人才能开始".to_string()));
    }

    room.status = RoomStatus::Playing;

    Ok(Json(serde_json::json!({
        "success": true,
        "message": "游戏开始"
    })))
}

#[derive(Debug, Deserialize)]
pub struct MatchRequest {
    pub user_id: String,
    pub username: String,
    pub mode: String,
}

pub async fn start_match(
    State(state): State<SharedState>,
    Json(payload): Json<MatchRequest>,
) -> Result<impl IntoResponse, (StatusCode, String)> {
    let room_id = uuid::Uuid::new_v4().to_string();

    let mut players = vec![RoomPlayer {
        user_id: payload.user_id.clone(),
        username: payload.username.clone(),
        is_ready: true,
        is_robot: false,
    }];

    let bot_count = match payload.mode.as_str() {
        "single" => 3,
        "double" => 2,
        "bot" => 3,
        _ => 3,
    };

    for i in 0..bot_count {
        players.push(RoomPlayer {
            user_id: format!("bot_{}", i),
            username: format!("机器人{}", i + 1),
            is_ready: true,
            is_robot: true,
        });
    }

    let room = Room {
        id: room_id.clone(),
        owner_id: payload.user_id.clone(),
        players,
        status: RoomStatus::Playing,
        created_at: chrono::Utc::now().timestamp(),
    };

    state.rooms.write().insert(room_id.clone(), room);

    Ok(Json(serde_json::json!({
        "room_id": room_id,
        "status": "matched"
    })))
}

pub async fn cancel_match(
    Json(_payload): Json<serde_json::Value>,
) -> Result<impl IntoResponse, (StatusCode, String)> {
    Ok(Json(serde_json::json!({
        "success": true,
        "message": "匹配取消"
    })))
}