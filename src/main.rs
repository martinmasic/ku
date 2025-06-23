mod game;
mod generator;
mod testing;

use crate::generator::{*};
use rand::SeedableRng;
use rand_chacha::ChaCha8Rng;
use std::time::{SystemTime, UNIX_EPOCH};

fn main() {

    let now: u64 = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards")
        .as_secs();

    let mut rng = ChaCha8Rng::seed_from_u64(now);


    let board = generate_valid_board(&mut rng, 52);
    testing::print_board(&board);

}
