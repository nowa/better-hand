use crate::types::{Beats, PokerHand};
use rayon::prelude::*;
use rs_poker::core::*;
use std::collections::HashMap;

// Take a deck of the remaining cards, generate single hands
fn enemy_hands(remaining: &FlatDeck) -> Vec<Vec<Card>> {
    CardIter::new(&remaining[..], 2).collect()
}

// take a user's hand and the board, generate the rank
fn hand_board_rank(hand: Vec<Card>, board: Vec<Card>) -> Rank {
    board
        .into_iter()
        .chain(hand.into_iter())
        .collect::<Vec<Card>>()
        .rank()
}

// Given a list of binary occurrences, calculate the prob(true) with laplace smoothing
fn prob(list: Vec<bool>) -> f64 {
    let count: usize = list.iter().filter(|&&x| x == true).count();
    let total = list.len();
    (count as f64) / (total as f64)
}

// Given a set of outcomes, present the output as a map of hands to possibilities
fn prob_calc(outcomes: Vec<Beats>) -> HashMap<PokerHand, f64> {
    let mut string_hands: HashMap<PokerHand, Vec<bool>> = HashMap::new();
    for item in outcomes {
        let as_str = PokerHand::new(item.hand);
        let mut v: Vec<bool> = match string_hands.get(&as_str) {
            Some(x) => x.clone(),
            None => Vec::new(),
        };

        v.push(item.we_beat);
        string_hands.insert(as_str, v);
    }

    let mut probs: HashMap<PokerHand, f64> = HashMap::new();
    for (name, v) in string_hands {
        probs.insert(name, prob(v));
    }

    probs
}

fn hand_beats(usr_rank: Rank, board: Vec<Card>, enemy: Vec<Vec<Card>>) -> Vec<Beats> {
    enemy
        .into_par_iter()
        .map(|hand| Beats {
            hand: (hand[0], hand[1]),
            we_beat: hand_board_rank(hand, board.clone()) < usr_rank,
        })
        .collect()
}

pub fn river_calc(
    user_hand: Vec<Card>,
    board: Vec<Card>,
    remaining: FlatDeck,
) -> HashMap<PokerHand, f64> {
    let usr_rank: Rank = hand_board_rank(user_hand, board.clone());
    let enemy: Vec<Vec<Card>> = enemy_hands(&remaining);
    prob_calc(hand_beats(usr_rank, board, enemy))
}
