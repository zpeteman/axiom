use noise::{NoiseFn, Perlin};
use std::{thread, time::Duration};

pub fn show_art(animate: bool) {
    let perlin = Perlin::new(42);
    let mut time = 0.0;

    for _ in 0..30 {
        for y in 0..20 {
            for x in 0..60 {
                let value = perlin.get([x as f64 * 0.1, y as f64 * 0.1, time]);
                let ch = match value {
                    v if v < -0.4 => '.',
                    v if v < 0.0 => '*',
                    v if v < 0.4 => '+',
                    _ => '#',
                };
                print!("{}", ch);
            }
            println!();
        }

        if animate {
            thread::sleep(Duration::from_millis(120));
            print!("\x1B[2J\x1B[1;1H");
            time += 0.05;
        } else {
            break;
        }
    }
}