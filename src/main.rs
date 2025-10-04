mod cli;
mod ai;
mod art;
mod utils;
mod fractals;

use art::intro;
use cli::run_cli;
use colored::Color;

#[tokio::main]
async fn main() {
    let sentence_color: Color = intro().await;
    run_cli(sentence_color).await;
}