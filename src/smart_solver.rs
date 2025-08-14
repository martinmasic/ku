use crate::game::{ *, Cell::{ * } };

fn count_solutions(board: &Board, test_unique: bool) -> u64 {
    // TODO
}

pub fn is_uniquely_solvable(board: &Board) -> bool {
    count_solutions(board, true) == 1
}
