mod game;
mod generator;
mod cli_display;
mod naive_solver;
mod evaluator;
mod utilities;
mod tui;

use std::io;

use crate::{
    generator::*,
    tui::*
};

use clap::{Parser, Subcommand};
use color_eyre::eyre::{WrapErr, bail};

#[derive(Parser)]
#[command(name = "ku")]
#[command(version = "0.1.0")]
#[command(about = "The ultimate sudoku app, eventually", long_about = None)]
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
    Tui,
    Gui,
}

fn generate(num: &u8) {
    let generator = NaiveGenerator::new(*num);
    let board = generator.generate_puzzle();
    cli_display::print_board(&board);
}

fn main() -> color_eyre::Result<()> {
    color_eyre::install()?; // TODO: is this needed?

    let cli = Cli::parse();

    match &cli.command {
        Some(Commands::Generate { num_givens: num }) => generate(num),
        Some(Commands::Tui) => { return tui::run(); },
        Some(Commands::Gui) => { panic!("Not implemented!"); },
        None => {}
    }

    Ok(())
}
