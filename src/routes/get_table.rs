use std::sync::{Arc, Mutex};

use axum::{extract::State, http::StatusCode, Json};
use serde::{Deserialize, Serialize};
use crate::engine::{card::{get_best_hand, Card, Color, FullHand, Rank}, Engine};


pub async fn get_table(
    State(engine): State<Arc<Mutex<Engine>>>,
    Json(Key { key }): Json<Key>
) -> Result<Json<PubTable>, StatusCode> {
    for table in engine.lock().unwrap().get_tables() {
        if table.players.iter().any(|e| {if let Some(player) = e { player.key == key } else { false }}) {
            let mut players = [const { None }; 8];
            let mut revealed_cards = [const { None }; 5];
            let mut is_first_game = false;
            let mut player_index = 10;
            for (i, player) in table.players.iter().enumerate() {
                if let Some(player) = player {
                    if key == player.key { player_index = i; }

                    let mut all_cards = [Card { color: Color::Heart, rank: Rank::Two }; 7];
                    for i in 0..5 { 
                        if let Some(card) = table.cards[i] {
                            all_cards[i] = card;
                        }
                        else {
                            is_first_game = true;
                        }
                    }
                    for i in 0..2 {
                        if let Some(card) = player.cards[i] {
                            all_cards[i+5] = card;
                        }
                        else {
                            is_first_game = true;
                        }
                    }
                    
                    if table.is_game_running || is_first_game {
                        players[i] = Some(PubPlayer {
                            name: player.name.clone(),
                            current_bet: player.current_bet,
                            chips: player.chips,
                            has_folded: player.has_folded,
                            cards: {if key == player.key { player.cards.clone() } else { [const { None }; 2] }},
                            best_hand: None,
                        });
                        continue;
                    }

                    players[i] = Some(PubPlayer {
                        name: player.name.clone(),
                        current_bet: player.current_bet,
                        chips: player.chips,
                        has_folded: player.has_folded,
                        cards: player.cards.clone(),
                        best_hand: Some(get_best_hand(all_cards)),
                    });
                }
            }
        
            for i in 0..table.revealed_cards {
                revealed_cards[i as usize] = table.cards[i as usize].clone();
            }
        
        
            return Ok(Json(PubTable {
                name: table.name.clone(),
                player: player_index,
                players,
                revealed_cards,
                pot: table.pot,
                current_required_bet: table.current_required_bet,
                current_player_index: table.current_player_index,
                button_index: table.button_index,
                is_game_running: table.is_game_running,
                minimal_bid: table.minimal_bid,
                max_players: table.max_players,
                starting_chips: table.starting_chips,
            }));
        }
    }

    Err(StatusCode::NOT_FOUND)
}

#[derive(Serialize)]
pub struct PubTable {
    name: String,
    player: usize,
    players: [Option<PubPlayer>; 8],
    revealed_cards: [Option<Card>; 5],
    pot: i32,
    current_required_bet: i32,
    current_player_index: usize,
    button_index: usize,
    is_game_running: bool,
    minimal_bid: i32,
    max_players: usize,
    starting_chips: i32,
}

#[derive(Serialize)]
pub struct PubPlayer {
    name: String,
    chips: i32,
    current_bet: i32,
    has_folded: bool,
    cards: [Option<Card>; 2],
    best_hand: Option<FullHand>,
}

#[derive(Deserialize)]
pub struct Key {
    pub key: String
}