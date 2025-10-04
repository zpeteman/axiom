mod cli;
mod ai;
mod art;
mod utils;

use art::intro;
use cli::run_cli;
use colored::Color;

#[tokio::main]
async fn main() {
    let sentence_color: Color = intro().await; // get AI sentence color
    run_cli(sentence_color).await;
}