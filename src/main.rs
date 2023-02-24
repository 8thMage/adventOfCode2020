use std::collections::{HashSet, VecDeque};

fn main() {
    let input = include_str!("input.txt");
    let string: Vec<_> = input.chars().collect();

    'big_loop: for i in 3..string.len() {
        let mut  hash =HashSet::new();
        for j in 0..14 {
            if hash.contains(&string[i - j]) {
                continue 'big_loop;
            }
            hash.insert(&string[i-j]);
        }
        println!("{}", i + 1);
        break;
    }
}
