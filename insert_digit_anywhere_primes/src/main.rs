use std::collections::HashMap;

#[derive(Debug)]
struct Tree {
    value: u64,
    children: Vec<Tree>,
}

impl Tree {
    fn new(value: u64, children: Vec<u64>) -> Tree {
        let mut tree_children = Vec::with_capacity(children.len());
        for child in children {
            tree_children.push(Tree::new(child, Vec::new()));
        }
        Tree {
            value,
            children: tree_children,
        }
    }

    fn step(&mut self) {
        if self.children.is_empty() {
            let xs = step(self.value);
            for val in xs {
                self.children.push(Tree::new(val, Vec::new()));
            }
            return;
        }
        // let handles = Vec::new();
        // let xs = Vec::new();
        for child in &mut self.children {
            child.step();

            // handles.push(std::thread::spawn(|| {
            //     self.children[i].step();
            // }));
        }
        /*
        for handle in handles {
            handle.join().unwrap();
        }
        */
    }

    fn longest_path(&self) -> Vec<u64> {
        if self.children.is_empty() {
            return vec![self.value];
        }
        let mut max_path = vec![];
        let mut max_length = 0;
        for child in &self.children {
            let temp = child.longest_path();
            if temp.len() > max_length {
                max_length = temp.len();
                max_path = temp;
            }
        }
        let mut retval = vec![self.value];
        retval.append(&mut max_path);
        return retval;
    }
}

fn step(x: u64) -> Vec<u64> {
    let str_x = x.to_string();
    let mut new_xs = Vec::new();
    for i in 0..str_x.len() + 1 {
        for d in 0..10 {
            let temp = format!("{}{}{}", &str_x[..i], d.to_string(), &str_x[i..]).parse().unwrap();
            if !(i == 0 && d == 0) {
                if primal::is_prime(temp) {
                    new_xs.push(temp);
                }
            }
        }
    }
    return new_xs;
}

fn is_prime(x: u64, primes: &mut HashMap<u64, bool>) -> bool {
    let is_prime = primes.get(&x);
    match is_prime {
        Some(val) => return *val,
        None => {
            let mut i = 2;
            while i * i <= x {
                if x % i == 0 {
                    primes.insert(x, false);
                    return false;
                }
                i += 1;
            }
            primes.insert(x, true);
            return true;
        },
    }
}

fn main() {
    let mut tree = Tree::new(0, vec![2, 3, 5, 7]);
    let mut primes: HashMap<u64, bool> = HashMap::new();
    for _ in 0..10 {
        tree.step();
        println!("{:?}", tree.longest_path());
    }
}
