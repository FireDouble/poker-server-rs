use std::{cmp::Ordering, collections::HashMap};

use serde::Serialize;
use strum::IntoEnumIterator;
use strum_macros::EnumIter;

#[derive(Serialize, Copy, Clone, EnumIter, PartialEq, Eq, Hash, PartialOrd, Ord, Debug)]
pub enum Color {
    Heart,
    Diamond,
    Club,
    Spade
}

#[derive(Serialize, Copy, Clone, EnumIter, PartialEq, Eq, Hash, PartialOrd, Ord, Debug)]
pub enum Rank {
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    Jack,
    Queen,
    King,
    Ace,
}


#[derive(Serialize, Copy, Clone, PartialEq, Debug)]
pub struct Card {
    pub color: Color,
    pub rank: Rank
}


#[derive(PartialEq, Eq, Copy, Clone, PartialOrd, Ord, Serialize, Debug)]
pub enum HandType {
    HighCard,
    OnePair,
    TwoPair,
    ThreeOfAKind,
    Straight,
    Flush,
    FullHouse,
    FourOfAKind,
    StraightFlush,
}

#[derive(Serialize)]
pub struct FullHand {
    pub hand_type: HandType,
    pub ranks: Vec<Rank>,
}


pub fn get_new_deck() -> Vec<Card> {
    let mut vec = Vec::new();
    for color in Color::iter() {
        for rank in Rank::iter() {
            vec.push(Card { color: color.clone(), rank })
        }
    }

    vec
}

pub fn compare_hands(hand1: &FullHand, hand2: &FullHand) -> Ordering {
    // use HandType::*;

    for (a, b) in hand1.ranks.iter().zip(hand2.ranks.iter()) {
        let ord = a.cmp(b);
        if ord != Ordering::Equal {
            return ord;
        }
    }

    Ordering::Equal    
    // match (hand1.hand_type, hand2.hand_type) {
    //     (StraightFlush, StraightFlush) => hand1.ranks[0].cmp(&hand2.ranks[0]),
    //     (FourOfAKind, FourOfAKind) => hand1.ranks[0].cmp(&hand2.ranks[0]).then(hand1.ranks[1].cmp(&hand2.ranks[1])),
    //     (FullHouse, FullHouse) => hand1.ranks[0].cmp(&hand2.ranks[0]).then(hand1.ranks[1].cmp(&hand2.ranks[1])),
    //     (Flush, Flush) => compare_rank_lists(&hand1.ranks, &hand2.ranks),
    //     (Straight, Straight) => hand1.ranks[0].cmp(&hand2.ranks[0]),
    //     (ThreeOfAKind, ThreeOfAKind) => hand1.ranks[0].cmp(&hand2.ranks[0]).then(compare_rank_lists(&hand1.ranks, &hand2.ranks)),
    //     (TwoPair, TwoPair) => hand1.ranks[0].cmp(&hand2.ranks[0]).then(hand1.ranks[1].cmp(&hand2.ranks[1])).then(hand1.ranks[2].cmp(&hand2.ranks[2])),
    //     (OnePair, OnePair) => hand1.ranks[0].cmp(&hand2.ranks[0]).then(compare_rank_lists(&hand1.ranks, &hand2.ranks)),
    //     (HighCard, OnePair) => hand1.ranks[0].cmp(&hand2.ranks[0]).then(compare_rank_lists(&hand1.ranks, &hand2.ranks)),
    //     _ => compare_rank_lists(&hand1.ranks, &hand2.ranks)
    // }
}

// fn compare_rank_lists(r1: &Vec<Rank>, r2: &Vec<Rank>) -> Ordering{
//     for (a, b) in r1.iter().zip(r2.iter()) {
//         let ord = a.cmp(b);
//         if ord != Ordering::Equal {
//             return ord;
//         }
//     }

//     Ordering::Equal
// }

pub fn get_best_hand(cards: [Card; 7]) -> FullHand {
    let mut color_counts = HashMap::new();
    let mut rank_counts = HashMap::new();
    
    let mut ranks: Vec<Rank> = cards.iter().map(|c| c.rank.clone()).collect();
    ranks.sort();
    ranks.reverse();

    for card in &cards {
        *rank_counts.entry(&card.rank).or_insert(0) += 1 as usize;
        color_counts.entry(&card.color).or_insert(Vec::new()).push(&card.rank);
    }

    if let Some(high) = find_straight_flush(&color_counts) {
        return FullHand {
            hand_type: HandType::StraightFlush,
            ranks: vec![high]
        };
    }

    if let Some((four, kicker)) = find_four_of_a_kind(&rank_counts, &ranks) {
        return FullHand{
            hand_type: HandType::FourOfAKind,
            ranks: vec![four, kicker]
        }
    }

    if let Some((three, pair)) = find_full_house(&rank_counts) {
        return FullHand {
            hand_type: HandType::FullHouse,
            ranks: vec![three, pair]
        };
    }

    if let Some(flush_ranks) = find_flush(&color_counts) {
        return FullHand {
            hand_type: HandType::Flush,
            ranks: flush_ranks
        };
    }

    if let Some(high) = find_straight(&ranks) {
        return FullHand {
            hand_type: HandType::Straight,
            ranks: vec![high]
        };
    }

    if let Some((three, mut kickers)) = find_three_of_a_kind(&rank_counts, &ranks) {
        let mut hand_ranks = vec![three];
        hand_ranks.append(&mut kickers);
        return FullHand {
            hand_type: HandType::ThreeOfAKind,
            ranks: hand_ranks
        };
    }

    if let Some((high_pair, low_pair, kicker)) = find_two_pair(&rank_counts,  &ranks) {
        return FullHand {
            hand_type: HandType::TwoPair,
            ranks: vec![high_pair, low_pair, kicker]
        };
    }

    if let Some((pair, mut kickers)) = find_one_pair(&rank_counts, &ranks) {
        let mut hand_ranks = vec![pair];
        hand_ranks.append(&mut kickers);
        return FullHand{
            hand_type: HandType::OnePair,
            ranks: hand_ranks,
        };
    }

    // High card
    FullHand {
        hand_type: HandType::HighCard,
        ranks: ranks.iter().take(5).cloned().collect()
    }
}

fn find_straight_flush(color_count: &HashMap<&Color, Vec<&Rank>>) -> Option<Rank> {
    for (_, ranks) in color_count {
        if ranks.len() >= 5 {
            let mut sorted = ranks.iter().map(|r| (*r).clone()).collect::<Vec<_>>();
            sorted.sort();
            sorted.reverse();
            if let Some(high) = find_straight(&sorted) {
                return Some(high);
            }
        }
    }

    None
}

fn find_four_of_a_kind(rank_counts: &HashMap<&Rank, usize>, ranks: &[Rank]) -> Option<(Rank, Rank)> {
    for (&rank, &count) in rank_counts {
        if count == 4 {
            let kicker = ranks.iter().filter(|&r| r != rank).next().unwrap().clone();
            return Some((rank.clone(), kicker));
        }
    }
    None
}

fn find_full_house(rank_counts: &HashMap<&Rank, usize>) -> Option<(Rank, Rank)> {
    let mut three = None;
    let mut pair = None;

    for (&rank, &count) in rank_counts {
        if count >= 3 {
            if three.is_none() { three = Some(rank.clone()); }
        } else if count >= 2 {
            if pair.is_none() { pair = Some(rank.clone()); }
        }
    }

    if three.is_some() && pair.is_some() {
        return Some((three.unwrap(), pair.unwrap()));
    }

    None
}

fn find_flush(color_counts: &HashMap<&Color, Vec<&Rank>>) -> Option<Vec<Rank>> {
    for (_, ranks) in color_counts {
        if ranks.len() >= 5 {
            let mut sorted = ranks.iter().map(|r| (*r).clone()).collect::<Vec<_>>();
            sorted.sort();
            sorted.reverse();
            return Some(sorted.into_iter().take(5).collect());
        }
    }
    None
}

fn find_straight(ranks: &[Rank]) -> Option<Rank> {
    let mut unique = ranks.to_vec();
    unique.dedup();

    for i in 0..(unique.len() - 5).max(0) {
        let values: Vec<u8> = unique[i..i + 5].iter().map(|r| match r {
            Rank::Two => 2,
            Rank::Three => 3,
            Rank::Four => 4,
            Rank::Five => 5,
            Rank::Six => 6,
            Rank::Seven => 7,
            Rank::Eight => 8,
            Rank::Nine => 9,
            Rank::Ten => 10,
            Rank::Jack => 11,
            Rank::Queen => 12,
            Rank::King => 13,
            Rank::Ace => 14,
        }).collect();
    
        let consecutive = values.windows(2).all(|w| w[0] == w[1] + 1);

        if consecutive {
            return Some(unique[i].clone())
        }

        if unique.contains(&Rank::Ace)
        && unique.contains(&Rank::Two)
        && unique.contains(&Rank::Three)
        && unique.contains(&Rank::Four)
        && unique.contains(&Rank::Five) {
            return Some(Rank::Five);
        }
    }

    None
}

fn find_three_of_a_kind(rank_counts: &HashMap<&Rank, usize>, ranks: &[Rank]) -> Option<(Rank, Vec<Rank>)> {
    for (&rank, &count) in rank_counts {
        if count == 3 {
            let kickers = ranks.iter().filter(|&r| r != rank).cloned().take(2).collect();
            return Some((rank.clone(), kickers));
        }
    }
    None
}

fn find_two_pair(rank_counts: &HashMap<&Rank, usize>, ranks: &[Rank]) -> Option<(Rank, Rank, Rank)> {
    let mut pairs = rank_counts.iter()
        .filter(|(_, &c)| c == 2)
        .map(|(r, _)| (*r).clone())
        .collect::<Vec<_>>();
    pairs.sort();
    pairs.reverse();

    if pairs.len() >= 2 {
        let kicker = ranks.iter()
            .filter(|&r| *r != pairs[0] && *r != pairs[1])
            .next()
            .unwrap()
            .clone();
        return Some((pairs[0].clone(), pairs[1].clone(), kicker));
    }
    None
}

fn find_one_pair(rank_counts: &HashMap<&Rank, usize>, ranks: &[Rank]) -> Option<(Rank, Vec<Rank>)> {
    for (&rank, &count) in rank_counts {
        if count == 2 {
            let kickers = ranks.iter().filter(|&r| r != rank).cloned().take(3).collect();
            return Some((rank.clone(), kickers));
        }
    }
    None
}