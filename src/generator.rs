use crate::{
    game,
    game::{ Board, Cell::{*} },
    utilities,
    naive_solver,
    solver::{*}
};

use rand::seq::SliceRandom;
use rand::rngs::ThreadRng;

use std::thread;
use crossbeam::channel;

use crate::evaluator;


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
            Given(x) | NonGiven(x) => if x == cand { return false; },
            Empty => {},
        }
    }
    for j in 0..c {
        match board.values[r][j] {
            Given(x) | NonGiven(x) => if x == cand { return false; },
            Empty => {},
        }
    }

    let (u, l) = utilities::square_limits_from_cell(r, c);
    for i in u..(u+3) {
        for j in l..(l+3) {
            match board.values[i][j] {
                Given(x) | NonGiven(x) => { if x == cand { return false; } },
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
                if !is_valid_candidate(&board, cand, r, c) { continue; }
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

pub struct NaiveGenerator {
    pub set_givens: u8
}

impl NaiveGenerator {
    pub fn new(set_givens: u8) -> NaiveGenerator {
        NaiveGenerator { set_givens }
    }

    pub fn generate_puzzle(&self) -> game::Board {
        assert!(
            (17..81).contains(&self.set_givens),
            "Invalid argument given for number of givens: {}.\n\
            Number of givens should be between 17 and 80.",
            &self.set_givens
        );
        use std::sync::{Arc, atomic::{AtomicBool, Ordering}};

        let (tx, rx) = channel::unbounded();
        let done = std::sync::Arc::new(std::sync::atomic::AtomicBool::new(false));

        let work = |
            rng: &mut ThreadRng,
            done: &Arc<AtomicBool>,
            set_givens: u8
        | -> Result<Board, &'static str> {
            let done = done.clone();
            // (pseudo-)randomly generated full board
            let mut board = generate_full_board(rng);
            while !done.load(Ordering::Relaxed) {
                board = board.clone();
                // (pseudo)random order of cells to clear
                let mut positions: Vec<usize> = (0..81).collect();
                positions.shuffle(rng);

                // clear cells and test if board is a valid puzzle
                // by trying to solve it
                let mut num_givens = 81;
                for i in 0..(81-17) as usize {
                    if done.load(Ordering::Relaxed) {
                        return Err("Thread killed!");
                    }

                    let r = positions[i] / 9;
                    let c = positions[i] % 9;
                    let val = board.values[r][c];
                    board.values[r][c] = game::Cell::Empty;

                    match naive_solver::solve(&mut board) {
                        SolverResult::Invalid => board.values[r][c] = val,
                        SolverResult::Valid => {
                            num_givens -= 1;
                            if num_givens == set_givens { return Ok(board); }
                        }
                    }
                }

                if (set_givens..=(set_givens + 2)).contains(&num_givens) {
                    if let SolverResult::Valid = naive_solver::solve(&mut board) {
                        return Ok(board);
                    }
                }
            }

            Err("Thread killed!")
        };

        let set_givens = self.set_givens;
        for _ in 0..1 {
            let tx = tx.clone();
            let done = Arc::clone(&done);
            thread::spawn(move || {
                let mut trng = rand::rng();
                let result = work(&mut trng, &done, set_givens);

                if let Ok(result) = result {
                    done.store(true, Ordering::Relaxed);
                    let _ = tx.send(result).unwrap();
                }
            });
        }

        let result = rx.recv().unwrap();
        let valid = evaluator::evaluate(&result);
        assert_eq!(valid, true, "The generated puzzle is not valid!");

        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generating_full_board() {
        use crate::cli_display;

        for _ in 0..9 {
            let mut trng = rand::rng();
            let board = generate_full_board(&mut trng);
            let valid = evaluator::evaluate(&board);

            if !valid { cli_display::print_board(&board); }
            assert_eq!(valid, true);
        }

    }
}
