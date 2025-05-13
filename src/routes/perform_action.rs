use std::sync::{Arc, Mutex};

use axum::{extract::State, http::StatusCode, Json};
use serde::{Deserialize, Serialize};
use crate::engine::{table::PlayerAction, Engine};

pub async fn perform_action(
    State(engine): State<Arc<Mutex<Engine>>>,
    Json(ActionRequest{ action, key} ): Json<ActionRequest>,
) -> StatusCode {
    let mut i = 0;
    let mut found_player = false;

    let mut engine = engine.lock().unwrap();

    for table in engine.get_tables() {
        for (j, player) in table.players.iter().enumerate() {
            if let Some(player) = player {
                if player.key != key { continue; }
                if j != table.current_player_index { continue; }
                found_player = true;
                break;
            }
        }
        if found_player { break; }
        i += 1;
    }
    if found_player {
        if let Some(table) = engine.get_tables().get_mut(i) {
            let result = table.player_action(action);
            if !result {
                return StatusCode::INTERNAL_SERVER_ERROR;
            }
        } else {
            return StatusCode::INTERNAL_SERVER_ERROR;
        }
    }

    StatusCode::ACCEPTED
}


#[derive(Deserialize, Serialize)]
pub struct ActionRequest {
    key: String,
    action: PlayerAction,
}