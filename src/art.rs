use colored::*;
use crate::ai::ask_ai;
use tokio::time::{sleep, Duration};

pub async fn intro() -> colored::Color {
    let banner = r#"
╭──────────────────────────────────────────────╮
│                                              │
│     A X I O M — Terminal of Thought          │
│                                              │
╰──────────────────────────────────────────────╯
"#;
    println!("{}", banner.truecolor(180, 160, 255));

    // choose a color for the sentence
    let sentence_color = Color::TrueColor { r: 150, g: 150, b: 150 };

    // animated thinking
    print!("{}", "🜂 ".color(sentence_color));
    print!("{}", "thinking".color(sentence_color));
    std::io::Write::flush(&mut std::io::stdout()).unwrap();

    let dots = vec!["   ", ".  ", ".. ", "..."];

    // spawn animation in its own task, move ownership into it
    let handle = tokio::spawn(async move {
        let mut idx = 0;
        loop {
            print!("\r{}", format!("🜂 thinking{}", dots[idx % dots.len()]).color(sentence_color));
            std::io::Write::flush(&mut std::io::stdout()).unwrap();
            idx += 1;
            sleep(Duration::from_millis(400)).await;
        }
    });

    // get AI-generated sentence
    let sentence = match ask_ai(
        "Generate a single abstract, poetic, philosophical sentence about mathematics and consciousness, of less than 8 words."
    ).await {
        Ok(s) => s,
        Err(_) => "Abstraction is the soul whispering through symbols.".to_string(),
    };

    // stop animation
    handle.abort();
    let _ = handle.await;

    print!("\r{}", " ".repeat(100)); // clear line
    print!("\r"); // return to beginning of line

    // print AI sentence in the same color
    for c in sentence.chars() {
        print!("{}", c.to_string().color(sentence_color));
        std::io::Write::flush(&mut std::io::stdout()).unwrap();
        sleep(Duration::from_millis(20)).await;
    }
    println!(); // new line after sentence

    sentence_color
}