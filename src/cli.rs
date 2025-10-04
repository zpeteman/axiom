use colored::*;
use std::io::{self, Write};
use crate::ai::ask_ai;
use crate::utils::slow_print_color;

pub async fn run_cli(sentence_color: Color) {
    loop {
        print!("{}", format!("🜂 > ").color(sentence_color));
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let input = input.trim();

        match input {
            "exit" | "quit" => {
                slow_print_color("Farewell, traveler of thought...", sentence_color, 30);
                break;
            }
            "help" => slow_print_color("commands: help, exit, ai <prompt>", sentence_color, 12),
            cmd if cmd.starts_with("ai ") => {
                let query = cmd.strip_prefix("ai ").unwrap();
                match ask_ai(query).await {
                    Ok(resp) => slow_print_color(&resp, sentence_color, 12),
                    Err(_) => slow_print_color("⚠️ failed to reach the oracle.", Color::Red, 12),
                }
            }
            _ => slow_print_color("unknown command — whisper 'help' to the void.", sentence_color, 12),
        }
    }
}