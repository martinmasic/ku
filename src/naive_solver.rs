use crate::game::{ *, Cell::{ * } };

use std::ops::Index;
use std::ops::IndexMut;

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
                if first.is_none() && let Cell::Empty = board.values[i][j] {
                    first = Some(i * 9 + j);
                }
                if let Cell::Empty = board.values[i][j] {
                    last = i * 9 + j;
                    candidates[i][j] = Some(('1'..='9').into_iter().collect());
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

    fn current(&mut self) -> &mut Option<Vec<char>> {
        let pos = self.current_pos;
        &mut self[pos]
    }
    fn pop_candidate(&mut self) {
        if let Some(cands) = self.current() {
            cands.pop();
        }
    }

    fn reset_curr_cands(&mut self) {
        *self.current() = Some(('1'..='9').into_iter().collect());
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

    fn is_current_candidate_valid(&self) -> bool {
        let r = self.current_pos / 9;
        let c = self.current_pos % 9;
        let cand = self[self.current_pos].as_ref().unwrap().last().unwrap();

        for i in 0..r {
            match self.board.values[i][c] {
                Given(x) | NonGiven(x) => {
                    if x == *cand { return false; }
                },
                Empty => {},
            }
        }
        for j in 0..c {
            match self.board.values[r][j] {
                Given(x) | NonGiven(x) => {
                    if x == *cand { return false; }
                },
                Empty => {},
            }
        }
        let l = c - c % 3; let u = r - r % 3;
        for i in u..(u+3) {
            for j in l..(l+3) {
                match self.board.values[i][j] {
                    Given(x) | NonGiven(x) => {
                        if x == *cand { return false; }
                    },
                    Empty => {},
                }
            }
        }

        true
    }
}

fn count_solutions(board: &Board, test_unique: bool) -> u64 {
    let mut solutions_count: u64 = 0;
    let mut cands_board = CandidatesBoard::new(&board);
    'outer: loop {
        if cands_board.is_current_candidate_valid() {
            if cands_board.is_last() {
                solutions_count += 1;
                if test_unique && solutions_count >= 2 {
                    return solutions_count;
                }
                if cands_board.is_first() {
                    return solutions_count;
                }

                loop {
                    cands_board.reset_curr_cands();
                    cands_board.dec();
                    cands_board.pop_candidate();

                    if cands_board.current().as_ref().unwrap().is_empty() {
                        if cands_board.is_first() {
                            return solutions_count;
                        }
                    } else {
                        continue 'outer;
                    }
                }
            } else {
                cands_board.inc();
            }
        } else {
            loop {
                if cands_board.is_first()
                && cands_board.current().as_ref().unwrap().is_empty() {
                    return solutions_count;
                }

                cands_board.pop_candidate();
                if cands_board.current().as_ref().unwrap().is_empty() {
                    if cands_board.is_first() {
                        return solutions_count;
                    }
                    cands_board.reset_curr_cands();
                    cands_board.dec();
                    cands_board.pop_candidate();
                } else {
                    continue 'outer;
                }
            }
        }
    }
}

pub fn is_uniquely_solvable(board: &Board) -> bool {
    count_solutions(board, true) == 1
}
