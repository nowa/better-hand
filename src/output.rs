use prettytable::{Attr, Cell, Row, Table};
use rayon::prelude::*;
use rs_poker::core::*;

fn value_flip(v: u8) -> u8 {
    12 - v
}

fn cell_color(i: u8, j: u8, occur: u8) -> u32 {
    if occur == 0 {
        if i <= j {
            // off
            4
        } else {
            7
        }
    } else {
        1
    }
}

fn position(hand: Hand) -> (u8, u8) {
    let card1: Card = hand.cards()[0];
    let card2: Card = hand.cards()[1];
    let a = value_flip(card1.value as u8);
    let b = value_flip(card2.value as u8);

    if card1.suit == card2.suit {
        if a > b {
            (b, a)
        } else {
            (a, b)
        }
    } else {
        if a > b {
            (a, b)
        } else {
            (b, a)
        }
    }
}

fn add_positions(indices: Vec<(u8, u8)>) -> [[u8; 13]; 13] {
    let mut arr = [[0u8; 13]; 13];
    for (a, b) in indices {
        arr[a as usize][b as usize] += 1;
    }
    arr
}

fn string_from_grid(i: u8, j: u8, count: u8) -> String {
    if i > j {
        // off suit
        format!(
            "{}{}{}: {}",
            Value::from_u8(value_flip(j)).to_char(),
            Value::from_u8(value_flip(i)).to_char(),
            "o",
            count,
        )
    } else if i < j {
        // suited
        format!(
            "{}{}{}: {}",
            Value::from_u8(value_flip(i)).to_char(),
            Value::from_u8(value_flip(j)).to_char(),
            "s",
            count,
        )
    } else {
        // pair
        format!(
            "{}{}: {}",
            Value::from_u8(value_flip(i)).to_char(),
            Value::from_u8(value_flip(j)).to_char(),
            count,
        )
    }
}

pub fn pretty_print(hands: Vec<Hand>) -> Table {
    let positions: Vec<(u8, u8)> = hands.into_par_iter().map(|h| position(h)).collect();
    let grid = add_positions(positions);

    let mut table = Table::new();

    for i in 0..13 {
        let mut row_vec: Vec<Cell> = Vec::new();
        for j in 0..13 {
            let occurrences: u8 = grid[i as usize][j as usize];
            row_vec.push(
                Cell::new(&string_from_grid(i, j, occurrences))
                    .with_style(Attr::ForegroundColor(cell_color(i, j, occurrences))),
            );
        }
        table.add_row(Row::new(row_vec));
    }

    table
}

fn position_cards(hand: Hand) -> (u8, u8, Card, Card) {
    let card1: Card = hand.cards()[0];
    let card2: Card = hand.cards()[1];
    let a = value_flip(card1.value as u8);
    let b = value_flip(card2.value as u8);

    if card1.suit == card2.suit {
        if a > b {
            (b, a, card2, card1)
        } else {
            (a, b, card1, card2)
        }
    } else {
        if a > b {
            (a, b, card1, card2)
        } else {
            (b, a, card2, card1)
        }
    }
}

fn card_grid(cards: Vec<(u8, u8, Card, Card)>) -> Vec<Vec<(String, u8)>> {
    let mut arr: Vec<Vec<(String, u8)>> = Vec::with_capacity(13);
    for _i in 0..13 {
        let mut col: Vec<(String, u8)> = Vec::with_capacity(13);
        for _j in 0..13 {
            col.push(("".to_string(), 0u8));
        }
        arr.push(col);
    }

    for (a, b, c1, c2) in cards {
        arr[a as usize][b as usize].1 += 1;
        arr[a as usize][b as usize]
            .0
            .push_str(&format!("{}, {}\n", c1, c2));
    }
    arr
}

pub fn pretty_print_cards(hands: Vec<Hand>) -> Table {
    let positions: Vec<(u8, u8, Card, Card)> =
        hands.into_par_iter().map(|h| position_cards(h)).collect();
    let grid = card_grid(positions);

    let mut table = Table::new();

    for i in 0..13 {
        let mut row_vec: Vec<Cell> = Vec::new();
        for j in 0..13 {
            let text = &grid[i as usize][j as usize].0;
            let occurrences: u8 = grid[i as usize][j as usize].1;
            row_vec.push(Cell::new(text).with_style(Attr::ForegroundColor(cell_color(
                i,
                j,
                occurrences,
            ))));
        }
        table.add_row(Row::new(row_vec));
    }

    table
}
