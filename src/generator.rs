use crate::{
    game,
    game::{ Board, Cell::{*} },
    utilities,
    naive_solver,
    naive_solver::{*},
};

use rand::seq::SliceRandom;

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
                Given(x) | NonGiven(x) => if x == cand { return false; },
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
        let (r, c) = utilities::coords_from_pos(pos);

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

pub trait Generator {
    fn generate_puzzle(&self) -> game::Board;
}

pub struct NaiveGenerator {
    pub set_givens: u8
}

impl NaiveGenerator {
    pub fn new(set_givens: u8) -> NaiveGenerator {
        NaiveGenerator { set_givens }
    }
}

impl Generator for NaiveGenerator {
    fn generate_puzzle(&self) -> game::Board {
        assert!(
            (17..81).contains(&self.set_givens),
            "Invalid argument given for number of givens: {}.\n\
            Number of givens should be between 17 and 80.",
            &self.set_givens
        );

        let mut rng = rand::rng();
        let mut board = generate_full_board(&mut rng);
        'main_loop: loop {
            board = board.clone();
            // (pseudo)random order of cells to clear
            let mut positions: Vec<usize> = (0..81).collect();
            positions.shuffle(&mut rng);

            // clear cells and test if board is a valid puzzle
            // by trying to solve it
            let mut num_givens = 81;
            for i in 0..(81-17) as usize {
                let (r, c) = utilities::coords_from_pos(positions[i]);
                let val = board.values[r][c];
                board.values[r][c] = game::Cell::Empty;

                match naive_solver::solve(&mut board) {
                    SolverResult::Invalid => board.values[r][c] = val,
                    SolverResult::Valid => {
                        num_givens -= 1;
                        if num_givens == self.set_givens { break 'main_loop; }
                    }
                }
            }
        }

        let valid = evaluator::evaluate(&board);
        debug_assert_eq!(valid, true, "The generated puzzle is not valid!");

        board
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
