use std::collections::HashMap;

#[derive(Debug)]
struct Tree {
    value: u128,
    children: Vec<Tree>,
}

impl Tree {
    fn new(value: u128, children: Vec<u128>) -> Tree {
        let mut tree_children = Vec::with_capacity(children.len());
        for child in children {
            tree_children.push(Tree::new(child, Vec::new()));
        }
        Tree {
            value,
            children: tree_children,
        }
    }

    fn to_string(&self) -> String {
        let mut retval = format!("[{}: ", self.value);
        for child in &self.children {
            retval.push_str(&child.to_string()[..]);
        }
        retval.push(']');
        retval
    }

    fn step(&mut self, primes: &mut HashMap<u128, bool>) {
        if self.children.is_empty() {
            let xs = step(self.value, primes);
            for val in xs {
                self.children.push(Tree::new(val, Vec::new()));
            }
            return;
        }
        for child in &mut self.children {
            child.step(primes);
        }
    }

    fn longest_path(&self) -> Vec<u128> {
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

fn is_prime(n: u128, primes: &mut HashMap<u128, bool>) -> bool {
    let res = primes.get(&n);
    match res {
        Some(val) => return *val,
        None => {
            let mut i = 2;
            while i * i <= n {
                if n % i == 0 {
                    primes.insert(n, false);
                    return false;
                }
                i += 1;
            }
            primes.insert(n, true);
            return true;
        }
    }
}

fn digits(n: u128) -> u32 {
    n.to_string().len() as u32
}

fn step(x: u128, primes: &mut HashMap<u128, bool>) -> Vec<u128> {
    let mut new_xs = Vec::new();
    let digits: u32 = digits(x);
    for d in 1..10 {
        let temp = d * 10u128.pow(digits) + x;
        if is_prime(temp, primes) {
            new_xs.push(temp);
        }
    }
    return new_xs;
}

fn main() {
    let mut tree = Tree::new(0, vec![2, 3, 5, 7]);
    let mut primes = HashMap::new();
    for i in 0..20 {
        tree.step(&mut primes);
        println!("{}: {:?}", i, tree.longest_path());
    }
    println!("{}", tree.to_string());
}
