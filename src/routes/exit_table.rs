use std::sync::{Arc, Mutex};

use axum::{extract::State, http::StatusCode, Json};
use serde::Deserialize;
use crate::engine::Engine;

pub async fn exit_table(
    State(engine): State<Arc<Mutex<Engine>>>,
    Json(Key { key }): Json<Key>,
) -> StatusCode {
    let mut engine = engine.lock().unwrap();

    for (j, table) in engine.get_tables().iter_mut().enumerate() {
        for (i, player) in table.players.iter().enumerate() {
            if let Some(player) = player {
                if player.key != key { continue; }

                let result = table.remove_player(i);
                if !result { return StatusCode::INTERNAL_SERVER_ERROR; }

                let mut current_players = 0;
                for player in table.players.iter().enumerate() {
                    if let Some(_) = player { current_players += 1; }
                }
                if current_players == 0 { engine.remove_table(j); }

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