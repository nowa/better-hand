extern crate clap;

use clap::{App, Arg};
use rs_poker::core::*;

mod driver;
mod output;

fn main() -> Result<(), String> {
    let matches = App::new("better-hand")
		.author("Adithya Chari, <adithya.chari@gmail.com>")
		.version("0.1.0")
		.about("Calculates opponent hands which beat yours for No-Limit Texas Hold 'Em")
		.arg(
			Arg::with_name("board")
				.short("b")
				.long("board")
				.required(true)
				.takes_value(true)
				.value_name("BOARD")
				.help("Takes a string of the board so far, with cards indicated in RANKsuit form (e.g. Th is the 10 of hearts). Cards are unseparated (e.g. AhAsAcAd9s)")
		)
		.arg(
			Arg::with_name("hand")
				.short("h")
				.long("hand")
				.required(true)
				.takes_value(true)
				.value_name("HAND")
				.help("Takes a string of your hole cards, with cards indicated in RANKsuit form (e.g. 9s is the 9 of spades). Cards are unseparated (e.g. AhAs)")
		).get_matches();

    // Grab hands from args
    let board = Hand::new_from_str(matches.value_of("board").unwrap())?;
    let hand = Hand::new_from_str(matches.value_of("hand").unwrap())?;

    // Check input lengths
    if board.len() < 3 || board.len() > 5 {
        return Err("Board should be post-flop".to_string());
    }
    if hand.len() != 2 {
        return Err("Hand length should be 2".to_string());
    }

    // Make a deck
    let mut deck: Deck = Deck::default();
    for card in board.cards() {
        deck.remove(*card);
    }
    for card in hand.cards() {
        deck.remove(*card);
    }

    // Verify that all provided cards were unique
    if deck.len() != (52 - board.len() - hand.len()) {
        return Err("Some provided cards were non-unique".to_string());
    }

    let enemy_wins: Vec<Hand> = driver::calc(hand, board, deck.flatten());

    let table = output::pretty_print(enemy_wins);
    table.printstd();
    Ok(())
}
