use prettytable::{Attr, Cell, Row, Table};
use rayon::prelude::*;
use rs_poker::core::*;

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
    let a = card1.value as u8;
    let b = card2.value as u8;

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
            Value::from_u8(i).to_char(),
            Value::from_u8(j).to_char(),
            "o",
            count,
        )
    } else if i < j {
        // suited
        format!(
            "{}{}{}: {}",
            Value::from_u8(j).to_char(),
            Value::from_u8(i).to_char(),
            "s",
            count,
        )
    } else {
        // pair
        format!(
            "{}{}: {}",
            Value::from_u8(i).to_char(),
            Value::from_u8(j).to_char(),
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
