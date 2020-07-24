use std::thread;
use std::collections::HashSet;

const NUM_THREADS: usize = 6;
// const THREAD_SKIP: u64 = 1_000;
const BASE: u64 = 10;

fn remove_duplicates(ls: Vec<u64>) -> Vec<u64> {
    let mut hs: HashSet<u64> = HashSet::new();
    for val in &ls {
        hs.insert(*val);
    }
    hs.iter().map(|x| *x).collect()
}

fn u64_to_v(x: u64) -> Vec<u64> {
    let mut ll = Vec::new();
    let mut x = x;
    while x > 0 {
        ll.push(x % BASE);
        x /= BASE;
    }
    ll
}

fn v_to_u64(ll: Vec<u64>) -> u64 {
    let mut val = 0;
    let mut ll = ll;
    loop {
        match ll.pop() {
            Some(v) => {
                val += v;
                val *= BASE;
            }
            None => break,
        };
    }
    val / BASE
}

fn step(x: u64) -> Vec<u64> {
    let mut new_xs = Vec::new();
    let v_x = u64_to_v(x);
    for i in 0..v_x.len() + 1 {
        for d in 0..BASE {
            let mut v_x = u64_to_v(x);
            v_x.insert(i, d);
            let temp = v_to_u64(v_x);
            if !(i == 0 && d == 0) {
                if primal::is_prime(temp) {
                    new_xs.push(temp);
                }
            }
        }
    }
    new_xs
}

fn next(ls: Vec<u64>) -> (f64, Vec<u64>) {
    let old = ls.len();
    let mut count = 0;
    let mut new: Vec<u64> = Vec::new();
    let mut slices: Vec<Vec<u64>> = Vec::new();
    let mut children = Vec::with_capacity(NUM_THREADS);
    let size = ls.len() / NUM_THREADS;
    for i in 0..(NUM_THREADS - 1) {
        let mut new = Vec::with_capacity(size);
        for val in ls.iter().skip(size * i).take(size) {
            new.push(*val);
        }
        slices.push(new);
    }
    {
        let mut new = Vec::new();
        for val in ls.iter().skip(size * (NUM_THREADS - 1)) {
            new.push(*val);
        }
        slices.push(new);
    }
    for slice in slices {
        children.push(thread::spawn(move || -> (u64, Vec<u64>) {
            let mut new = Vec::new();
            let mut count = 0;
            for oldval in slice {
                let mut temp = step(oldval);
                // let temp = step(oldval);
                count += temp.len() as u64;
                // new.push(oldval + THREAD_SKIP);
                new.append(&mut temp);
            }
            (count, new)
        }));
    }
    for child in children {
        let (temp1, mut temp2) = child.join().unwrap();
        new.append(&mut temp2);
        count += temp1;
    }
    (count as f64 / old as f64, remove_duplicates(new))
}

fn main() {
    // let mut ls: Vec<u64> = (1..(NUM_THREADS as u64) * THREAD_SKIP)
    //     .into_iter()
    //     .collect();
    let mut count: f64;
    let mut i = 0;
    let mut total: f64 = 0.0;
    let mut ls = primal::Primes::all().take_while(|x| *x < BASE as usize).map(|x| x as u64).collect();
    loop {
        let res = next(ls);
        count = res.0;
        ls = res.1;
        i += 1;
        total += count;
        let res = total as f64 / i as f64;
        println!("{}\t{}", res, f64::abs(res / (BASE as f64 / f64::ln(BASE as f64))));
    }
}
