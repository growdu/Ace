use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct GameMessage {
    pub msg_type: String,
    pub data: serde_json::Value,
}

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