mod game;
mod generator;
mod testing;

// use crate::game::Board;
use crate::generator::{*};

fn main() {
    // use rand::seq::SliceRandom;
    use rand::SeedableRng;
    use rand_chacha::ChaCha8Rng;

    let seed: u64 = 42;
    let mut rng = ChaCha8Rng::seed_from_u64(seed);


    let board = generate_valid_board(&mut rng, 20);
    testing::print_board(&board);

}
