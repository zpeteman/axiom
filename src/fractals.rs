use colored::*;
use rand::Rng;
use std::{thread::sleep, time::Duration};

pub fn draw_random_fractal(color: Color) {
    // Default random selection
    let choice = rand::thread_rng().gen_range(0..3);
    match choice {
        0 => draw_mandelbrot(color),
        1 => draw_tree(color),
        _ => draw_noise(color),
    }
}

pub fn draw_fractal_variant(arg: Option<&str>, color: Color) {
    match arg {
        Some("-mandel") => draw_mandelbrot(color),
        Some("-tree") => draw_tree(color),
        Some("-noise") => draw_noise(color),
        _ => draw_random_fractal(color),
    }
}

// ─────────────────────────────────────────────
// FRACTAL VARIANTS
// ─────────────────────────────────────────────

fn draw_mandelbrot(color: Color) {
    let width = 80;
    let height = 30;
    let max_iter = 40;

    println!();
    for y in 0..height {
        for x in 0..width {
            let re = (x as f64 - width as f64 / 2.0) * 4.0 / width as f64;
            let im = (y as f64 - height as f64 / 2.0) * 2.0 / height as f64;

            let mut zr = 0.0;
            let mut zi = 0.0;
            let mut iter = 0;

            while zr * zr + zi * zi < 4.0 && iter < max_iter {
                let temp = zr * zr - zi * zi + re;
                zi = 2.0 * zr * zi + im;
                zr = temp;
                iter += 1;
            }

            let ch = match iter {
                0..=5 => "░",
                6..=10 => "▒",
                11..=20 => "▓",
                21..=30 => "█",
                _ => " ",
            };
            print!("{}", ch.color(color));
        }
        println!();
        sleep(Duration::from_millis(25));
    }
    println!("{}", "\n🜂 Mandelbrot — Infinity mirrored in itself.".color(color));
}

fn draw_tree(color: Color) {
    let mut rng = rand::thread_rng();
    let height = 16;

    println!();
    for i in 0..height {
        let branches = "🌿".repeat(i);
        let spacing = " ".repeat(height - i);
        println!("{}", format!("{}🜂{}", spacing, branches).color(color));
        sleep(Duration::from_millis(rng.gen_range(30..70)));
    }
    println!("{}", "🜂 The tree of thought grows unseen roots.".color(color));
}

fn draw_noise(color: Color) {
    let mut rng = rand::thread_rng();
    let width = 80;
    let height = 25;

    println!();
    for _ in 0..height {
        for _ in 0..width {
            let r = rng.gen_range(0..100);
            let ch = match r {
                0..=40 => "░",
                41..=70 => "▒",
                71..=90 => "▓",
                _ => "█",
            };
            print!("{}", ch.color(color));
        }
        println!();
        sleep(Duration::from_millis(15));
    }
    println!("{}", "\n🜂 Noise — order disguised as chaos.".color(color));
}