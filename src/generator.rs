use crate::game::{*};
use crate::testing;

use rand::seq::SliceRandom;
use std::vec;
use std::ops::Index;
use std::ops::IndexMut;


fn candidates_board<T: rand::Rng>(rng: &mut T) -> [[Vec::<Option<char>>; 9]; 9] {
    let candidates: [[Vec::<Option<char>>; 9]; 9];

    candidates = std::array::from_fn(|_|
        std::array::from_fn(|_| {
            let mut cands = crate::game::LEGAL_VALUES.to_vec();
            cands.shuffle(rng);
            cands
        })
    );

    candidates
}

fn is_valid_candidate(board: &Board, cand: Option<char>, r: usize, c: usize) -> bool {
    for i in 0..r {
        if board.values[i][c] == cand {
            return false;
        }
    }
    for j in 0..c {
        if board.values[r][j] == cand {
            return false;
        }
    }

    let l = c - c % 3; let u = r - r % 3;
    for i in u..(u+3) {
        for j in l..(l+3) {
            if board.values[i][j] == cand {
                return false;
            }
        }
    }

    true
}

pub fn generate_full_board<T: rand::Rng>(rng: &mut T) -> Board {
    let mut board = Board::zeroed();
    let mut candidates = candidates_board(rng);

    let mut pos = 0;
    while pos < 81 {
        let r = pos / 9;
        let c = pos % 9;

        match candidates[r][c].pop() {
            Some(cand) => {
                if !is_valid_candidate(&board, cand, r, c) {
                    continue;
                }

                board.values[r][c] = cand;
                pos += 1;
            },
            None => {
                candidates[r][c] = crate::game::LEGAL_VALUES.to_vec();
                candidates[r][c].shuffle(rng);
                board.values[r][c] = None;
                pos -= 1;
                continue;
            }
        }
    }

    board
}

struct CandidatesBoard<'a> {
    board: &'a Board,
    candidates: [[Option<Vec<char>>;9];9],
    first_pos: usize,
    last_pos: usize,
    current_pos: usize
}

impl Index<usize> for CandidatesBoard<'_> {
    type Output = Option<Vec<char>>;
    fn index(&self, pos: usize) -> &Self::Output {
        if pos >= 81 { return &None; }
        &self.candidates[pos / 9][pos % 9]
    }
}
impl IndexMut<usize> for CandidatesBoard<'_> {
    fn index_mut(&mut self, pos: usize) -> &mut Option<Vec<char>> {
        &mut self.candidates[pos / 9][pos % 9]
    }
}
impl Index<(usize,usize)> for CandidatesBoard<'_> {
    type Output = Option<Vec<char>>;
    fn index(&self, xy: (usize,usize)) -> &Self::Output {
        &self.candidates[xy.0][xy.1]
    }
}
impl IndexMut<(usize,usize)> for CandidatesBoard<'_> {
    fn index_mut(&mut self, xy: (usize,usize)) -> &mut Option<Vec<char>> {
        &mut self.candidates[xy.0][xy.1]
    }
}
impl<'a> CandidatesBoard<'a> {
    fn new(board: &'a Board) -> Self {
        let mut candidates: [[Option<Vec<char>>; 9]; 9]  = Default::default();
        let mut first = None; let mut last = 0;
        for i in 0..9 {
            for j in 0..9 {
                if first.is_none() && board.values[i][j].is_none() {
                    first = Some(i * 9 + j);
                }
                if board.values[i][j].is_none() {
                    last = i * 9 + j;
                    candidates[i][j] = Some(vec!['1','2','3','4','5','6','7','8','9']);
                }
            }
        }
        CandidatesBoard {
            board: board,
            candidates: candidates,
            first_pos: first.unwrap(),
            last_pos: last,
            current_pos: first.unwrap()
        }
    }

    fn is_first(&self) -> bool { self.current_pos == self.first_pos }
    fn is_last(&self) -> bool { self.current_pos == self.last_pos }

    fn curr(&mut self) -> &mut Option<Vec<char>> {
        let pos = self.current_pos;
        &mut self[pos]
    }
    fn pop_candidate(&mut self) {
        if let Some(cands) = self.curr() {
            cands.pop();
        }
    }

    fn reset_curr_cands(&mut self) {
        *self.curr() = Some(vec!['1','2','3','4','5','6','7','8','9']);
    }

    fn inc(&mut self) {
        for i in (self.current_pos + 1)..81 {
            let cands = &self[i];
            if !cands.is_none() { self.current_pos = i; return; }
        }
    }
    fn dec(&mut self) {
        for i in (0..self.current_pos).rev() {
            let cands = &self[i];
            if !cands.is_none() { self.current_pos = i; return; }
        }
    }

    fn curr_cand_valid(&self) -> bool {
        let r = self.current_pos / 9; let c = self.current_pos % 9;
        let cand = self[self.current_pos].as_ref().unwrap().last().unwrap();

        for i in 0..r {
            if self.board.values[i][c] == Some(*cand) {
                return false;
            }
        }
        for j in 0..c {
            if self.board.values[r][j] == Some(*cand) {
                return false;
            }
        }

        let l = c - c % 3; let u = r - r % 3;
        for i in u..(u+3) {
            for j in l..(l+3) {
                if self.board.values[i][j] == Some(*cand) {
                    return false;
                }
            }
        }

        true
    }
}


fn solutions_count(board: &Board) -> u64 {
    let mut solutions_count: u64 = 0;
    let mut cands_board = CandidatesBoard::new(&board);
    'outer: loop {
        if cands_board.curr_cand_valid() {
            if cands_board.is_last() {
                solutions_count += 1;
                if cands_board.is_first() { return solutions_count; } // ??

                'inner: loop {
                    cands_board.reset_curr_cands();
                    cands_board.dec();
                    cands_board.pop_candidate();

                    if cands_board.curr().as_ref().unwrap().is_empty() {
                        if cands_board.is_first() {
                            return solutions_count;
                        } else { continue 'inner; }
                    } else { continue 'outer; }
                }
            } else {
                cands_board.inc();
                continue;
            }
        } else {
            'inner: loop {
                if cands_board.is_first()
                    && cands_board.curr().as_ref().unwrap().is_empty() {
                        return solutions_count;
                    }

                cands_board.pop_candidate();
                if cands_board.curr().as_ref().unwrap().is_empty() {
                    if cands_board.is_first() {
                        return solutions_count;
                    }
                    cands_board.reset_curr_cands();
                    cands_board.dec();
                    cands_board.pop_candidate();
                    continue 'inner;
                } else {
                    continue 'outer;
                }
            }
        }
    }

    solutions_count
}


fn is_uniquely_solvable(board: &Board) -> bool {
    solutions_count(board) == 1
}

pub fn generate_valid_board<T: rand::Rng>(rng: &mut T, num_givens: u8) -> Board {
    if num_givens < 17 || num_givens >= 81 {
        panic!("Invalid argument given for number of givens: {num_givens}.");
    }

    while true {
        let mut board = generate_full_board(rng);
        // (pseudo)random order of cells to clear
        let mut positions: Vec<usize> = (0..81).collect();
        positions.shuffle(rng);

        let mut remaining = 81;
        for i in 0..(81-17) as usize {
            let r = positions[i] / 9;
            let c = positions[i] % 9;

            let val = board.values[r][c];
            board.values[r][c] = None;
            board.is_given[r][c] = false;

            if !is_uniquely_solvable(&board) {
                board.values[r][c] = val;
                board.is_given[r][c] = true;
                continue;
            }

            remaining -= 1;
            if remaining == num_givens {
                return board;
            }
        }

        if num_givens <= remaining && remaining <= num_givens + 2 {
            return board;
        }
    }
    return Board::zeroed();
}
