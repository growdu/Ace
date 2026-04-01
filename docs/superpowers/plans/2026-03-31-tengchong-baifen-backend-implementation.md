# 腾冲百分游戏 - 后端系统实现计划

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** 实现腾冲百分游戏后端服务系统，支持用户注册登录、房间管理、在线对战

**Architecture:** 使用 Rust + Axum + Tokio 构建后端服务，JWT 认证，WebSocket 实时通信，复用客户端游戏引擎

**Tech Stack:** Rust, Axum, Tokio, tokio-tungstenite, serde, jsonwebtoken, bcrypt

---

## 项目结构

```
server/
├── src/
│   ├── main.rs           # 入口
│   ├── lib.rs            # 库导出
│   ├── state.rs          # 应用状态
│   ├── auth/             # 认证模块
│   │   ├── mod.rs
│   │   ├── handler.rs
│   │   └── service.rs
│   ├── user/             # 用户模块
│   │   ├── mod.rs
│   │   ├── handler.rs
│   │   └── service.rs
│   ├── room/             # 房间模块
│   │   ├── mod.rs
│   │   ├── handler.rs
│   │   └── service.rs
│   ├── game/             # 游戏模块
│   │   ├── mod.rs
│   │   └── handler.rs
│   └── ws/               # WebSocket
│       ├── mod.rs
│       └── handler.rs
├── Cargo.toml
└── data/                 # 数据存储
    └── users.json
```

---

## Task 1: 初始化后端项目

**Files:**
- Create: `server/Cargo.toml`
- Create: `server/src/main.rs`
- Create: `server/src/lib.rs`
- Create: `server/src/state.rs`

- [ ] **Step 1: 创建 server 目录**

```bash
mkdir -p server/src/auth server/src/user server/src/room server/src/game server/src/ws server/data
```

- [ ] **Step 2: 创建 Cargo.toml**

```toml
[package]
name = "ace-server"
version = "1.0.0"
edition = "2021"

[dependencies]
axum = "0.7"
tokio = { version = "1", features = ["full"] }
tower = "0.4"
tower-http = { version = "0.5", features = ["cors"] }
serde = { version = "1", features = ["derive"] }
serde_json = "1"
tokio-tungstenite = "0.21"
futures-util = "0.3"
uuid = { version = "1", features = ["v4"] }
jsonwebtoken = "9"
bcrypt = "0.15"
chrono = "0.4"
log = "0.4"
env_logger = "0.10"
parking_lot = "0.12"

[dev-dependencies]
tokio-test = "0.4"
```

- [ ] **Step 3: 创建 src/state.rs**

```rust
use std::sync::Arc;
use parking_lot::RwLock;
use std::collections::HashMap;

pub struct AppState {
    pub users: RwLock<HashMap<String, User>>,
    pub rooms: RwLock<HashMap<String, Room>>,
    pub sessions: RwLock<HashMap<String, String>>, // token -> user_id
}

impl AppState {
    pub fn new() -> Self {
        AppState {
            users: RwLock::new(HashMap::new()),
            rooms: RwLock::new(HashMap::new()),
            sessions: RwLock::new(HashMap::new()),
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
```

- [ ] **Step 4: 创建 src/lib.rs**

```rust
pub mod state;

pub use state::*;
```

- [ ] **Step 5: 创建 src/main.rs**

```rust
use axum::{
    routing::{get, post},
    Router,
    extract::State,
};
use tower_http::cors::{CorsLayer, Any};
use std::sync::Arc;

mod auth;
mod user;
mod room;
mod game;
mod ws;

use state::SharedState;

#[tokio::main]
async fn main() {
    env_logger::init();

    let state = state::create_state();

    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(Any)
        .allow_headers(Any);

    let app = Router::new()
        .route("/health", get(health_check))
        .route("/api/auth/register", post(auth::register))
        .route("/api/auth/login", post(auth::login))
        .route("/api/auth/verify", post(auth::verify))
        .route("/api/user/profile", get(user::get_profile))
        .route("/api/user/profile", put(user::update_profile))
        .route("/api/user/stats", get(user::get_stats))
        .route("/api/room/create", post(room::create_room))
        .route("/api/room/join", post(room::join_room))
        .route("/api/room/leave", post(room::leave_room))
        .route("/api/room/:id", get(room::get_room))
        .route("/api/room/:id/start", post(room::start_game))
        .route("/api/match", post(room::start_match))
        .route("/api/match/cancel", post(room::cancel_match))
        .route("/ws/game/:room_id", get(ws::game_ws))
        .layer(cors)
        .with_state(state);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:8080").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn health_check() -> &'static str {
    "OK"
}
```

- [ ] **Step 6: 测试编译**

```bash
cd server && cargo check
```

Expected: 编译成功（会有未定义模块错误，后续修复）

- [ ] **Step 7: 提交代码**

```bash
git add server/
git commit -m "feat: initialize backend server project"
```

---

## Task 2: 实现认证模块

**Files:**
- Create: `server/src/auth/mod.rs`
- Create: `server/src/auth/service.rs`
- Modify: `server/src/main.rs`

- [ ] **Step 1: 创建 auth/service.rs**

```rust
use serde::{Deserialize, Serialize};
use jsonwebtoken::{encode, decode, Header, Validation, EncodingKey, DecodingKey};
use bcrypt::{hash, verify, DEFAULT_COST};
use chrono::{Utc, Duration};

const SECRET: &[u8] = b"ace_tengchong_baifen_secret_key_2024";

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String, // user_id
    pub username: String,
    pub exp: i64,
}

#[derive(Debug, Deserialize)]
pub struct RegisterRequest {
    pub username: String,
    pub password: String,
}

#[derive(Debug, Deserialize)]
pub struct LoginRequest {
    pub username: String,
    pub password: String,
}

#[derive(Debug, Serialize)]
pub struct AuthResponse {
    pub token: String,
    pub user: UserInfo,
}

#[derive(Debug, Serialize)]
pub struct UserInfo {
    pub id: String,
    pub username: String,
    pub score: i32,
    pub wins: i32,
    pub losses: i32,
}

pub fn hash_password(password: &str) -> String {
    hash(password, DEFAULT_COST).unwrap()
}

pub fn verify_password(password: &str, hash: &str) -> bool {
    verify(password, hash).unwrap_or(false)
}

pub fn create_token(user_id: &str, username: &str) -> String {
    let exp = Utc::now() + Duration::hours(24);
    let claims = Claims {
        sub: user_id.to_string(),
        username: username.to_string(),
        exp: exp.timestamp(),
    };
    encode(&Header::default(), &claims, &EncodingKey::from_secret(SECRET)).unwrap()
}

pub fn verify_token(token: &str) -> Option<Claims> {
    let validation = Validation::default();
    decode::<Claims>(&token, &DecodingKey::from_secret(SECRET), &validation)
        .ok()
        .map(|d| d.claims)
}
```

- [ ] **Step 2: 创建 auth/mod.rs**

```rust
pub mod service;

pub use service::*;
```

- [ ] **Step 3: 创建 auth/handler.rs**

```rust
use axum::{
    extract::{State, Json},
    http::StatusCode,
    response::IntoResponse,
};
use crate::state::{SharedState, User};
use super::service::{RegisterRequest, LoginRequest, AuthResponse, UserInfo, hash_password, verify_password, create_token, verify_token};

pub async fn register(
    State(state): State<SharedState>,
    Json(payload): Json<RegisterRequest>,
) -> Result<impl IntoResponse, (StatusCode, String)> {
    if payload.username.is_empty() || payload.password.len() < 6 {
        return Err((StatusCode::BAD_REQUEST, "用户名或密码格式错误".to_string()));
    }

    let mut users = state.users.write();

    // 检查用户名是否已存在
    for user in users.values() {
        if user.username == payload.username {
            return Err((StatusCode::CONFLICT, "用户名已存在".to_string()));
        }
    }

    let user_id = uuid::Uuid::new_v4().to_string();
    let password_hash = hash_password(&payload.password);

    let user = User {
        id: user_id.clone(),
        username: payload.username.clone(),
        password_hash,
        score: 1000,
        wins: 0,
        losses: 0,
        created_at: chrono::Utc::now().timestamp(),
    };

    let token = create_token(&user_id, &payload.username);

    users.insert(user_id.clone(), user);

    let response = AuthResponse {
        token,
        user: UserInfo {
            id: user_id,
            username: payload.username,
            score: 1000,
            wins: 0,
            losses: 0,
        },
    };

    Ok(Json(response))
}

pub async fn login(
    State(state): State<SharedState>,
    Json(payload): Json<LoginRequest>,
) -> Result<impl IntoResponse, (StatusCode, String)> {
    let users = state.users.read();

    let user = users.values().find(|u| u.username == payload.username);

    if let Some(user) = user {
        if verify_password(&payload.password, &user.password_hash) {
            let token = create_token(&user.id, &user.username);

            // 记录 session
            state.sessions.write().insert(token.clone(), user.id.clone());

            let response = AuthResponse {
                token,
                user: UserInfo {
                    id: user.id.clone(),
                    username: user.username.clone(),
                    score: user.score,
                    wins: user.wins,
                    losses: user.losses,
                },
            };

            return Ok(Json(response));
        }
    }

    Err((StatusCode::UNAUTHORIZED, "用户名或密码错误".to_string()))
}

pub async fn verify(
    Json(payload): Json<serde_json::Value>,
) -> Result<impl IntoResponse, (StatusCode, String)> {
    let token = payload["token"].as_str().unwrap_or("");

    if let Some(claims) = verify_token(token) {
        Ok(Json(serde_json::json!({
            "valid": true,
            "user_id": claims.sub,
            "username": claims.username
        })))
    } else {
        Ok(Json(serde_json::json!({
            "valid": false
        })))
    }
}
```

- [ ] **Step 4: 更新 auth/mod.rs 导出 handler**

```rust
pub mod service;
pub mod handler;

pub use handler::*;
```

- [ ] **Step 5: 测试编译**

```bash
cd server && cargo check
```

- [ ] **Step 6: 提交代码**

```bash
git add server/src/auth/
git commit -m "feat: implement authentication module"
```

---

## Task 3: 实现用户模块

**Files:**
- Create: `server/src/user/mod.rs`
- Create: `server/src/user/service.rs`
- Create: `server/src/user/handler.rs`

- [ ] **Step 1: 创建 user/service.rs**

```rust
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize)]
pub struct UserProfile {
    pub id: String,
    pub username: String,
    pub score: i32,
    pub wins: i32,
    pub losses: i32,
    pub created_at: i64,
}

#[derive(Debug, Serialize)]
pub struct UserStats {
    pub total_games: i32,
    pub win_rate: f64,
    pub avg_score: f64,
}
```

- [ ] **Step 2: 创建 user/mod.rs**

```rust
pub mod service;
pub mod handler;

pub use service::*;
pub use handler::*;
```

- [ ] **Step 3: 创建 user/handler.rs**

```rust
use axum::{
    extract::{State, Query},
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use serde::Deserialize;
use crate::state::SharedState;
use super::service::{UserProfile, UserStats};

#[derive(Deserialize)]
pub struct UserQuery {
    user_id: Option<String>,
}

pub async fn get_profile(
    State(state): State<SharedState>,
    Query(query): Query<UserQuery>,
) -> Result<impl IntoResponse, (StatusCode, String)> {
    // 从 header 获取 user_id（实际应从 token 解析，这里简化）
    let user_id = query.user_id.unwrap_or_default();

    let users = state.users.read();

    if let Some(user) = users.get(&user_id) {
        let profile = UserProfile {
            id: user.id.clone(),
            username: user.username.clone(),
            score: user.score,
            wins: user.wins,
            losses: user.losses,
            created_at: user.created_at,
        };
        Ok(Json(profile))
    } else {
        Err((StatusCode::NOT_FOUND, "用户不存在".to_string()))
    }
}

pub async fn update_profile(
    State(state): State<SharedState>,
    Json(payload): Json<serde_json::Value>,
) -> Result<impl IntoResponse, (StatusCode, String)> {
    let user_id = payload["user_id"].as_str().unwrap_or("");
    let new_username = payload["username"].as_str().unwrap_or("");

    let mut users = state.users.write();

    if let Some(user) = users.get_mut(user_id) {
        if !new_username.is_empty() {
            user.username = new_username.to_string();
        }
        Ok(Json(serde_json::json!({
            "success": true,
            "user": user
        })))
    } else {
        Err((StatusCode::NOT_FOUND, "用户不存在".to_string()))
    }
}

pub async fn get_stats(
    State(state): State<SharedState>,
    Query(query): Query<UserQuery>,
) -> Result<impl IntoResponse, (StatusCode, String)> {
    let user_id = query.user_id.unwrap_or_default();

    let users = state.users.read();

    if let Some(user) = users.get(&user_id) {
        let total_games = user.wins + user.losses;
        let win_rate = if total_games > 0 {
            (user.wins as f64 / total_games as f64) * 100.0
        } else {
            0.0
        };

        let stats = UserStats {
            total_games,
            win_rate,
            avg_score: user.score as f64, // 简化
        };
        Ok(Json(stats))
    } else {
        Err((StatusCode::NOT_FOUND, "用户不存在".to_string()))
    }
}
```

- [ ] **Step 4: 测试编译**

```bash
cd server && cargo check
```

- [ ] **Step 5: 提交代码**

```bash
git add server/src/user/
git commit -m "feat: implement user module"
```

---

## Task 4: 实现房间模块

**Files:**
- Create: `server/src/room/mod.rs`
- Create: `server/src/room/service.rs`
- Create: `server/src/room/handler.rs`

- [ ] **Step 1: 创建 room/service.rs**

```rust
use serde::{Deserialize, Serialize};
use crate::state::{Room, RoomPlayer, RoomStatus};

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
```

- [ ] **Step 2: 创建 room/mod.rs**

```rust
pub mod service;
pub mod handler;

pub use service::*;
pub use handler::*;
```

- [ ] **Step 3: 创建 room/handler.rs**

```rust
use axum::{
    extract::{State, Path},
    http::StatusCode,
    response::IntoResponse,
    Json,
};
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
            user_id: payload.user_id,
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

    // 检查是否已加入
    for player in &room.players {
        if player.user_id == payload.user_id {
            return Err((StatusCode::CONFLICT, "已在房间中".to_string()));
        }
    }

    room.players.push(RoomPlayer {
        user_id: payload.user_id,
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

        // 如果房主离开，转移给下一个玩家
        if room.owner_id == user_id && !room.players.is_empty() {
            room.owner_id = room.players[0].user_id.clone();
        }

        // 房间空了则删除
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

// 匹配系统
#[derive(Debug, Deserialize)]
pub struct MatchRequest {
    pub user_id: String,
    pub username: String,
    pub mode: String, // "single", "double", "bot"
}

pub async fn start_match(
    State(state): State<SharedState>,
    Json(payload): Json<MatchRequest>,
) -> Result<impl IntoResponse, (StatusCode, String)> {
    // 简化：创建房间并自动匹配机器人
    let room_id = uuid::Uuid::new_v4().to_string();

    let mut players = vec![RoomPlayer {
        user_id: payload.user_id.clone(),
        username: payload.username.clone(),
        is_ready: true,
        is_robot: false,
    }];

    // 根据模式添加机器人
    let bot_count = match payload.mode.as_str() {
        "single" => 3,      // 1玩家 + 3机器人
        "double" => 2,     // 1玩家 + 1机器人 + 2玩家
        "bot" => 3,        // 纯人机
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
    Json(payload): Json<serde_json::Value>,
) -> Result<impl IntoResponse, (StatusCode, String)> {
    // 简化实现
    Ok(Json(serde_json::json!({
        "success": true,
        "message": "匹配取消"
    })))
}
```

- [ ] **Step 4: 测试编译**

```bash
cd server && cargo check
```

- [ ] **Step 5: 提交代码**

```bash
git add server/src/room/
git commit -m "feat: implement room and match module"
```

---

## Task 5: 实现 WebSocket 游戏模块

**Files:**
- Create: `server/src/ws/handler.rs`
- Modify: `server/src/ws/mod.rs`
- Create: `server/src/game/handler.rs`
- Modify: `server/src/game/mod.rs`

- [ ] **Step 1: 创建 ws/handler.rs**

```rust
use axum::{
    extract::{State, Path, ws::{WebSocket, WebSocketUpgrade}},
    response::Response,
};
use futures_util::{SinkExt, StreamExt};
use std::sync::Arc;
use tokio::sync::broadcast;
use serde_json::json;
use crate::state::SharedState;

pub async fn game_ws(
    ws: WebSocketUpgrade,
    State(state): State<SharedState>,
    Path(room_id): Path<String>,
) -> Response {
    ws.on_upgrade(move |socket| handle_socket(socket, state, room_id))
}

async fn handle_socket(socket: WebSocket, state: SharedState, room_id: String) {
    let (mut sender, mut receiver) = socket.split();

    // 创建广播通道用于游戏消息
    let (tx, _rx) = broadcast::channel::<String>(100);

    // 接收客户端消息
    let state_clone = state.clone();
    let room_id_clone = room_id.clone();
    let tx_clone = tx.clone();

    tokio::spawn(async move {
        while let Some(msg) = receiver.next().await {
            if let Ok(text) = msg.to_str() {
                if let Err(e) = handle_game_message(&state_clone, &room_id_clone, text, tx_clone.clone()).await {
                    log::error!("处理游戏消息错误: {}", e);
                }
            }
        }
    });

    // 发送游戏状态
    let mut rx = tx.subscribe();
    tokio::spawn(async move {
        while let Ok(msg) = rx.recv().await {
            if sender.send(axum::extract::ws::Message::Text(msg)).await.is_err() {
                break;
            }
        }
    });
}

async fn handle_game_message(
    state: &SharedState,
    room_id: &str,
    message: &str,
    tx: broadcast::Sender<String>,
) -> Result<(), String> {
    let msg: serde_json::Value = serde_json::from_str(message)
        .map_err(|e| format!("JSON解析错误: {}", e))?;

    let msg_type = msg["type"].as_str().unwrap_or("");

    match msg_type {
        "play_card" => {
            let user_id = msg["user_id"].as_str().unwrap_or("");
            let card_index = msg["card_index"].as_u64().unwrap_or(0) as usize;

            // 广播出牌消息
            let response = json!({
                "type": "card_played",
                "user_id": user_id,
                "card_index": card_index,
            });
            let _ = tx.send(response.to_string());
        }
        "bid" => {
            let user_id = msg["user_id"].as_str().unwrap_or("");
            let bid = msg["bid"].as_u64().unwrap_or(75) as u32;

            let response = json!({
                "type": "bid_placed",
                "user_id": user_id,
                "bid": bid,
            });
            let _ = tx.send(response.to_string());
        }
        "ready" => {
            let response = json!({
                "type": "player_ready",
                "user_id": msg["user_id"].as_str().unwrap_or(""),
            });
            let _ = tx.send(response.to_string());
        }
        _ => {}
    }

    Ok(())
}
```

- [ ] **Step 2: 更新 ws/mod.rs**

```rust
pub mod handler;

pub use handler::*;
```

- [ ] **Step 3: 创建 game/handler.rs**

```rust
// 游戏逻辑处理 - 复用客户端游戏引擎
// 此模块负责与服务端游戏状态同步

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct GameMessage {
    pub msg_type: String,
    pub data: serde_json::Value,
}

// 前端消息类型
#[derive(Debug, Deserialize)]
#[serde(tag = "type")]
pub enum ClientMessage {
    #[serde(rename = "play_card")]
    PlayCard { user_id: String, card_index: usize },
    #[serde(rename = "bid")]
    Bid { user_id: String, bid: u32 },
    #[serde(rename = "ready")]
    Ready { user_id: String },
}

// 服务端消息类型
#[derive(Debug, Serialize)]
#[serde(tag = "type")]
pub enum ServerMessage {
    #[serde(rename = "room_state")]
    RoomState { room: serde_json::Value },
    #[serde(rename = "game_start")]
    GameStart { players: Vec<serde_json::Value> },
    #[serde(rename = "deal_cards")]
    DealCards { hands: Vec<Vec<serde_json::Value>> },
    #[serde(rename = "your_turn")]
    YourTurn { player_id: String },
    #[serde(rename = "card_played")]
    CardPlayed { player_id: String, card: serde_json::Value },
    #[serde(rename = "round_end")]
    RoundEnd { winner: usize, score: i32 },
    #[serde(rename = "game_end")]
    GameEnd { winner_team: usize, scores: [i32; 2] },
}
```

- [ ] **Step 4: 更新 game/mod.rs**

```rust
pub mod handler;

pub use handler::*;
```

- [ ] **Step 5: 测试编译**

```bash
cd server && cargo check
```

- [ ] **Step 6: 提交代码**

```bash
git add server/src/ws/ server/src/game/
git commit -m "feat: implement WebSocket game module"
```

---

## Task 6: 客户端连接后端

**Files:**
- Modify: `client/src/App.tsx`
- Create: `client/src/services/api.ts`
- Create: `client/src/hooks/useGame.ts`

- [ ] **Step 1: 创建 services/api.ts**

```typescript
const API_BASE = 'http://localhost:8080';

export interface User {
  id: string;
  username: string;
  score: number;
  wins: number;
  losses: number;
}

export interface LoginResponse {
  token: string;
  user: User;
}

export const auth = {
  register: async (username: string, password: string) => {
    const res = await fetch(`${API_BASE}/api/auth/register`, {
      method: 'POST',
      headers: { 'Content-Type': 'application/json' },
      body: JSON.stringify({ username, password }),
    });
    return res.json();
  },

  login: async (username: string, password: string) => {
    const res = await fetch(`${API_BASE}/api/auth/login`, {
      method: 'POST',
      headers: { 'Content-Type': 'application/json' },
      body: JSON.stringify({ username, password }),
    });
    return res.json();
  },
};

export const match = {
  start: async (userId: string, username: string, mode: string) => {
    const res = await fetch(`${API_BASE}/api/match`, {
      method: 'POST',
      headers: { 'Content-Type': 'application/json' },
      body: JSON.stringify({ user_id: userId, username, mode }),
    });
    return res.json();
  },
};
```

- [ ] **Step 2: 创建 hooks/useGame.ts**

```typescript
import { useState, useCallback, useRef } from 'react';

interface GameMessage {
  type: string;
  [key: string]: any;
}

export function useGame() {
  const [connected, setConnected] = useState(false);
  const [gameState, setGameState] = useState<any>(null);
  const wsRef = useRef<WebSocket | null>(null);

  const connect = useCallback((roomId: string) => {
    const ws = new WebSocket(`ws://localhost:8080/ws/game/${roomId}`);

    ws.onopen = () => {
      setConnected(true);
      console.log('WebSocket connected');
    };

    ws.onmessage = (event) => {
      const msg: GameMessage = JSON.parse(event.data);
      handleMessage(msg);
    };

    ws.onclose = () => {
      setConnected(false);
      console.log('WebSocket disconnected');
    };

    wsRef.current = ws;
  }, []);

  const handleMessage = (msg: GameMessage) => {
    switch (msg.type) {
      case 'game_start':
        setGameState({ phase: 'playing', ...msg });
        break;
      case 'deal_cards':
        // 更新手牌
        break;
      case 'your_turn':
        // 轮到自己出牌
        break;
      case 'round_end':
        // 回合结束
        break;
      case 'game_end':
        // 游戏结束
        break;
    }
  };

  const sendMessage = useCallback((msg: object) => {
    if (wsRef.current?.readyState === WebSocket.OPEN) {
      wsRef.current.send(JSON.stringify(msg));
    }
  }, []);

  return { connected, gameState, connect, sendMessage };
}
```

- [ ] **Step 3: 更新 App.tsx 添加登录和匹配**

```typescript
import { useState } from 'react';
import { auth, match } from './services/api';
import { useGame } from './hooks/useGame';
import { GameBoard } from './components/GameBoard';

function App() {
  const [user, setUser] = useState<any>(null);
  const [username, setUsername] = useState('');
  const [password, setPassword] = useState('');
  const [gameStarted, setGameStarted] = useState(false);
  const { connected, gameState, connect, sendMessage } = useGame();

  const handleLogin = async () => {
    try {
      const result = await auth.login(username, password);
      if (result.token) {
        setUser(result.user);
      }
    } catch (e) {
      console.error('登录失败', e);
    }
  };

  const handleStartMatch = async () => {
    if (!user) return;
    try {
      const result = await match.startMatch(user.id, user.username, 'bot');
      if (result.room_id) {
        connect(result.room_id);
        setGameStarted(true);
      }
    } catch (e) {
      console.error('匹配失败', e);
    }
  };

  if (!user) {
    return (
      <div className="login-screen">
        <h1>腾冲百分</h1>
        <input
          placeholder="用户名"
          value={username}
          onChange={e => setUsername(e.target.value)}
        />
        <input
          type="password"
          placeholder="密码"
          value={password}
          onChange={e => setPassword(e.target.value)}
        />
        <button onClick={handleLogin}>登录</button>
      </div>
    );
  }

  if (!gameStarted) {
    return (
      <div className="lobby">
        <h1>欢迎, {user.username}</h1>
        <p>积分: {user.score}</p>
        <button onClick={handleStartMatch}>开始匹配</button>
        <p>状态: {connected ? '已连接' : '未连接'}</p>
      </div>
    );
  }

  return <GameBoard gameState={gameState} onPlayCard={(i) => sendMessage({ type: 'play_card', card_index: i })} onBid={(b) => sendMessage({ type: 'bid', bid: b })} />;
}

export default App;
```

- [ ] **Step 4: 添加登录样式**

```css
.login-screen {
  display: flex;
  flex-direction: column;
  align-items: center;
  justify-content: center;
  height: 100vh;
  gap: 10px;
}

.login-screen input {
  padding: 10px;
  font-size: 16px;
}

.login-screen button {
  padding: 10px 30px;
  font-size: 16px;
  background: #1976d2;
  color: white;
  border: none;
  border-radius: 4px;
  cursor: pointer;
}

.lobby {
  display: flex;
  flex-direction: column;
  align-items: center;
  justify-content: center;
  height: 100vh;
  gap: 20px;
}
```

- [ ] **Step 5: 测试前后端连接**

```bash
# 启动后端
cd server && cargo run

# 启动前端
cd client && npm run dev
```

- [ ] **Step 6: 提交代码**

```bash
git add client/src/services/ client/src/hooks/
git commit -m "feat: connect client to backend"
```

---

## Task 7: 集成测试与修复

- [ ] **Step 1: 整体测试登录流程**

```bash
curl -X POST http://localhost:8080/api/auth/register \
  -H "Content-Type: application/json" \
  -d '{"username":"test","password":"123456"}'
```

- [ ] **Step 2: 测试房间创建**

```bash
curl -X POST http://localhost:8080/api/match \
  -H "Content-Type: application/json" \
  -d '{"user_id":"xxx","username":"test","mode":"bot"}'
```

- [ ] **Step 3: 修复发现的问题**

- [ ] **Step 4: 最终提交**

```bash
git add server/ client/src/
git commit -m "feat: complete backend system integration"
```

---

**计划完成。文件保存在 `docs/superpowers/plans/2026-03-31-tengchong-baifen-backend-implementation.md`**

---

**执行选项：**

**1. Subagent-Driven (recommended)** - 我为每个任务分配一个子代理，进行快速迭代审查

**2. Inline Execution** - 在此会话中执行任务，使用 executing-plans 进行批量审查

请选择执行方式？