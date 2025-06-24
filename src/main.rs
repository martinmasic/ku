mod game;
mod generator;
mod testing;

use crate::generator::{*};
use rand::SeedableRng;
use rand_chacha::ChaCha8Rng;
use std::time::{SystemTime, UNIX_EPOCH};
use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>
}

#[derive(Subcommand)]
enum Commands {
    Test {

    },
    Generate {
        #[arg(short, long)]
        num_givens: u8,
    },
}

fn main() {
    let cli = Cli::parse();

    match &cli.command {
        Some(Commands::Test {}) => {
            // TODO
            println!("Testing... not yet implemented");
        },
        Some(Commands::Generate { num_givens: num }) => {
            let now: u64 = SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .expect("Time went backwards")
                .as_secs();

            let mut rng = ChaCha8Rng::seed_from_u64(now);
            let board = generate_valid_puzzle(&mut rng, *num);
            testing::print_board(&board);
        },
        None => {
            println!("???");
        }
    }





}
