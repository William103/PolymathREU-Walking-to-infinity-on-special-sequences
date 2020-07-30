use std::io::prelude::*;
use uint::construct_uint;
use rand::prelude::*;

construct_uint! {
    pub struct U256(4);
}

fn is_prime(x: &U256) -> bool {
    let zero = U256::from_dec_str("0").unwrap();
    let six = U256::from_dec_str("6").unwrap();
    if x % 2 == zero || x % 3 == zero {
        return false;
    }
    let mut i = U256::from_dec_str("5").unwrap();
    while i * i < *x {
        if x % i == zero || x % (i + 2) == zero {
            return false;
        }
        i += six;
    }
    true
}

fn step(x: &U256, rng: &mut ThreadRng) -> Option<U256> {
    let mut count = 0;
    loop {
        let mut str_x = format!("{}", x);
        count += 1;
        if count > 10 * str_x.len() {
            return None;
        }
        let i = rng.gen_range(0, str_x.len() + 1);
        let d = rng.gen_range(0, 10) as u8;
        str_x.insert(i, (d + '0' as u8) as char);
        let temp = U256::from_dec_str(&str_x).unwrap();
        if !(i == 0 && d == 0) {
            if is_prime(&temp) {
                return Some(temp);
            }
        }
    }
}

fn main() {
    let mut x = U256::from_dec_str("0").unwrap();
    let mut rng = thread_rng();
    let mut length = 0;
    loop {
        match step(&x, &mut rng) {
            Some(val) => {
                length += 1;
                print!("{}, ", x);
                std::io::stdout().flush().ok().expect("Could not flush stdout");
                x = val;
            },
            None => {
                x = U256::from_dec_str("0").unwrap();
                println!(" {}\n\n", length);
            }
        }
    }
}
