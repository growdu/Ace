use axum::{
    extract::{State, Path, ws::{WebSocket, WebSocketUpgrade, Message}},
    response::Response,
};
use futures_util::{SinkExt, StreamExt};
use tokio::sync::broadcast;
use serde_json::json;
use crate::state::SharedState;
use crate::game::{GameRoom, GamePlayer, GamePhase};

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
    let tx_clone = tx.clone();

    // 发送初始游戏状态
    if let Some(game_room) = state.game_rooms.read().get(&room_id) {
        let state_msg = json!({
            "type": "game_state",
            "phase": format!("{:?}", game_room.phase),
            "current_player": game_room.current_player,
            "current_bid": game_room.current_bid,
            "trump_suit": game_room.trump_suit,
            "scores": game_room.scores,
            "round_number": game_room.round_number,
            "players": game_room.players.iter().map(|p| {
                json!({
                    "user_id": p.user_id,
                    "username": p.username,
                    "cards_count": p.cards.len(),
                    "is_robot": p.is_robot,
                })
            }).collect::<Vec<_>>(),
        });
        let _ = tx.send(state_msg.to_string());
    }

    let state_clone = state.clone();
    let room_id_clone = room_id.clone();

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
    state: &SharedState,
    room_id: &str,
    message: &str,
    tx: broadcast::Sender<String>,
) -> Result<(), String> {
    let msg: serde_json::Value = serde_json::from_str(message)
        .map_err(|e| format!("JSON解析错误: {}", e))?;

    let msg_type = msg["type"].as_str().unwrap_or("");
    let user_id = msg["user_id"].as_str().unwrap_or("");

    match msg_type {
        "bid" => {
            let bid = msg["bid"].as_u64().unwrap_or(75) as u32;

            let mut game_rooms = state.game_rooms.write();
            if let Some(game) = game_rooms.get_mut(room_id) {
                // 找到玩家索引
                if let Some(player_idx) = game.players.iter().position(|p| p.user_id == user_id) {
                    if game.bid(player_idx, bid) {
                        // 广播叫分成功
                        let response = json!({
                            "type": "bid_placed",
                            "user_id": user_id,
                            "bid": bid,
                            "current_bid": game.current_bid,
                            "current_player": game.current_player,
                        });
                        let _ = tx.send(response.to_string());

                        // 检查是否完成叫分阶段
                        if game.bids.iter().all(|b| b.is_some()) && game.bidder.is_some() {
                            // 进入打牌阶段
                            let start_response = json!({
                                "type": "game_start",
                                "bidder": game.bidder,
                                "trump_suit": game.trump_suit,
                                "current_player": game.current_player,
                                "players": game.players.iter().map(|p| {
                                    json!({
                                        "user_id": p.user_id,
                                        "username": p.username,
                                        "cards": p.cards.iter().map(|c| {
                                            json!({
                                                "suit": c.suit,
                                                "rank": c.rank,
                                                "joker": c.joker,
                                            })
                                        }).collect::<Vec<_>>(),
                                    })
                                }).collect::<Vec<_>>(),
                            });
                            let _ = tx.send(start_response.to_string());
                        }
                    }
                }
            }
        }
        "pass_bid" => {
            let mut game_rooms = state.game_rooms.write();
            if let Some(game) = game_rooms.get_mut(room_id) {
                if let Some(player_idx) = game.players.iter().position(|p| p.user_id == user_id) {
                    if game.pass_bid(player_idx) {
                        let response = json!({
                            "type": "bid_passed",
                            "user_id": user_id,
                            "current_player": game.current_player,
                            "current_bid": game.current_bid,
                        });
                        let _ = tx.send(response.to_string());

                        // 检查是否完成叫分
                        if game.phase == GamePhase::Playing {
                            let start_response = json!({
                                "type": "game_start",
                                "bidder": game.bidder,
                                "trump_suit": game.trump_suit,
                                "current_player": game.current_player,
                                "players": game.players.iter().map(|p| {
                                    json!({
                                        "user_id": p.user_id,
                                        "username": p.username,
                                        "cards": p.cards.iter().map(|c| {
                                            json!({
                                                "suit": c.suit,
                                                "rank": c.rank,
                                                "joker": c.joker,
                                            })
                                        }).collect::<Vec<_>>(),
                                    })
                                }).collect::<Vec<_>>(),
                            });
                            let _ = tx.send(start_response.to_string());
                        }
                    }
                }
            }
        }
        "play_card" => {
            let card_index = msg["card_index"].as_u64().unwrap_or(0) as usize;

            let mut game_rooms = state.game_rooms.write();
            if let Some(game) = game_rooms.get_mut(room_id) {
                if let Some(player_idx) = game.players.iter().position(|p| p.user_id == user_id) {
                    // 检查是否可以出牌
                    if player_idx == game.current_player && !game.round_cards[player_idx].is_some() {
                        if let Some(card) = game.play_card(player_idx, card_index) {
                            let response = json!({
                                "type": "card_played",
                                "user_id": user_id,
                                "card": {
                                    "suit": card.suit,
                                    "rank": card.rank,
                                    "joker": card.joker,
                                },
                                "current_player": game.current_player,
                                "lead_suit": game.lead_suit,
                            });
                            let _ = tx.send(response.to_string());

                            // 检查回合结束
                            if game.round_cards.iter().all(|c| c.is_some()) {
                                let round_response = json!({
                                    "type": "round_end",
                                    "scores": game.scores,
                                    "round_number": game.round_number,
                                    "current_player": game.current_player,
                                });
                                let _ = tx.send(round_response.to_string());
                            }

                            // 检查游戏结束
                            if game.is_game_over() {
                                if let Some((success, score, bid)) = game.get_result() {
                                    let end_response = json!({
                                        "type": "game_end",
                                        "success": success,
                                        "bidder_score": score,
                                        "bid": bid,
                                        "scores": game.scores,
                                    });
                                    let _ = tx.send(end_response.to_string());
                                }
                            }
                        }
                    }
                }
            }
        }
        "start_game" => {
            // 房主开始游戏
            let mut game_rooms = state.game_rooms.write();
            if let Some(game) = game_rooms.get_mut(room_id) {
                if game.phase == GamePhase::Waiting {
                    game.deal_cards();

                    let response = json!({
                        "type": "game_started",
                        "phase": "bidding",
                        "current_player": game.current_player,
                        "players": game.players.iter().map(|p| {
                            json!({
                                "user_id": p.user_id,
                                "username": p.username,
                                "cards_count": p.cards.len(),
                            })
                        }).collect::<Vec<_>>(),
                    });
                    let _ = tx.send(response.to_string());
                }
            }
        }
        _ => {}
    }

    Ok(())
}