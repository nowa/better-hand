extern crate clap;

use clap::{App, Arg};
use dialoguer::Input;
use rs_poker::core::*;

mod driver;
mod output;
mod types;

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
				.required_unless("interactive")
				.takes_value(true)
				.value_name("BOARD")
				.help("Takes a string of the board so far, with cards indicated in RANKsuit form (e.g. Th is the 10 of hearts). Cards are unseparated (e.g. AhAsAcAd9s)")
		)
		.arg(
			Arg::with_name("hand")
				.short("h")
				.long("hand")
				.required(true)
				.required_unless("interactive")
				.takes_value(true)
				.value_name("HAND")
				.help("Takes a string of your hole cards, with cards indicated in RANKsuit form (e.g. 9s is the 9 of spades). Cards are unseparated (e.g. AhAs)")
		).arg(
			Arg::with_name("interactive")
				.short("I")
				.long("interactive")
				.conflicts_with("board")
				.conflicts_with("hand")
				.takes_value(false)
				.help("Starts the tool in interactive mode")
		).get_matches();

    if matches.is_present("interactive") {
        loop {
            let input_hand: String = Input::new()
                .with_prompt("Hand")
                .allow_empty(false)
                .validate_with(|input: &String| -> Result<(), &str> {
                    if input.eq_ignore_ascii_case("exit") {
                        return Ok(());
                    }
                    let hand = Hand::new_from_str(input);
                    if hand.is_err() {
                        return Err("Hand improperly formatted");
                    }
                    let hand = hand.unwrap();
                    if hand.len() != 2 {
                        return Err("Hand contains incorrect length");
                    }

                    Ok(())
                })
                .interact_text()
                .unwrap();

            if input_hand.eq_ignore_ascii_case("exit") {
                break;
            }

            let hand: Hand = Hand::new_from_str(&input_hand).unwrap();

            let input_flop: String = Input::new()
                .with_prompt("Flop")
                .allow_empty(false)
                .validate_with(|input: &String| -> Result<(), &str> {
                    if input.eq_ignore_ascii_case("exit") {
                        return Ok(());
                    }
                    let flop = Hand::new_from_str(input);
                    if flop.is_err() {
                        return Err("Flop improperly formatted");
                    }
                    let flop = flop.unwrap();
                    if flop.len() != 3 {
                        return Err("Flop contains incorrect length");
                    }

                    Ok(())
                })
                .interact_text()
                .unwrap();

            if input_flop.eq_ignore_ascii_case("exit") {
                break;
            }

            let mut board: Hand = Hand::new_from_str(&input_flop).unwrap();

            let mut deck: Deck = Deck::default();
            for card in board.cards() {
                deck.remove(*card);
            }
            for card in hand.cards() {
                deck.remove(*card);
            }

            if deck.len() != (52 - 5) {
                println!("{}", "Some provided cards were non-unique");
                continue;
            }

            let enemy_wins: Vec<Hand> = driver::calc(hand.clone(), board.clone(), deck.flatten());
            let table = if matches.is_present("verbose") {
                output::pretty_print_cards(enemy_wins)
            } else {
                output::pretty_print(enemy_wins)
            };

            table.printstd();

            let input_turn: String = Input::new()
                .with_prompt("Turn")
                .allow_empty(false)
                .validate_with(|input: &String| -> Result<(), &str> {
                    if input.eq_ignore_ascii_case("exit") {
                        return Ok(());
                    }
                    let turn = Hand::new_from_str(input);
                    if turn.is_err() {
                        return Err("Turn improperly formatted");
                    }
                    let turn = turn.unwrap();
                    if turn.len() != 1 {
                        return Err("Turn contains incorrect length");
                    }

                    Ok(())
                })
                .interact_text()
                .unwrap();

            if input_turn.eq_ignore_ascii_case("exit") {
                break;
            }

            let turn: Hand = Hand::new_from_str(&input_turn).unwrap();
            board.push(turn.cards()[0]);

            let mut deck: Deck = Deck::default();
            for card in board.cards() {
                deck.remove(*card);
            }
            for card in hand.cards() {
                deck.remove(*card);
            }

            if deck.len() != (52 - 6) {
                println!("{}", "Some provided cards were non-unique");
                continue;
            }

            let enemy_wins: Vec<Hand> = driver::calc(hand.clone(), board.clone(), deck.flatten());
            let table = if matches.is_present("verbose") {
                output::pretty_print_cards(enemy_wins)
            } else {
                output::pretty_print(enemy_wins)
            };

            table.printstd();

            let input_river: String = Input::new()
                .with_prompt("River")
                .allow_empty(false)
                .validate_with(|input: &String| -> Result<(), &str> {
                    if input.eq_ignore_ascii_case("exit") {
                        return Ok(());
                    }
                    let river = Hand::new_from_str(input);
                    if river.is_err() {
                        return Err("River improperly formatted");
                    }
                    let river = river.unwrap();
                    if river.len() != 1 {
                        return Err("River contains incorrect length");
                    }

                    Ok(())
                })
                .interact_text()
                .unwrap();

            if input_river.eq_ignore_ascii_case("exit") {
                break;
            }

            let river: Hand = Hand::new_from_str(&input_river).unwrap();
            board.push(river.cards()[0]);

            let mut deck: Deck = Deck::default();
            for card in board.cards() {
                deck.remove(*card);
            }
            for card in hand.cards() {
                deck.remove(*card);
            }

            if deck.len() != (52 - 7) {
                println!("{}", "Some provided cards were non-unique");
                continue;
            }

            let enemy_wins: Vec<Hand> = driver::calc(hand.clone(), board.clone(), deck.flatten());
            let table = if matches.is_present("verbose") {
                output::pretty_print_cards(enemy_wins)
            } else {
                output::pretty_print(enemy_wins)
            };

            table.printstd();
        }
    } else {
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
        let table = if matches.is_present("verbose") {
            output::pretty_print_cards(enemy_wins)
        } else {
            output::pretty_print(enemy_wins)
        };

        table.printstd();
    }

    Ok(())
}
