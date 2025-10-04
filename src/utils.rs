use std::{thread::sleep, time::Duration};
use colored::*;

pub fn slow_print_color(text: &str, color: Color, delay: u64) {
    for c in text.chars() {
        print!("{}", c.to_string().color(color));
        std::io::Write::flush(&mut std::io::stdout()).unwrap();
        sleep(Duration::from_millis(delay));
    }
    println!();
}