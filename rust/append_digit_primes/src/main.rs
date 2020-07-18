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

    // fn to_string(&self) -> String {
    //     let mut retval = format!("[{}: ", self.value);
    //     for child in &self.children {
    //         retval.push_str(&child.to_string()[..]);
    //     }
    //     retval.push(']');
    //     retval
    // }

    fn step(&mut self) {
        if self.children.is_empty() {
            let xs = step(self.value);
            for val in xs {
                self.children.push(Tree::new(val, Vec::new()));
            }
            return;
        }
        for child in &mut self.children {
            child.step();
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

fn step(x: u64) -> Vec<u64> {
    let mut new_xs = Vec::new();
    let temp = x * 10;
    for d in 0..10 {
        let temp2 = temp + d;
        if primal::is_prime(temp2) {
            new_xs.push(temp2);
        }
    }
    return new_xs;
}

fn main() {
    /* find all non-truncatable primes in (100,000,000; 1,000,000,000) */
    // let primes: Vec<u64> = primal::Primes::all()
    //     .skip_while(|x| *x < 100_000_000)
    //     .take_while(|x| *x < 1_000_000_000)
    //     .map(|x| x as u64)
    //     .filter(|x| !(primal::is_prime(*x / 10)))
    //     .collect();

    let mut tree = Tree::new(0, Vec::new());
    for _ in 0..9 {
        tree.step();
        println!("{:?}", tree.longest_path());
    }
}
