use std::sync::{Arc, Mutex};

use axum::{extract::State, http::StatusCode, Json};
use serde::Deserialize;
use crate::engine::Engine;

pub async fn exit_table(
    State(engine): State<Arc<Mutex<Engine>>>,
    Json(Key { key }): Json<Key>,
) -> StatusCode {
    let mut engine = engine.lock().unwrap();

    for table in engine.get_tables() {
        for (i, player) in table.players.iter().enumerate() {
            if let Some(player) = player {
                if player.key != key { continue; }

                let result = table.remove_player(i);
                if !result { return StatusCode::INTERNAL_SERVER_ERROR; }
                return StatusCode::ACCEPTED;
            }
        }
    }

    StatusCode::UNAUTHORIZED
}

#[derive(Deserialize)]
pub struct Key {
    key: String
}