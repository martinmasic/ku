use crate::{
    game,
    game::{ Board, Cell, Digit },
    utilities,
};

use std::ops::Index;
use std::ops::IndexMut;

struct CandidatesBoard<'a> {
    board: &'a mut Board,
    candidates: [[Option<Vec<Digit>>;9];9],
    first_pos: usize,
    last_pos: usize,
    current_pos: usize
}

impl Index<usize> for CandidatesBoard<'_> {
    type Output = Option<Vec<Digit>>;
    fn index(&self, pos: usize) -> &Self::Output {
        if pos >= 81 { return &None; }
        let (r, c) = utilities::coords_from_pos(pos);
        &self.candidates[r][c]
    }
}
impl IndexMut<usize> for CandidatesBoard<'_> {
    fn index_mut(&mut self, pos: usize) -> &mut Option<Vec<Digit>> {
        let (r, c) = utilities::coords_from_pos(pos);
        &mut self.candidates[r][c]
    }
}

impl<'a> CandidatesBoard<'a> {
    fn new(board: &'a mut Board) -> Self {
        let mut candidates: [[Option<Vec<Digit>>; 9]; 9]  = Default::default();
        let mut first_non_given = None; let mut last_non_given = 0;
        for i in 0..9 {
            for j in 0..9 {
                if first_non_given.is_none() && let Cell::Empty = board.at(i, j) {
                    first_non_given = Some(i * 9 + j);
                }
                if let Cell::Empty = board.at(i, j) {
                    last_non_given = i * 9 + j;
                    candidates[i][j] = Some(game::LEGAL_VALUES.into_iter().collect());
                }
            }
        }
        CandidatesBoard {
            board: board,
            candidates: candidates,
            first_pos: first_non_given.unwrap(),
            last_pos: last_non_given,
            current_pos: first_non_given.unwrap()
        }
    }

    fn is_first_nongiven(&self) -> bool { self.current_pos == self.first_pos }
    fn is_last_nongiven(&self) -> bool { self.current_pos == self.last_pos }

    fn current_cell_candidates(&mut self) -> &mut Option<Vec<Digit>> {
        let pos = self.current_pos;
        &mut self[pos]
    }

    fn pop_current_candidate(&mut self) {
        if let Some(cands) = self.current_cell_candidates() {
            cands.pop();
        }
    }

    fn current_candidate(&mut self) -> Option<Digit> {
        if let Some(cands) = self.current_cell_candidates().as_ref() {
            cands.last().cloned()
        } else {
            None
        }
    }

    fn reset_current_cell_candidates(&mut self) {
        *self.current_cell_candidates() = Some(game::LEGAL_VALUES.into_iter().collect());
    }

    fn next_non_given(&mut self) {
        for i in (self.current_pos + 1)..81 {
            if !&self[i].is_none() { self.current_pos = i; return; }
        }
    }
    fn prev_non_given(&mut self) {
        for i in (0..self.current_pos).rev() {
            if !&self[i].is_none() { self.current_pos = i; return; }
        }
    }

    fn current_candidate_valid(&self) -> bool {
        let (r, c) = utilities::coords_from_pos(self.current_pos);
        let cand = self[self.current_pos].as_ref().unwrap().last().unwrap();

        for i in 0..r {
            match self.board.at(i, c) {
                Cell::Given(x) | Cell::NonGiven(x) =>
                    if x == *cand { return false; },
                Cell::Empty => {},
            }
        }
        for j in 0..c {
            match self.board.at(r, j) {
                Cell::Given(x) | Cell::NonGiven(x) =>
                    if x == *cand { return false; },
                Cell::Empty => {},
            }
        }
        let (u, l) = utilities::square_limits_from_cell(r, c);
        for i in u..(u+3) {
            for j in l..(l+3) {
                match self.board.at(i, j) {
                    Cell::Given(x) | Cell::NonGiven(x) =>
                        if x == *cand { return false; },
                    Cell::Empty => {},
                }
            }
        }

        true
    }

    pub fn curr_cell_no_candidates(&mut self) -> bool {
        self.current_cell_candidates().as_ref().unwrap().is_empty()
    }

    pub fn generate_solution(&mut self) {
        for i in 0..9 {
            for j in 0..9 {
                if let Cell::Empty = self.board.at(i, j) {
                    let value = self.candidates[i][j]
                        .as_ref().unwrap()
                        .last().unwrap().clone();
                    self.board.set(i, j, Cell::NonGiven(value));
                }
            }
        }
    }

}

pub enum SolverResult { Invalid, Valid }

pub fn solve(board: &mut Board) -> SolverResult {
    let mut solutions_count: u64 = 0;
    let mut candidates_board = CandidatesBoard::new(board);
    // let mut solution = board.clone();
    'outer: loop {
        // current candidate is valid
        if candidates_board.current_candidate_valid() {
            if candidates_board.is_last_nongiven() {
                // found a solution!
                solutions_count += 1;
                // invalid puzzle
                if solutions_count >= 2 { return SolverResult::Invalid; }

                // set solution board
                // TODO: this is not explicit enough!!
                candidates_board.generate_solution();

                // only one non-given in puzzle (trivial case), valid solution
                if candidates_board.is_first_nongiven() {
                    return SolverResult::Valid;
                }

                // recursively reset cell candidates until
                // first available cell candidate
                loop {
                    // backtrack to previous cell
                    candidates_board.reset_current_cell_candidates();
                    candidates_board.prev_non_given();
                    candidates_board.pop_current_candidate();

                    if candidates_board.current_candidate().is_none() {
                        if candidates_board.is_first_nongiven() {
                            return SolverResult::Valid;
                        }
                    } else {
                        continue 'outer;
                    }
                }
            } else {
                candidates_board.next_non_given();
            }
        }
        // current candidate is invalid
        else {
            // backtrack to first valid candidate
            loop {
                // remove invalid candidate
                candidates_board.pop_current_candidate();

                // no valid candidates for current cell
                if candidates_board.curr_cell_no_candidates() {
                    if candidates_board.is_first_nongiven() {
                        return match solutions_count {
                            2..   => SolverResult::Invalid,
                            0..=1 => SolverResult::Valid,
                        }
                    }

                    // backtrack to previous cell
                    candidates_board.reset_current_cell_candidates();
                    candidates_board.prev_non_given();
                    candidates_board.pop_current_candidate();
                } else {
                    // valid candidates exist so continue
                    continue 'outer;
                }
            }
        }
    }
}
