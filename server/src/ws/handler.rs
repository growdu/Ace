use axum::{
    extract::{State, Path, ws::{WebSocket, WebSocketUpgrade, Message}},
    response::Response,
};
use futures_util::{SinkExt, StreamExt};
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

    let (tx, _rx) = broadcast::channel::<String>(100);

    let state_clone = state.clone();
    let room_id_clone = room_id.clone();
    let tx_clone = tx.clone();

    tokio::spawn(async move {
        while let Some(msg) = receiver.next().await {
            match msg {
                Ok(Message::Text(text)) => {
                    if let Err(e) = handle_game_message(&state_clone, &room_id_clone, &text, tx_clone.clone()).await {
                        log::error!("处理游戏消息错误: {}", e);
                    }
                }
                Ok(Message::Close(_)) => break,
                Err(e) => {
                    log::error!("WebSocket error: {}", e);
                    break;
                }
                _ => {}
            }
        }
    });

    let mut rx = tx.subscribe();
    tokio::spawn(async move {
        while let Ok(msg) = rx.recv().await {
            if sender.send(Message::Text(msg)).await.is_err() {
                break;
            }
        }
    });
}

async fn handle_game_message(
    _state: &SharedState,
    _room_id: &str,
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