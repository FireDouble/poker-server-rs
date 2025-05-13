use std::sync::{Arc, Mutex};

use axum::{extract::State, http::StatusCode, Json};
use crate::engine::Engine;

use super::get_table::Key;

pub async fn start_game(
    State(engine): State<Arc<Mutex<Engine>>>,
    Json(Key { key }): Json<Key>
) -> StatusCode {
    let mut engine = engine.lock().unwrap();

    for table in engine.get_tables().iter_mut() {
        for (i, player) in table.players.clone().into_iter().enumerate() {
            if let Some(player) = player {
                if player.key == key {
                    if i != 0 {
                        return StatusCode::UNAUTHORIZED;
                    }

                    let result = table.start_new_game();
                    if !result { return StatusCode::TOO_EARLY; }
                }
            }
        }
    }


    StatusCode::ACCEPTED
}