use crate::game::{*};
use colored::*;

fn print_cell_value(cell: &Cell) {
    match cell {
        Cell::Given(x) => {
            print!("{}", x.to_string().yellow());
        },
        Cell::NonGiven(x) => {
            print!("{}", x.to_string().green());
        },
        Cell::Empty => {
            print!("{}", ".".white());
        }
    }
}

fn print_three_rows(board: &Board, rows: std::ops::Range<usize>) {
    for i in rows {
        print!("│ ");
        for j in 0..9 {
            print_cell_value(&board.values[i][j]);
            if j != 8 { print!(" "); }
            if j == 2 || j == 5 { print!("│ "); }
        }
        println!(" │");
    }
}

pub fn print_board(board: &Board) {
    let horizontal_separator = format!(
        "{}{}{}{}{}{}{}",
        "├".to_owned(), &"─".repeat(7),
        "┼", &"─".repeat(7),
        "┼", &"─".repeat(7), "┤"
    ) ;
    let top_border = format!(
        "{}{}{}{}{}{}{}",
        "┌".to_owned(), &"─".repeat(7),
        "┬", &"─".repeat(7),
        "┬", &"─".repeat(7), "┐"
    );

    let bot_border = format!(
        "{}{}{}{}{}{}{}",
        "└".to_owned(), &"─".repeat(7),
        "┴", &"─".repeat(7),
        "┴", &"─".repeat(7), "┘"
    );

    println!("{}", top_border);

    print_three_rows(board, 0..3);

    println!("{}", horizontal_separator);

    print_three_rows(board, 3..6);

    println!("{}", horizontal_separator);

    print_three_rows(board, 6..9);

    println!("{}", bot_border);
}

