use std::collections::HashSet;

use crate::{game::{ Board, Cell, Digit::{*} }, utilities};

pub fn evaluate(board: &Board) -> bool {
    // evaluating every row
    for i in 0..9 {
        let mut set = HashSet::new();
        for j in 0..9 {
            match board.at(i, j) {
                Cell::Given(x) => if !set.insert(x) {
                    return false;
                },
                Cell::NonGiven(x) => if !set.insert(x) {
                    return false;
                },
                Cell::Empty => { return false; },
            }
            if set.len() != j + 1 { return false; }
        }
    }

    // evaluating every column
    for j in 0..9 {
        let mut set = HashSet::new();
        for i in 0..9 {
            match board.at(i, j) {
                Cell::Given(x) => if !set.insert(x) {
                    return false;
                },
                Cell::NonGiven(x) => if !set.insert(x) {
                    return false;
                },
                Cell::Empty => { return false; },
            }
            if set.len() != i + 1 { return false; }
        }
    }

    // evaluating every square
    for s in 0..9 {
        let (u, l) = utilities::square_limits_from_square(s);
        let mut set = HashSet::new();
        for i in u..=(u+2) {
            for j in l..=(l+2) {
                match board.at(i, j) {
                    Cell::Given(x) => if !set.insert(x) {
                        return false;
                    },
                    Cell::NonGiven(x) => if !set.insert(x) {
                        return false;
                    },
                    Cell::Empty => { return false; },
                }
            }
        }

    }

    true
}

#[cfg(test)]
mod evaluator_tests {
    use super::*;
    use crate::cli_display;

    #[test]
    fn test_evaluating_valid_board1() {
        let board = Board::new([
            [Cell::Given(D1), Cell::Given(D2), Cell::Given(D3), Cell::Given(D4), Cell::Given(D5), Cell::Given(D6), Cell::Given(D7), Cell::Given(D8), Cell::Given(D9)],
            [Cell::Given(D4), Cell::Given(D5), Cell::Given(D6), Cell::Given(D7), Cell::Given(D8), Cell::Given(D9), Cell::Given(D1), Cell::Given(D2), Cell::Given(D3)],
            [Cell::Given(D7), Cell::Given(D8), Cell::Given(D9), Cell::Given(D1), Cell::Given(D2), Cell::Given(D3), Cell::Given(D4), Cell::Given(D5), Cell::Given(D6)],
            [Cell::Given(D2), Cell::Given(D3), Cell::Given(D4), Cell::Given(D5), Cell::Given(D6), Cell::Given(D7), Cell::Given(D8), Cell::Given(D9), Cell::Given(D1)],
            [Cell::Given(D5), Cell::Given(D6), Cell::Given(D7), Cell::Given(D8), Cell::Given(D9), Cell::Given(D1), Cell::Given(D2), Cell::Given(D3), Cell::Given(D4)],
            [Cell::Given(D8), Cell::Given(D9), Cell::Given(D1), Cell::Given(D2), Cell::Given(D3), Cell::Given(D4), Cell::Given(D5), Cell::Given(D6), Cell::Given(D7)],
            [Cell::Given(D3), Cell::Given(D4), Cell::Given(D5), Cell::Given(D6), Cell::Given(D7), Cell::Given(D8), Cell::Given(D9), Cell::Given(D1), Cell::Given(D2)],
            [Cell::Given(D6), Cell::Given(D7), Cell::Given(D8), Cell::Given(D9), Cell::Given(D1), Cell::Given(D2), Cell::Given(D3), Cell::Given(D4), Cell::Given(D5)],
            [Cell::Given(D9), Cell::Given(D1), Cell::Given(D2), Cell::Given(D3), Cell::Given(D4), Cell::Given(D5), Cell::Given(D6), Cell::Given(D7), Cell::Given(D8)],
        ]);
        let valid = evaluate(&board);
        if !valid { cli_display::print_board(&board); }
        assert!(valid);
    }

    #[test]
    fn test_evaluating_valid_board2() {
        let board = Board::new([
            [Cell::NonGiven(D1), Cell::Given(D2), Cell::Given(D3), Cell::Given(D4), Cell::Given(D5), Cell::Given(D6), Cell::Given(D7), Cell::Given(D8), Cell::Given(D9)],
            [Cell::Given(D4), Cell::NonGiven(D5), Cell::Given(D6), Cell::Given(D7), Cell::Given(D8), Cell::Given(D9), Cell::Given(D1), Cell::Given(D2), Cell::Given(D3)],
            [Cell::Given(D7), Cell::Given(D8), Cell::NonGiven(D9), Cell::Given(D1), Cell::Given(D2), Cell::Given(D3), Cell::Given(D4), Cell::Given(D5), Cell::Given(D6)],
            [Cell::Given(D2), Cell::Given(D3), Cell::Given(D4), Cell::NonGiven(D5), Cell::NonGiven(D6), Cell::Given(D7), Cell::Given(D8), Cell::Given(D9), Cell::Given(D1)],
            [Cell::Given(D5), Cell::Given(D6), Cell::Given(D7), Cell::Given(D8), Cell::Given(D9), Cell::Given(D1), Cell::Given(D2), Cell::Given(D3), Cell::Given(D4)],
            [Cell::Given(D8), Cell::Given(D9), Cell::Given(D1), Cell::Given(D2), Cell::Given(D3), Cell::Given(D4), Cell::Given(D5), Cell::Given(D6), Cell::Given(D7)],
            [Cell::Given(D3), Cell::Given(D4), Cell::Given(D5), Cell::Given(D6), Cell::Given(D7), Cell::Given(D8), Cell::Given(D9), Cell::Given(D1), Cell::Given(D2)],
            [Cell::Given(D6), Cell::Given(D7), Cell::Given(D8), Cell::Given(D9), Cell::Given(D1), Cell::Given(D2), Cell::Given(D3), Cell::Given(D4), Cell::Given(D5)],
            [Cell::Given(D9), Cell::Given(D1), Cell::Given(D2), Cell::Given(D3), Cell::Given(D4), Cell::Given(D5), Cell::Given(D6), Cell::Given(D7), Cell::Given(D8)],
        ]);
        let valid = evaluate(&board);
        if !valid { cli_display::print_board(&board); }
        assert!(valid);
    }


    #[test]
    fn test_evaluating_valid_board3() {
        let board = Board::new([
            [Cell::Given(D8), Cell::Given(D7), Cell::Given(D2), Cell::Given(D4), Cell::Given(D1), Cell::Given(D5), Cell::Given(D3), Cell::Given(D6), Cell::Given(D9)],
            [Cell::Given(D6), Cell::Given(D9), Cell::Given(D1), Cell::Given(D7), Cell::Given(D8), Cell::Given(D3), Cell::Given(D5), Cell::Given(D4), Cell::Given(D2)],
            [Cell::Given(D4), Cell::Given(D3), Cell::Given(D5), Cell::Given(D9), Cell::Given(D2), Cell::Given(D6), Cell::Given(D7), Cell::Given(D8), Cell::Given(D1)],
            [Cell::Given(D7), Cell::Given(D6), Cell::Given(D3), Cell::Given(D2), Cell::Given(D5), Cell::Given(D8), Cell::Given(D1), Cell::Given(D9), Cell::Given(D4)],
            [Cell::Given(D2), Cell::Given(D5), Cell::Given(D8), Cell::Given(D1), Cell::Given(D9), Cell::Given(D4), Cell::Given(D6), Cell::Given(D3), Cell::Given(D7)],
            [Cell::Given(D1), Cell::Given(D4), Cell::Given(D9), Cell::Given(D6), Cell::Given(D3), Cell::Given(D7), Cell::Given(D2), Cell::Given(D5), Cell::Given(D8)],
            [Cell::Given(D3), Cell::Given(D8), Cell::Given(D7), Cell::Given(D5), Cell::Given(D4), Cell::Given(D2), Cell::Given(D9), Cell::Given(D1), Cell::Given(D6)],
            [Cell::Given(D5), Cell::Given(D1), Cell::Given(D6), Cell::Given(D8), Cell::Given(D7), Cell::Given(D9), Cell::Given(D4), Cell::Given(D2), Cell::Given(D3)],
            [Cell::Given(D9), Cell::Given(D2), Cell::Given(D4), Cell::Given(D3), Cell::Given(D6), Cell::Given(D1), Cell::Given(D8), Cell::Given(D7), Cell::Given(D5)],
        ]);
        let valid = evaluate(&board);
        if !valid { cli_display::print_board(&board); }
        assert!(valid);
    }

    #[test]
    fn test_evaluating_invalid_board1() {
        let board = Board::new([
            [Cell::Given(D1), Cell::Given(D7), Cell::Given(D2), Cell::Given(D4), Cell::Given(D1), Cell::Given(D5), Cell::Given(D3), Cell::Given(D6), Cell::Given(D9)],
            [Cell::Given(D6), Cell::NonGiven(D9), Cell::Given(D1), Cell::Given(D7), Cell::Given(D8), Cell::Given(D3), Cell::Given(D5), Cell::Given(D4), Cell::Given(D2)],
            [Cell::Given(D4), Cell::Given(D3), Cell::Given(D5), Cell::Given(D9), Cell::Given(D2), Cell::Given(D6), Cell::Given(D7), Cell::Given(D8), Cell::Given(D1)],
            [Cell::Given(D7), Cell::Given(D6), Cell::Given(D3), Cell::Given(D2), Cell::Given(D5), Cell::Given(D8), Cell::Given(D1), Cell::Given(D9), Cell::Given(D4)],
            [Cell::Given(D2), Cell::Given(D5), Cell::Given(D8), Cell::Given(D1), Cell::Given(D9), Cell::Given(D4), Cell::Given(D6), Cell::Given(D3), Cell::Given(D7)],
            [Cell::Given(D1), Cell::Given(D4), Cell::NonGiven(D9), Cell::Given(D6), Cell::Given(D3), Cell::Given(D7), Cell::Given(D2), Cell::Given(D5), Cell::Given(D8)],
            [Cell::Given(D3), Cell::Given(D8), Cell::Given(D7), Cell::Given(D5), Cell::Given(D4), Cell::Given(D2), Cell::Given(D9), Cell::Given(D1), Cell::Given(D6)],
            [Cell::Given(D5), Cell::Given(D1), Cell::Given(D6), Cell::Given(D8), Cell::Given(D7), Cell::Given(D9), Cell::Given(D4), Cell::Given(D2), Cell::Given(D3)],
            [Cell::Given(D9), Cell::Given(D2), Cell::Given(D4), Cell::Given(D3), Cell::Given(D6), Cell::Given(D1), Cell::Given(D8), Cell::Given(D7), Cell::Given(D5)],
        ]);
        let valid = evaluate(&board);
        if valid { cli_display::print_board(&board); }
        assert!(!valid);
    }

    #[test]
    fn test_evaluating_invalid_board2() {
        let board = Board::new([
            [Cell::Empty, Cell::Given(D7), Cell::Given(D2), Cell::Given(D4), Cell::Given(D1), Cell::Given(D5), Cell::Given(D3), Cell::Given(D6), Cell::Given(D9)],
            [Cell::Given(D6), Cell::Given(D9), Cell::Given(D1), Cell::Given(D7), Cell::Given(D8), Cell::Given(D3), Cell::Given(D5), Cell::Given(D4), Cell::Given(D2)],
            [Cell::Given(D4), Cell::Given(D3), Cell::Given(D5), Cell::Given(D9), Cell::Given(D2), Cell::Given(D6), Cell::Given(D7), Cell::Given(D8), Cell::Given(D1)],
            [Cell::Given(D7), Cell::Given(D6), Cell::Given(D3), Cell::Given(D2), Cell::Given(D5), Cell::Given(D8), Cell::Given(D1), Cell::Given(D9), Cell::Given(D4)],
            [Cell::Given(D2), Cell::Given(D5), Cell::Given(D8), Cell::Given(D1), Cell::Given(D9), Cell::Given(D4), Cell::Given(D6), Cell::Given(D3), Cell::Given(D7)],
            [Cell::Given(D1), Cell::Given(D4), Cell::NonGiven(D9), Cell::Given(D6), Cell::Given(D3), Cell::Given(D7), Cell::Given(D2), Cell::Given(D5), Cell::Given(D8)],
            [Cell::Given(D3), Cell::Given(D8), Cell::Given(D7), Cell::Given(D5), Cell::Given(D4), Cell::Given(D2), Cell::Given(D9), Cell::Given(D1), Cell::Given(D6)],
            [Cell::Given(D5), Cell::Empty, Cell::Given(D6), Cell::Given(D8), Cell::Given(D7), Cell::Given(D9), Cell::Given(D1), Cell::Given(D2), Cell::Given(D3)],
            [Cell::Given(D9), Cell::Given(D2), Cell::Given(D4), Cell::Given(D3), Cell::Given(D6), Cell::Given(D1), Cell::Given(D8), Cell::Given(D7), Cell::Given(D5)],
        ]);
        let valid = evaluate(&board);
        if valid { cli_display::print_board(&board); }
        assert!(!valid);
    }

}
