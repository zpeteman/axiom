use rand::Rng;
use std::{thread, time::Duration};

pub fn show_fractal(animate: bool) {
    let mut rng = rand::thread_rng();

    for _ in 0..10 {
        for _y in 0..20 {
            for _x in 0..60 {
                let val = rng.gen_range(0..=10);
                let ch = match val {
                    0..=2 => '.',
                    3..=5 => '*',
                    6..=8 => '+',
                    _ => '#',
                };
                print!("{}", ch);
            }
            println!();
        }

        if animate {
            thread::sleep(Duration::from_millis(150));
            print!("\x1B[2J\x1B[1;1H");
        } else {
            break;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_show_fractal_no_animate() {
        show_fractal(false);
    }
}