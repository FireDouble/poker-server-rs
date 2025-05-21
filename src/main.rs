use axum::{
    http::{header::CONTENT_TYPE, Method}, routing::{get, post}, Router
};
use engine::Engine;
use tower_http::cors::{Any, CorsLayer};
use std::sync::{Arc, Mutex};
use routes::{create_table::create_table, edit_table::edit_table, exit_table::exit_table, find_player::find_player, get_table::get_table, join_table::join_table, perform_action::perform_action, search_tables::search_tables, start_game::start_game};

pub mod engine;
mod routes;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    let engine = Arc::new(Mutex::new(Engine::new()));

    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods([Method::GET, Method::POST])
        .allow_headers([CONTENT_TYPE]);

    let app = Router::new()
        .route("/get_table", post(get_table))
        .route("/create", post(create_table))
        .route("/join", post(join_table))
        .route("/exit", post(exit_table))
        .route("/edit", post(edit_table))
        .route("/action", post(perform_action))
        .route("/search", get(search_tables))
        .route("/start", post(start_game))
        .route("/find", post(find_player))
        .with_state(engine)
        .layer(cors);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
