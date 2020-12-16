use std::collections::{HashMap, HashSet};
fn main() {
    let input = include_str!("input.txt");
    let input = "8,0,17,4,1,12";
    let mut input_vec: Vec<usize> = input
        .split_terminator(",")
        .map(|s| s.parse().unwrap())
        .collect();
    let mut hash:HashMap<usize, usize> = HashMap::new();
    for (k, i) in input_vec.iter().take(input_vec.len() - 1).enumerate() {
        hash.insert(*i, k + 1);
    }
    let mut new = *input_vec.last().unwrap();
    for i in input_vec.len() + 1..30000000 + 1 {
        let new_new = {
            if hash.contains_key(&new) {
                i - hash.get(&new).unwrap() - 1
            } else {
                0
            }
        };
        // hash[&new] = i-1;
        hash.insert(new, i- 1 );
        // println!("{}", new_new);
        new = new_new;
    }
    println!("{}", new);
}
