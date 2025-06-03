use crate::game::{*};

use rand::seq::SliceRandom;
use rand::rngs::ThreadRng;
use rand::Rng;

pub fn fill_box_randomly<T: rand::Rng>(
    board: &mut Board,
    box_number: usize,
    rng: &mut T
) {
    let mut nums = crate::game::LEGAL_VALUES;
    nums.shuffle(rng);

    let mut num_i = 0;
    let l = ((box_number - 1) % 3) * 3;
    let u = (box_number - 1) / 3 * 3;

    for i in u..(u+3) {
        for j in l..(l+3) {
            board.values[i][j] = nums[num_i];
            num_i += 1;
        }
    }
}

pub fn zero_board() -> Board {
    let values = [[crate::game::EMPTY_VALUE; 9]; 9];

    let mut board = Board::new();
    board.values = values;

    board
}
