use std::sync::{Arc, Mutex};

use axum::{extract::State, Json};
use serde::{Deserialize, Serialize};
use crate::engine::Engine;

pub async fn search_tables(
    State(engine): State<Arc<Mutex<Engine>>>,
    Json(criteria): Json<SearchCriteria>, 
) -> Json<Vec<PubTable>> {
    let mut response = Vec::new();

    for (id, table) in engine.lock().unwrap().get_tables().iter().enumerate() {
        let name = table.name.clone();
        let max_players = table.max_players;
        let minimal_bid = table.minimal_bid;
        let starting_chips = table.starting_chips;
        let mut current_players = 0;

        for player in table.players.clone() {
            if let Some(_) = player { current_players += 1; }
        }

        if (max_players == criteria.max_players || criteria.max_players == 0)
        && (minimal_bid == criteria.minimal_bid || criteria.minimal_bid == 0)
        && (starting_chips == criteria.starting_chips || criteria.starting_chips == 0)
        && (current_players == criteria.current_players || criteria.current_players == 0)
        && (name == criteria.name)
        {
            response.push(PubTable { id, name: table.name.clone(), current_players, max_players, minimal_bid, starting_chips });
            continue;
        }
    }

    Json(response)
}


#[derive(Serialize)]
pub struct PubTable {
    id: usize,
    name: String,
    current_players: i32,
    max_players: usize,
    minimal_bid: i32,
    starting_chips: i32,
}

#[derive(Deserialize)]
pub struct SearchCriteria {
    name: String,
    max_players: usize,
    current_players: i32,
    minimal_bid: i32,
    starting_chips: i32,
}