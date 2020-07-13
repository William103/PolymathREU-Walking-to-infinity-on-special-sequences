/* An attempt to see if you can walk to infinity on non-squarefree numbers
 * The answer is a very obvious "yes" I should have seen coming.
 * 4 -> 44 -> 444 -> 4444 -> ...
 * will always be divisible by 4 and thus will always be non-squarefree
 * Similarly
 * 9 -> 99 -> 999 -> 9999 -> ...
 * will always be divisible by 9.
 */

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

    fn step(&mut self, square_free: &mut HashMap<u64, bool>) {
        if self.children.is_empty() {
            let xs = step_end(self.value, square_free);
            for val in xs {
                self.children.push(Tree::new(val, Vec::new()));
            }
            return;
        }
        for child in &mut self.children {
            child.step(square_free);
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

    fn to_string(&self) -> String {
        let mut retval = format!("[{}: ", self.value);
        for child in &self.children {
            retval.push_str(&child.to_string()[..]);
        }
        retval.push(']');
        retval
    }

    fn count(&self) -> usize {
        if self.children.is_empty() {
            return 1;
        }
        return self.children.iter().fold(0, |mut sum, x| { sum += x.count(); sum });
    }
}

fn step_end(x: u64, square_free: &mut HashMap<u64, bool>) -> Vec<u64> {
    let mut new_xs = Vec::new();
    for d in 0..10 {
        let temp = x * 10 + d;
        if !is_squarefree(temp, square_free) {
            new_xs.push(temp);
        }
    }
    return new_xs;
}

fn is_squarefree(n: u64, square_free: &mut HashMap<u64, bool>) -> bool {
    match square_free.get(&n) {
        Some(val) => *val,
        None => {
            let n2 = n as usize;
            for prime in primal::Primes::all().take(n2) {
                if n2 % (prime * prime) == 0 {
                    square_free.insert(n, false);
                    return false;
                }
            }
            square_free.insert(n, true);
            true
        }
    }
}

fn main() {
    let mut tree = Tree::new(0, vec![4, 9]);
    let mut square_free: HashMap<u64, bool> = HashMap::new();
    for _ in 0..10 {
        tree.step(&mut square_free);
        println!("{:?}", tree.longest_path());
        println!("{}", tree.count());
    }
    println!("{:?}", tree.to_string());
}
