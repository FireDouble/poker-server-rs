use std::sync::{Arc, Mutex};

use axum::{extract::State, Json};
use serde::{Deserialize, Serialize};
use crate::engine::Engine;

pub async fn find_player (
    State(engine): State<Arc<Mutex<Engine>>>,
    Json(Key { key }): Json<Key>,
) -> Json<FindResponse> {
    let mut engine = engine.lock().unwrap();

    for (i, table) in engine.get_tables().iter().enumerate() {
        for player in table.players.clone() {
            if let Some(player) = player {
                if player.key != key { continue; }
                return Json(FindResponse { table: i as i32 });
            }
        }
    }

    return Json(FindResponse { table: -1 as i32 })
}


#[derive(Deserialize)]
pub struct Key {
    pub key: String
}

#[derive(Serialize)]
pub struct FindResponse {
    table: i32
}