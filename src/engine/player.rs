use serde::Serialize;

use super::card::Card;

#[derive(Serialize, Clone, Debug)]
pub struct Player {
    pub name: String,
    pub cards: [Option<Card>; 2],
    pub chips: i32,
    pub current_bet: i32,
    pub has_acted: bool,
    pub has_folded: bool,
    pub key: String
}


impl Player {
    pub fn new(name: String, key: String, chips: i32) -> Self {
        Self {
            name,
            cards: [const { None }; 2],
            chips,
            current_bet: 0,
            has_acted: false,
            has_folded: false,
            key,
        }
    } 
}
