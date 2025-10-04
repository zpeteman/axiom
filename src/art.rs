use colored::*;
use crate::ai::ask_ai;
use tokio::time::{sleep, Duration};
use tokio::select;

pub async fn intro() -> colored::Color {
    let banner = r#"
╭──────────────────────────────────────────────╮
│                                              │
│     A X I O M — Terminal of Thought          │
│                                              │
╰──────────────────────────────────────────────╯
"#;
    println!("{}", banner.truecolor(180, 160, 255));
    
    let sentence_color = Color::TrueColor { r: 150, g: 150, b: 150 };
    
    let dots = vec!["   ", ".  ", ".. ", "..."];
    
    // Start AI request
    let mut ai_task = tokio::spawn(async {
        ask_ai(
            "Generate a single abstract, poetic, philosophical sentence about mathematics and consciousness, of less than 8 words."
        ).await
    });
    
    // Run animation until AI completes
    let mut idx = 0;
    let sentence = loop {
        print!("\r{}", format!("🜂 thinking{}", dots[idx % dots.len()]).color(sentence_color));
        std::io::Write::flush(&mut std::io::stdout()).unwrap();
        idx += 1;
        
        select! {
            result = &mut ai_task => {
                break match result {
                    Ok(Ok(s)) => s,
                    _ => "Abstraction is the soul whispering through symbols.".to_string(),
                };
            }
            _ = sleep(Duration::from_millis(400)) => {
                // Continue animation
            }
        }
    };
    
    print!("\r{}", " ".repeat(100)); // clear line
    print!("\r");
    
    // print AI sentence
    for c in sentence.chars() {
        print!("{}", c.to_string().color(sentence_color));
        std::io::Write::flush(&mut std::io::stdout()).unwrap();
        sleep(Duration::from_millis(20)).await;
    }
    println!();
    
    sentence_color
}