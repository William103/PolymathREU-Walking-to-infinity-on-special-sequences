use rug::{Assign, Integer};
use std::thread;

const NUM_THREADS: usize = 12;
const BASE: u64 = 10;

fn is_fourth_free(x: &Integer) -> bool {
    let mut i = Integer::new();
    i.assign(2);
    let mut fourth = Integer::from(&Integer::from(&i * &i) * &Integer::from(&i * &i));
    while fourth < *x {
        if x % (fourth) == 0 {
            return false;
        }
        i += 1;
        fourth = Integer::from(&Integer::from(&i * &i) * &Integer::from(&i * &i));
    }
    true
}

fn step(x: &Integer) -> Vec<Integer> {
    let mut new_xs = Vec::new();
    for d in 0..BASE {
        let mut temp = Integer::from(x * BASE);
        temp += d;
        if is_fourth_free(&temp) {
            new_xs.push(temp);
        }
    }
    new_xs
}

fn next(ls: &Vec<Integer>) -> Vec<Integer> {
    let mut new: Vec<Integer> = Vec::new();
    let mut slices: Vec<Vec<Integer>> = Vec::new();
    let mut children = Vec::with_capacity(NUM_THREADS);
    let size = ls.len() / NUM_THREADS;
    for i in 0..(NUM_THREADS - 1) {
        let mut new = Vec::with_capacity(size);
        for val in ls.iter().skip(size * i).take(size) {
            new.push(val.clone());
        }
        slices.push(new);
    }
    {
        let mut new = Vec::new();
        for val in ls.iter().skip(size * (NUM_THREADS - 1)) {
            new.push(val.clone());
        }
        slices.push(new);
    }
    for slice in slices {
        children.push(thread::spawn(move || -> Vec<Integer> {
            let mut new = Vec::new();
            for oldval in slice {
                new.append(&mut step(&oldval));
            }
            new
        }));
    }
    for child in children {
        new.append(&mut child.join().unwrap());
    }
    new
}

fn main() {
    let mut i = 0;
    let mut ls: Vec<Integer> = (1..BASE)
        .into_iter()
        .map(|x| Integer::from(x))
        .filter(|x| is_fourth_free(x))
        .collect();
    loop {
        i += 1;
        println!("{}\t{}", i, ls.len());
        ls = next(&ls);
    }
}
