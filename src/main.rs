mod game;
mod generator;
mod cli_display;
mod naive_solver;

use crate::generator::{*};
use rand::SeedableRng;
use rand_chacha::ChaCha8Rng;
use std::time::{SystemTime, UNIX_EPOCH};
use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "ku")]
#[command(version = "0.0.1")]
#[command(about = "Ultimate sudoku app", long_about = None)]
#[command(next_line_help = true)]
#[command(arg_required_else_help = true)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>
}

#[derive(Subcommand)]
enum Commands {
    Generate {
        #[arg(short, long)]
        num_givens: u8,
    },
}

fn generate(num: &u8) {
    let now: u64 = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards")
        .as_secs();

    let mut rng = ChaCha8Rng::seed_from_u64(now);
    let board = generate_valid_puzzle(&mut rng, *num);
    cli_display::print_board(&board);
}

fn main() {
    let cli = Cli::parse();

    match &cli.command {
        Some(Commands::Generate { num_givens: num }) => generate(num),
        None => {}
    }

}
