use std::cmp::Ordering;

use rand::prelude::*;
use serde::{Deserialize, Serialize};
use crate::engine::card::{compare_hands, FullHand};

use super::{card::{get_best_hand, get_new_deck, Card, Color, HandType, Rank}, player::Player};

#[derive(Serialize, Deserialize, Clone)]
pub enum PlayerAction {
    Fold,
    Check,
    Call,
    Raise(i32)
}

#[derive(Serialize, Clone, Debug)]
pub struct Table {
    pub name: String,
    pub players: [Option<Player>; 8],
    pub revealed_cards: usize,
    pub cards: [Option<Card>; 5],
    pub pot: i32,
    pub current_required_bet: i32,
    pub current_player_index: usize,
    pub button_index: usize,

    pub is_game_running: bool,

    pub minimal_bid: i32,
    pub max_players: usize,
    pub starting_chips: i32,
}


impl Table {
    pub fn new(host_name: String, host_key: String) -> Self {
        let mut players = [const { None }; 8];
        players[0] = Some(Player::new(host_name, host_key, 100));
        Self {
            name: "Table".to_string(),
            players,
            revealed_cards: 0,
            cards: [const { None }; 5],
            pot: 0,
            current_required_bet: 0,
            current_player_index: 0,
            button_index: 8,
            is_game_running: false,
            minimal_bid: 10,
            max_players: 8,
            starting_chips: 100,
        }
    }

    pub fn add_player(&mut self, name: String, key: String) -> bool {
        for (i, player) in self.players.iter().enumerate() {
            if let Some(_) = player { continue; }
            if i > self.max_players { return false; }

            self.players[i] = Some(Player::new(name, key, self.starting_chips));
            return true;
        }

        false
    }

    pub fn remove_player(&mut self, index: usize) -> bool {
        if let Some(_) = self.players[index] {
            self.players[index] = None;
            if index != 7 {
                for i in index..7 {
                    if let Some(player) = &self.players[i+1] {
                        self.players[i] = Some(player.clone());
                        self.players[i+1] = None;
                    }
                }
            }
            return true;
        }
        return false;
    }

    pub fn print(&self) {
        // print!("{}[2J", 27 as char);
        println!("{}", self.name);

        println!("\t{}\n\t{}", self.pot, self.current_required_bet);


        for (i, player) in self.players.iter().enumerate() {
            if let Some(player) = player {
                println!("\t{}: {} {} {} {}", player.name, player.chips, player.current_bet, player.has_folded, {if self.current_player_index == i { "<" } else { "" }});
                for card in player.cards.clone() {
                    if let Some(card) = card {
                        println!("\t\t{:?}", card);
                    }
                    else { println!("\t\tNone"); }
                }
            }
        }
        println!("\n");

        for (i, card) in self.cards.iter().enumerate() {
            if i < self.revealed_cards {
                println!("\t{:?}", card);
            }
        }

        println!("{}", self.button_index);
    }
    

    pub fn start_new_game(&mut self) -> bool {
        let mut current_player_count = 0;
        for player in &self.players {
            if let Some(_) = player {
                current_player_count += 1;
            }
        }
        if current_player_count < 3 { return false }

        self.is_game_running = true;
        self.reroll_cards();
        self.increment_button();

        true
    }

    fn end_game(&mut self) {
        let mut best_hand = FullHand{
            hand_type: HandType::HighCard,
            ranks: vec![ Rank::Two, Rank::Two, Rank::Two, Rank::Two, Rank::Two ],
        };
        let mut best_hand_player_index = 0;
        let mut tied_indexes = Vec::new();

        self.is_game_running = false;
        self.current_required_bet = 0;

        

        for (i, player) in &mut self.players.iter().enumerate() {
            if let Some(player) = player {
                if player.has_folded { continue; }

                let mut all_cards = [Card { color: Color::Heart, rank: Rank::Two }; 7];
                for i in 0..5 { all_cards[i] = self.cards[i].unwrap(); }
                for i in 0..2 { all_cards[i+5] = player.cards[i].unwrap(); }

                let result = compare_hands(&get_best_hand(all_cards), &best_hand);

                if result == Ordering::Greater {
                    best_hand_player_index = i;
                    best_hand = get_best_hand(all_cards);
                    tied_indexes = Vec::new();
                    continue;
                }
                if result == Ordering::Equal {
                    tied_indexes.push(i);
                    continue;
                }
            }
        }

        if tied_indexes != Vec::<usize>::new() {
            let len = tied_indexes.len();
            for i in tied_indexes {
                self.players[i].as_mut().unwrap().chips += self.pot / len as i32;
            }
        }
        else {
            self.players[best_hand_player_index].as_mut().unwrap().chips += self.pot;
        }

        self.pot = 0;
    }

    fn reveal_next_cards(&mut self) {
        if self.revealed_cards < 3 {
            self.revealed_cards = 3;
        }
        else if self.revealed_cards < 5{
            self.revealed_cards += 1;
        }
        else {
            self.end_game();
            return;
        }

        self.current_required_bet = 0;
        for player in &mut self.players {
            if let Some(player) = player {
                player.has_acted = false;
                player.current_bet = 0;
            }
        }

        return;
    }

    fn reroll_cards(&mut self) {
        let mut rng = rand::rng();
        let mut deck = get_new_deck();
        deck.shuffle(&mut rng);


        for player in &mut self.players {
            if let Some(player) = player {
                player.has_acted = false;
                player.has_folded = false;
                player.current_bet = 0;
                for i in 0..2 {
                    let deck_clone = deck.clone();
                    let (j, card) = deck_clone.iter().enumerate().choose(&mut rng).unwrap();
                    deck.remove(j);
                    player.cards[i] = Some(card.clone());
                }
            }
        }

        for i in 0..5 {
            let deck_clone = deck.clone();
            let (j, card) = deck_clone.iter().enumerate().choose(&mut rng).unwrap();
            deck.remove(j);
            self.cards[i] = Some(card.clone());
        }
        self.revealed_cards = 0;

        return;
    }

    fn increment_button(&mut self) {
        if self.button_index == 8 { self.button_index = 0; }
        else { self.button_index += 1; }

        for (i, player) in self.players.iter().enumerate() {
            if let Some(_) = player { continue; }
            if self.button_index >= i { self.button_index = 0;}
            break;
        }
        
        self.current_player_index = self.button_index;
        self.increment_current_player();
        for _ in 0..=1 {
            let _ = self.player_action(PlayerAction::Raise(5));
        }
        return;
    }

    fn increment_current_player(&mut self) {
        if self.current_player_index == 8 { self.current_player_index = 0 }
        else { self.current_player_index += 1; }
        if let Some(player) = &self.players[self.current_player_index] {
            if player.has_folded { self.increment_current_player(); }
        }
        else {
            self.current_player_index = 0;
            if let Some(player) = &self.players[self.current_player_index] {
                if player.has_folded { self.increment_current_player(); }
            }
        }

        return;
    }

    fn check_for_round_end(&mut self) {
        if !self.is_game_running { return; }

        let mut round_ended = false;
        let mut folded_count = 0;
        let mut total_count = 0;
        for player in self.players.clone() {
            if let Some(player) = player {
                total_count += 1;
                if player.has_folded { folded_count += 1; }
            }
        }
        if (folded_count == total_count || folded_count == total_count - 1) && total_count != 1 { self.end_game(); return; }

        for player in self.players.clone() {
            if let Some(player) = player {
                if !player.has_acted { break; }
                if player.current_bet != self.current_required_bet
                { return; }

                round_ended = true;
            }
        }

        if round_ended {
            self.current_player_index = 8;
            self.increment_current_player();
            self.reveal_next_cards();
        }
        return;
    }

    pub fn player_action(&mut self, action: PlayerAction) -> bool {
        if let Some(player) = &mut self.players[self.current_player_index] {
            match action {
                PlayerAction::Fold => {
                    player.has_folded = true;
                    player.has_acted = true;
                    
                    self.check_for_round_end();
                    if self.is_game_running { self.increment_current_player(); }
                    return true;
                }
                PlayerAction::Check => {
                    if self.current_required_bet == 0 {
                        player.has_acted = true;
                        self.check_for_round_end();
                        if self.is_game_running { self.increment_current_player(); }
                    }
                    return true;
                },
                PlayerAction::Call => {
                    player.has_acted = true;
                    let additional_chips;
                    if player.chips < self.current_required_bet - player.current_bet {
                        additional_chips = player.chips;
                    }
                    else {
                        additional_chips = self.current_required_bet - player.current_bet;
                    }

                    player.chips -= additional_chips;
                    player.current_bet += additional_chips;
                    self.pot += additional_chips;

                    
                    self.check_for_round_end();
                    if self.is_game_running { self.increment_current_player(); }
                    return true;
                },
                PlayerAction::Raise(val) => {
                    if val > player.chips { return false; }
                    player.has_acted = true;

                    player.chips -= val + (self.current_required_bet - player.current_bet);
                    self.pot += val + (self.current_required_bet - player.current_bet);

                    player.current_bet += val + (self.current_required_bet - player.current_bet);
                    self.current_required_bet += val;


                    self.check_for_round_end();
                    if self.is_game_running { self.increment_current_player(); }
                    return true;
                },
            }
        }
        else { self.increment_current_player(); return false; }
    }
}
