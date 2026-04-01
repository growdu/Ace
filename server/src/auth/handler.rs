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