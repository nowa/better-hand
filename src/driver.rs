use crate::types::{Beats, PokerHand};
use rayon::prelude::*;
use rs_poker::core::*;
use std::collections::HashMap;

// Take a deck of the remaining cards, generate single hands
fn enemy_hands(remaining: &FlatDeck) -> Vec<Vec<Card>> {
    CardIter::new(&remaining[..], 2).collect()
}

// take a user's hand and the board, generate the rank
fn hand_board_rank(hand: &Vec<Card>, board: &Vec<Card>) -> Rank {
    board
        .iter()
        .chain(hand.iter())
        .map(|c| *c)
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
            we_beat: hand_board_rank(&hand, &board) < usr_rank,
        })
        .collect()
}

pub fn deck_without(hand: &Vec<Card>, board: &Vec<Card>) -> Deck {
    let mut deck = Deck::default();
    for card in board {
        deck.remove(*card);
    }

    for card in hand {
        deck.remove(*card);
    }

    deck
}

pub fn river_calc(
    user_hand: Vec<Card>,
    board: Vec<Card>,
    remaining: Deck,
) -> HashMap<PokerHand, f64> {
    let usr_rank: Rank = hand_board_rank(&user_hand, &board);
    let enemy: Vec<Vec<Card>> = enemy_hands(&remaining.flatten());
    prob_calc(hand_beats(usr_rank, board, enemy))
}

pub fn turn_calc(
    user_hand: Vec<Card>,
    board: Vec<Card>,
    remaining: Deck,
) -> HashMap<PokerHand, f64> {
    let remaining_cards: Vec<Card> = remaining.into_iter().collect();
    let beats: Vec<Beats> = remaining_cards
        .into_par_iter()
        .map(|card| {
            let mut river_board = board.clone();
            river_board.push(card);
            let river_remaining = deck_without(&user_hand, &river_board);

            let usr_rank: Rank = hand_board_rank(&user_hand, &river_board);
            let enemy: Vec<Vec<Card>> = enemy_hands(&river_remaining.flatten());
            hand_beats(usr_rank, river_board, enemy)
        })
        .flatten()
        .collect();

    prob_calc(beats)
}
