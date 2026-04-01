mod state;
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

    let cors = tower_http::cors::CorsLayer::new()
        .allow_origin(tower_http::cors::Any)
        .allow_methods(tower_http::cors::Any)
        .allow_headers(tower_http::cors::Any);

    let app = axum::Router::new()
        .route("/health", axum::routing::get(health_check))
        .route("/api/auth/register", axum::routing::post(auth::register))
        .route("/api/auth/login", axum::routing::post(auth::login))
        .route("/api/auth/verify", axum::routing::post(auth::verify))
        .route("/api/user/profile", axum::routing::get(user::get_profile))
        .route("/api/user/profile", axum::routing::put(user::update_profile))
        .route("/api/user/stats", axum::routing::get(user::get_stats))
        .route("/api/room/create", axum::routing::post(room::create_room))
        .route("/api/room/join", axum::routing::post(room::join_room))
        .route("/api/room/leave", axum::routing::post(room::leave_room))
        .route("/api/room/:id", axum::routing::get(room::get_room))
        .route("/api/room/:id/start", axum::routing::post(room::start_game))
        .route("/api/match", axum::routing::post(room::start_match))
        .route("/api/match/cancel", axum::routing::post(room::cancel_match))
        .route("/ws/game/:room_id", axum::routing::get(ws::game_ws))
        .layer(cors)
        .with_state(state);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:8080").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn health_check() -> &'static str {
    "OK"
}