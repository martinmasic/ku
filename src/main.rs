mod game;
mod generator;

use crate::game::Board;
use crate::generator::{*};


pub fn print_board(board: Board) {
    for i in 0..3 {
        print!(
            "{} {} {} | {} {} {} | {} {} {}\n",
            board.values[i][0], board.values[i][1], board.values[i][2],
            board.values[i][3], board.values[i][4], board.values[i][5],
            board.values[i][6], board.values[i][7], board.values[i][8],
        );
    }
    for i in 0..21 {
        print!("-");
    }
    print!("\n");
    for i in 3..6 {
        print!(
            "{} {} {} | {} {} {} | {} {} {}\n",
            board.values[i][0], board.values[i][1], board.values[i][2],
            board.values[i][3], board.values[i][4], board.values[i][5],
            board.values[i][6], board.values[i][7], board.values[i][8],
        );
    }
    for i in 0..21 {
        print!("-");
    }
    print!("\n");
    for i in 6..9 {
        print!(
            "{} {} {} | {} {} {} | {} {} {}\n",
            board.values[i][0], board.values[i][1], board.values[i][2],
            board.values[i][3], board.values[i][4], board.values[i][5],
            board.values[i][6], board.values[i][7], board.values[i][8],
        );
    }
}

fn main() {
    use rand::seq::SliceRandom;
    use rand::{rng, SeedableRng};
    use rand_chacha::ChaCha8Rng;

    let mut board = generator::zero_board();

    let seed: u64 = 42;
    let mut rng = ChaCha8Rng::seed_from_u64(seed);

    generator::fill_box_randomly(&mut board, 2, &mut rng);

    print_board(board);

}
