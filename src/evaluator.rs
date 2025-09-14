use std::collections::HashSet;

use crate::{game::{ Board, Cell }, utilities, evaluator};

pub fn evaluate(board: &Board) -> bool {
    // evaluating every row
    for i in 0..9 {
        let mut set = HashSet::new();
        for j in 0..9 {
            match board.at(i, j) {
                Cell::Given(x) => if !('0'..='9').contains(&x) || !set.insert(x) {
                    return false;
                },
                Cell::NonGiven(x) => if !('0'..='9').contains(&x) || !set.insert(x) {
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
                Cell::Given(x) => if !('0'..='9').contains(&x) || !set.insert(x) {
                    return false;
                },
                Cell::NonGiven(x) => if !('0'..='9').contains(&x) || !set.insert(x) {
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
                    Cell::Given(x) => if !('0'..='9').contains(&x) || !set.insert(x) {
                        return false;
                    },
                    Cell::NonGiven(x) => if !('0'..='9').contains(&x) || !set.insert(x) {
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
            [Cell::Given('1'), Cell::Given('2'), Cell::Given('3'), Cell::Given('4'), Cell::Given('5'), Cell::Given('6'), Cell::Given('7'), Cell::Given('8'), Cell::Given('9')],
            [Cell::Given('4'), Cell::Given('5'), Cell::Given('6'), Cell::Given('7'), Cell::Given('8'), Cell::Given('9'), Cell::Given('1'), Cell::Given('2'), Cell::Given('3')],
            [Cell::Given('7'), Cell::Given('8'), Cell::Given('9'), Cell::Given('1'), Cell::Given('2'), Cell::Given('3'), Cell::Given('4'), Cell::Given('5'), Cell::Given('6')],
            [Cell::Given('2'), Cell::Given('3'), Cell::Given('4'), Cell::Given('5'), Cell::Given('6'), Cell::Given('7'), Cell::Given('8'), Cell::Given('9'), Cell::Given('1')],
            [Cell::Given('5'), Cell::Given('6'), Cell::Given('7'), Cell::Given('8'), Cell::Given('9'), Cell::Given('1'), Cell::Given('2'), Cell::Given('3'), Cell::Given('4')],
            [Cell::Given('8'), Cell::Given('9'), Cell::Given('1'), Cell::Given('2'), Cell::Given('3'), Cell::Given('4'), Cell::Given('5'), Cell::Given('6'), Cell::Given('7')],
            [Cell::Given('3'), Cell::Given('4'), Cell::Given('5'), Cell::Given('6'), Cell::Given('7'), Cell::Given('8'), Cell::Given('9'), Cell::Given('1'), Cell::Given('2')],
            [Cell::Given('6'), Cell::Given('7'), Cell::Given('8'), Cell::Given('9'), Cell::Given('1'), Cell::Given('2'), Cell::Given('3'), Cell::Given('4'), Cell::Given('5')],
            [Cell::Given('9'), Cell::Given('1'), Cell::Given('2'), Cell::Given('3'), Cell::Given('4'), Cell::Given('5'), Cell::Given('6'), Cell::Given('7'), Cell::Given('8')],
        ]);
        let valid = evaluate(&board);
        if !valid { cli_display::print_board(&board); }
        assert!(valid);
    }

    #[test]
    fn test_evaluating_valid_board2() {
        let board = Board::new([
            [Cell::NonGiven('1'), Cell::Given('2'), Cell::Given('3'), Cell::Given('4'), Cell::Given('5'), Cell::Given('6'), Cell::Given('7'), Cell::Given('8'), Cell::Given('9')],
            [Cell::Given('4'), Cell::NonGiven('5'), Cell::Given('6'), Cell::Given('7'), Cell::Given('8'), Cell::Given('9'), Cell::Given('1'), Cell::Given('2'), Cell::Given('3')],
            [Cell::Given('7'), Cell::Given('8'), Cell::NonGiven('9'), Cell::Given('1'), Cell::Given('2'), Cell::Given('3'), Cell::Given('4'), Cell::Given('5'), Cell::Given('6')],
            [Cell::Given('2'), Cell::Given('3'), Cell::Given('4'), Cell::NonGiven('5'), Cell::NonGiven('6'), Cell::Given('7'), Cell::Given('8'), Cell::Given('9'), Cell::Given('1')],
            [Cell::Given('5'), Cell::Given('6'), Cell::Given('7'), Cell::Given('8'), Cell::Given('9'), Cell::Given('1'), Cell::Given('2'), Cell::Given('3'), Cell::Given('4')],
            [Cell::Given('8'), Cell::Given('9'), Cell::Given('1'), Cell::Given('2'), Cell::Given('3'), Cell::Given('4'), Cell::Given('5'), Cell::Given('6'), Cell::Given('7')],
            [Cell::Given('3'), Cell::Given('4'), Cell::Given('5'), Cell::Given('6'), Cell::Given('7'), Cell::Given('8'), Cell::Given('9'), Cell::Given('1'), Cell::Given('2')],
            [Cell::Given('6'), Cell::Given('7'), Cell::Given('8'), Cell::Given('9'), Cell::Given('1'), Cell::Given('2'), Cell::Given('3'), Cell::Given('4'), Cell::Given('5')],
            [Cell::Given('9'), Cell::Given('1'), Cell::Given('2'), Cell::Given('3'), Cell::Given('4'), Cell::Given('5'), Cell::Given('6'), Cell::Given('7'), Cell::Given('8')],
        ]);
        let valid = evaluate(&board);
        if !valid { cli_display::print_board(&board); }
        assert!(valid);
    }


    #[test]
    fn test_evaluating_valid_board3() {
        let board = Board::new([
            [Cell::Given('8'), Cell::Given('7'), Cell::Given('2'), Cell::Given('4'), Cell::Given('1'), Cell::Given('5'), Cell::Given('3'), Cell::Given('6'), Cell::Given('9')],
            [Cell::Given('6'), Cell::Given('9'), Cell::Given('1'), Cell::Given('7'), Cell::Given('8'), Cell::Given('3'), Cell::Given('5'), Cell::Given('4'), Cell::Given('2')],
            [Cell::Given('4'), Cell::Given('3'), Cell::Given('5'), Cell::Given('9'), Cell::Given('2'), Cell::Given('6'), Cell::Given('7'), Cell::Given('8'), Cell::Given('1')],
            [Cell::Given('7'), Cell::Given('6'), Cell::Given('3'), Cell::Given('2'), Cell::Given('5'), Cell::Given('8'), Cell::Given('1'), Cell::Given('9'), Cell::Given('4')],
            [Cell::Given('2'), Cell::Given('5'), Cell::Given('8'), Cell::Given('1'), Cell::Given('9'), Cell::Given('4'), Cell::Given('6'), Cell::Given('3'), Cell::Given('7')],
            [Cell::Given('1'), Cell::Given('4'), Cell::Given('9'), Cell::Given('6'), Cell::Given('3'), Cell::Given('7'), Cell::Given('2'), Cell::Given('5'), Cell::Given('8')],
            [Cell::Given('3'), Cell::Given('8'), Cell::Given('7'), Cell::Given('5'), Cell::Given('4'), Cell::Given('2'), Cell::Given('9'), Cell::Given('1'), Cell::Given('6')],
            [Cell::Given('5'), Cell::Given('1'), Cell::Given('6'), Cell::Given('8'), Cell::Given('7'), Cell::Given('9'), Cell::Given('4'), Cell::Given('2'), Cell::Given('3')],
            [Cell::Given('9'), Cell::Given('2'), Cell::Given('4'), Cell::Given('3'), Cell::Given('6'), Cell::Given('1'), Cell::Given('8'), Cell::Given('7'), Cell::Given('5')],
        ]);
        let valid = evaluate(&board);
        if !valid { cli_display::print_board(&board); }
        assert!(valid);
    }

    #[test]
    fn test_evaluating_invalid_board1() {
        let board = Board::new([
            [Cell::Given('1'), Cell::Given('7'), Cell::Given('2'), Cell::Given('4'), Cell::Given('1'), Cell::Given('5'), Cell::Given('3'), Cell::Given('6'), Cell::Given('9')],
            [Cell::Given('6'), Cell::NonGiven('9'), Cell::Given('1'), Cell::Given('7'), Cell::Given('8'), Cell::Given('3'), Cell::Given('5'), Cell::Given('4'), Cell::Given('2')],
            [Cell::Given('4'), Cell::Given('3'), Cell::Given('5'), Cell::Given('9'), Cell::Given('2'), Cell::Given('6'), Cell::Given('7'), Cell::Given('8'), Cell::Given('1')],
            [Cell::Given('7'), Cell::Given('6'), Cell::Given('3'), Cell::Given('2'), Cell::Given('5'), Cell::Given('8'), Cell::Given('1'), Cell::Given('9'), Cell::Given('4')],
            [Cell::Given('2'), Cell::Given('5'), Cell::Given('8'), Cell::Given('1'), Cell::Given('9'), Cell::Given('4'), Cell::Given('6'), Cell::Given('3'), Cell::Given('7')],
            [Cell::Given('1'), Cell::Given('4'), Cell::NonGiven('9'), Cell::Given('6'), Cell::Given('3'), Cell::Given('7'), Cell::Given('2'), Cell::Given('5'), Cell::Given('8')],
            [Cell::Given('3'), Cell::Given('8'), Cell::Given('7'), Cell::Given('5'), Cell::Given('4'), Cell::Given('2'), Cell::Given('9'), Cell::Given('1'), Cell::Given('6')],
            [Cell::Given('5'), Cell::Given('1'), Cell::Given('6'), Cell::Given('8'), Cell::Given('7'), Cell::Given('9'), Cell::Given('4'), Cell::Given('2'), Cell::Given('3')],
            [Cell::Given('9'), Cell::Given('2'), Cell::Given('4'), Cell::Given('3'), Cell::Given('6'), Cell::Given('1'), Cell::Given('8'), Cell::Given('7'), Cell::Given('5')],
        ]);
        let valid = evaluate(&board);
        if valid { cli_display::print_board(&board); }
        assert!(!valid);
    }

    #[test]
    fn test_evaluating_invalid_board2() {
        let board = Board::new([
            [Cell::Empty, Cell::Given('7'), Cell::Given('2'), Cell::Given('4'), Cell::Given('1'), Cell::Given('5'), Cell::Given('3'), Cell::Given('6'), Cell::Given('9')],
            [Cell::Given('6'), Cell::Given('9'), Cell::Given('1'), Cell::Given('7'), Cell::Given('8'), Cell::Given('3'), Cell::Given('5'), Cell::Given('4'), Cell::Given('2')],
            [Cell::Given('4'), Cell::Given('3'), Cell::Given('5'), Cell::Given('9'), Cell::Given('2'), Cell::Given('6'), Cell::Given('7'), Cell::Given('8'), Cell::Given('1')],
            [Cell::Given('7'), Cell::Given('6'), Cell::Given('3'), Cell::Given('2'), Cell::Given('5'), Cell::Given('8'), Cell::Given('1'), Cell::Given('9'), Cell::Given('4')],
            [Cell::Given('2'), Cell::Given('5'), Cell::Given('8'), Cell::Given('1'), Cell::Given('9'), Cell::Given('4'), Cell::Given('6'), Cell::Given('3'), Cell::Given('7')],
            [Cell::Given('1'), Cell::Given('4'), Cell::NonGiven('9'), Cell::Given('6'), Cell::Given('3'), Cell::Given('7'), Cell::Given('2'), Cell::Given('5'), Cell::Given('8')],
            [Cell::Given('3'), Cell::Given('8'), Cell::Given('7'), Cell::Given('5'), Cell::Given('4'), Cell::Given('2'), Cell::Given('9'), Cell::Given('1'), Cell::Given('6')],
            [Cell::Given('5'), Cell::Empty, Cell::Given('6'), Cell::Given('8'), Cell::Given('7'), Cell::Given('9'), Cell::Given('0'), Cell::Given('2'), Cell::Given('3')],
            [Cell::Given('9'), Cell::Given('2'), Cell::Given('4'), Cell::Given('3'), Cell::Given('6'), Cell::Given('1'), Cell::Given('8'), Cell::Given('7'), Cell::Given('a')],
        ]);
        let valid = evaluate(&board);
        if valid { cli_display::print_board(&board); }
        assert!(!valid);
    }


}
