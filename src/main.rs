mod cli;
mod fractals;
mod art;
mod utils;
mod ai;

use cli::Cli;
use clap::Parser;
use utils::clear_terminal;

fn main() {
    let args = Cli::parse();

    if args.clear {
        clear_terminal();
        return;
    }

    match args.mode.as_deref() {
        Some("fractal") => fractals::show_fractal(args.animate),
        Some("art") => art::show_art(args.animate),
        _ => {
            println!("Welcome to Axiom!");
            println!("Use --help to explore available modes.");
        }
    }
}