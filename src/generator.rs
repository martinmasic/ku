use crate::{game, game::{ Board, Cell::{ * } }};
use crate::naive_solver as solver; // TODO: replace when new solver written

use rand::seq::SliceRandom;


fn generate_candidates_matrix<T: rand::Rng>(rng: &mut T) -> [[Vec::<char>; 9]; 9] {
    std::array::from_fn(|_|
        std::array::from_fn(|_| {
            let mut cands = game::LEGAL_VALUES.to_vec();
            cands.shuffle(rng);
            cands
        })
    )
}

fn is_valid_candidate(board: &Board, cand: char, r: usize, c: usize) -> bool {
    for i in 0..r {
        match board.values[i][c] {
            Given(x) => if x == cand { return false; },
            NonGiven(x) => if x == cand { return false; },
            Empty => {},
        }
    }
    for j in 0..c {
        match board.values[r][j] {
            Given(x) => if x == cand { return false; },
            NonGiven(x) => if x == cand { return false; },
            Empty => {},
        }
    }

    let l = c - c % 3; let u = r - r % 3;
    for i in u..(u+3) {
        for j in l..(l+3) {
            match board.values[i][j] {
                Given(x) => if x == cand { return false; },
                NonGiven(x) => if x == cand { return false; },
                Empty => {},
            }
        }
    }

    true
}

pub fn generate_full_board<T: rand::Rng>(rng: &mut T) -> Board {
    let mut board = Board::zeroed();
    let mut candidates = generate_candidates_matrix(rng);

    let mut pos = 0;
    while pos < 81 {
        let r = pos / 9;
        let c = pos % 9;

        match candidates[r][c].pop() {
            Some(cand) => {
                if !is_valid_candidate(&board, cand, r, c) {
                    continue;
                }
                board.values[r][c] = Given(cand);
                pos += 1;
            },
            None => {
                candidates[r][c] = game::LEGAL_VALUES.to_vec();
                candidates[r][c].shuffle(rng);
                board.values[r][c] = Empty;
                pos -= 1;
                continue;
            }
        }
    }

    board
}

pub fn generate_valid_puzzle<T: rand::Rng>(rng: &mut T, set_givens: u8) -> Board {
    if set_givens < 17 || set_givens >= 81 {
        panic!(
            "Invalid argument given for number of givens: {set_givens}.\n\
            Number of givens should be between 17 and 80."
        );
    }

    loop {
        let mut board = generate_full_board(rng);
        // (pseudo)random order of cells to clear
        let mut positions: Vec<usize> = (0..81).collect();
        positions.shuffle(rng);

        let mut num_givens = 81;
        for i in 0..(81-17) as usize {
            let r = positions[i] / 9;
            let c = positions[i] % 9;
            let val = board.values[r][c];
            board.values[r][c] = game::Cell::Empty;

            if !solver::is_uniquely_solvable(&board) {
                board.values[r][c] = val;
            } else {
                num_givens -= 1;
                if num_givens == set_givens {
                    return board;
                }
            }
        }

        if (set_givens..=(set_givens + 2)).contains(&num_givens) {
            return board;
        }
    }
}
