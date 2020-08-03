use is_prime::is_prime_with_witnesses;
use rand::prelude::*;
use std::io::prelude::*;

fn step(x: String, rng: &mut ThreadRng) -> Option<String> {
    let mut count = 0;
    loop {
        count += 1;
        if count > 10 * x.len() {
            return None;
        }
        let i = rng.gen_range(0, x.len() + 1);
        let d = rng.gen_range(0, 10) as u8;
        let mut copy = x.clone();
        copy.insert(i, (d + '0' as u8) as char);
        if copy.as_bytes()[0] != '0' as u8 {
            if is_prime_with_witnesses(&copy, 40) {
                return Some(copy);
            }
        }
    }
}

fn main() {
    let mut rng = thread_rng();
    let mut x;
    match rng.gen_range(0, 4) {
        0 => x = String::from("2"),
        1 => x = String::from("3"),
        2 => x = String::from("5"),
        3 => x = String::from("7"),
        _ => x = String::from("0"),
    }
    let mut length = 1;
    println!("{}", x);
    loop {
        match step(x, &mut rng) {
            Some(val) => {
                length += 1;
                println!("{}", val);
                std::io::stdout()
                    .flush()
                    .ok()
                    .expect("Could not flush stdout");
                x = val;
                if length > 100 {
                    length = 1;
                    match rng.gen_range(0, 4) {
                        0 => x = String::from("2"),
                        1 => x = String::from("3"),
                        2 => x = String::from("5"),
                        3 => x = String::from("7"),
                        _ => x = String::from("0"),
                    }
                    println!("{}", x);
                }
            }
            None => {
                println!("\n\nUnsucessful! Only length: {}\n\n\n", length);
                length = 1;
                match rng.gen_range(0, 4) {
                    0 => x = String::from("2"),
                    1 => x = String::from("3"),
                    2 => x = String::from("5"),
                    3 => x = String::from("7"),
                    _ => x = String::from("0"),
                }
            }
        }
    }
}
