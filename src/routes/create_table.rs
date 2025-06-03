use std::sync::{Arc, Mutex};

use axum::{extract::State, Json};
use chrono::Utc;
use serde::Deserialize;
use crate::engine::Engine;

pub async fn create_table(
    State(engine): State<Arc<Mutex<Engine>>>,
    Json(JoinRequest { name, table_name, max_players, minimal_bid, starting_chips }): Json<JoinRequest>,
) -> String {
    let key = name.clone() + &Utc::now().to_string();

    engine.lock().unwrap().new_table(name, table_name, max_players, minimal_bid, starting_chips, key.clone());

    key
}

#[derive(Deserialize)]
pub struct JoinRequest {
    name: String,
    table_name: String,
    max_players: usize,
    minimal_bid: i32,
    starting_chips: i32,
}
