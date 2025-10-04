use colored::*;
use std::io::{self, Write};
use crate::utils::{slow_print_color, clear_screen};
use crate::fractals::{draw_random_fractal, draw_fractal_variant};

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
            "help" => slow_print_color(
                "commands: help, exit, clear, fractal [-mandel | -tree | -noise]",
                sentence_color,
                12,
            ),
            "clear" | "cls" => clear_screen(),
            cmd if cmd.starts_with("fractal") => {
                let arg = cmd.split_whitespace().nth(1);
                draw_fractal_variant(arg, sentence_color);
            }
            _ => slow_print_color("unknown command — whisper 'help' to the void.", sentence_color, 12),
        }
    }
}