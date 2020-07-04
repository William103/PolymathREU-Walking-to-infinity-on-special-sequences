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

    fn step(&mut self, primes: &mut HashMap<u64, bool>) {
        if self.children.is_empty() {
            let str_x = self.value.to_string();
            for i in 0..str_x.len() + 1 {
                for d in 0..10 {
                    let temp = format!("{}{}{}", &str_x[..i], d.to_string(), &str_x[i..]).parse().unwrap();
                    if !(i == 0 && d == 0) {
                        if is_prime(temp, primes) {
                            self.children.push(Tree::new(temp, Vec::new()));
                        }
                    }
                }
            }
            return;
        }
        for child in &mut self.children {
            child.step(primes);
        }
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
        tree.step(&mut primes);
        println!("{:?}", tree.longest_path());
    }
}
