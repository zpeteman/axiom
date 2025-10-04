use clap::Parser;

/// 🌙 Axiom — terminal art, noise, and fractal beauty
#[derive(Parser, Debug)]
#[command(author, version, about)]
pub struct Cli {
    /// Choose a mode: fractal | art
    #[arg(short, long)]
    pub mode: Option<String>,

    /// Animate the pattern
    #[arg(short, long)]
    pub animate: bool,

    /// Clear the terminal
    #[arg(short, long)]
    pub clear: bool,
}