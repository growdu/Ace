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