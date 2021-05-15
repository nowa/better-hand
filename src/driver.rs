use rayon::prelude::*;
use rs_poker::core::*;

// Take a deck of the remaining cards, generate single hands
pub fn enemy_hands(remaining: &FlatDeck) -> Vec<Hand> {
    CardIter::new(&remaining[..], 2)
        .map(|v| Hand::new_with_cards(v))
        .collect()
}

// take a user's hand and the board, generate the rank
pub fn hand_board_rank(hand: Hand, board: Hand) -> Rank {
    let full_hand: Vec<Card> = board
        .cards()
        .into_iter()
        .chain(hand.cards().into_iter())
        .map(|c| *c)
        .collect();
    full_hand.rank()
}

pub fn calc(user_hand: Hand, board: Hand, remaining: FlatDeck) -> Vec<Hand> {
    let usr_rank: Rank = hand_board_rank(user_hand, board.clone());
    let enemy: Vec<Hand> = enemy_hands(&remaining);
    let beats_us: Vec<Hand> = enemy
        .into_par_iter()
        .filter(|hand| hand_board_rank(hand.clone(), board.clone()) > usr_rank)
        .collect();
    beats_us
}
