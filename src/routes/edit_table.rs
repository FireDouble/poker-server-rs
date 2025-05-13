use std::sync::{Arc, Mutex};

use axum::{extract::State, http::StatusCode, Json};
use serde::Deserialize;
use crate::engine::Engine;

pub async fn edit_table(
    State(engine): State<Arc<Mutex<Engine>>>,
    Json(settings): Json<TableSettings>,
) -> StatusCode {
    let mut engine = engine.lock().unwrap();
    for table in engine.get_tables() {
        for player in table.players.iter() {
            if let Some(player) = player {
                if player.key != settings.key { continue; }

                if settings.minimal_bid <= 0
                || settings.max_players <= 0
                || settings.max_players >= 8
                || settings.starting_chips <= 0 {
                    return StatusCode::BAD_REQUEST;
                }

                table.name = settings.name;
                table.minimal_bid = settings.minimal_bid;
                table.max_players = settings.max_players;
                table.starting_chips = settings.starting_chips;

                return StatusCode::ACCEPTED;
            }
        }
    }

    StatusCode::UNAUTHORIZED
}

#[derive(Deserialize)]
pub struct TableSettings {
    key: String,
    name: String,
    minimal_bid: i32,
    max_players: usize,
    starting_chips: i32,
}