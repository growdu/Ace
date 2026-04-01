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
            avg_score: user.score as f64,
        };
        Ok(Json(stats))
    } else {
        Err((StatusCode::NOT_FOUND, "用户不存在".to_string()))
    }
}