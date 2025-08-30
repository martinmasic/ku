use crate::{game::{ Board, Cell::{ * } }, utilities};
use std::collections::HashSet;

pub fn evaluate(board: &Board) -> bool {
    // evaluating every row
    for i in 0..9 {
        let mut set = HashSet::new();
        for j in 0..9 {
            match board.values[i][j] {
                Given(x) => if !set.insert(x) { return false; },
                NonGiven(x) => if !set.insert(x) { return false; },
                Empty => { return false; },
            }
            if set.len() != j + 1 { return false; }
        }
    }

    // evaluating every column
    for j in 0..9 {
        let mut set = HashSet::new();
        for i in 0..9 {
            match board.values[i][j] {
                Given(x) => if !set.insert(x) { return false; },
                NonGiven(x) => if !set.insert(x) { return false; },
                Empty => { return false; },
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
                match board.values[i][j] {
                    Given(x) => if !set.insert(x) { return false; },
                    NonGiven(x) => if !set.insert(x) { return false; },
                    Empty => { return false; },
                }
            }
        }

    }

    true
}

// TODO: test evaluate() on hardcoded boards
