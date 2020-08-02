use rayon::prelude::*;

// use std::thread;

// const NUM_THREADS: usize = 12;
const BASE: u64 = 2;

fn is_fourth_free(x: &u64) -> bool {
    let mut i = 2;
    let mut fourth = i * i * i * i;
    while fourth <= *x {
        if x % fourth == 0 {
            return false;
        }
        i += 1;
        fourth = i * i * i * i;
    }
    true
}

fn step(x: &u64) -> Vec<u64> {
    (0..BASE)
        .into_iter()
        .map(|d| x * BASE + d)
        .filter(|d| is_fourth_free(d))
        .collect()
}

fn next(ls: Vec<u64>) -> Vec<u64> {
    ls.par_iter().map(step).flatten().collect()
}

// fn next(ls: Vec<u64>) -> Vec<u64> {
//     let mut new: Vec<u64> = Vec::new();
//     let mut children = Vec::with_capacity(NUM_THREADS);
//     let size = ls.len() / NUM_THREADS;
//     (0..(NUM_THREADS - 1))
//         .map(|i| -> Vec<u64> { (&ls[size * i..size * (i + 1)]).iter().copied().collect() })
//         .for_each(|slice| {
//             children.push(thread::spawn(move || {
//                 slice.iter().map(|oldval| step(oldval)).flatten().collect()
//             }))
//         });
//     children.push(thread::spawn(move || -> Vec<u64> {
//         let mut new = Vec::new();
//         for oldval in &ls[size * (NUM_THREADS - 1)..] {
//             new.append(&mut step(oldval));
//         }
//         new
//     }));
//     for child in children {
//         new.append(&mut child.join().unwrap());
//     }
//     new
// }

fn main() {
    let mut i = 0;
    let mut val: f64 = 0.0;
    let mut ls: Vec<u64> = (1..BASE)
        .into_iter()
        .filter(|x| is_fourth_free(x))
        .collect();
    loop {
        i += 1;
        val += ls.len() as f64 / 2u64.pow(i) as f64;
        println!("{}\t{}\t{}", i, ls.len(), val);
        ls = next(ls);
    }
}
