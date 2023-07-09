use std::{
    collections::{HashMap, HashSet, VecDeque},
    hash::Hash,
    iter, path,
    str::FromStr,
    thread::current,
    vec,
};

fn main() {
    let input = include_str!("input.txt");
    let mut stones = HashSet::new();
    let mut max_in_any_dim = 0;
    for line in input.lines() {
        let vec = line
            .split(",")
            .map(|s| i32::from_str(s).unwrap())
            .collect::<Vec<_>>();
        max_in_any_dim = max_in_any_dim.max(*vec.iter().max().unwrap());
        stones.insert(vec);
    }
    let mut counter = 0;
    let mut counter2 = 0;
    let mut interior = HashSet::new();
    let mut exterior = HashSet::new();
    for entry in stones.iter() {
        for i in 0..entry.len() {
            for d in [-1, 1] {
                let mut new_entry = entry.clone();
                new_entry[i] += d;
                if !stones.contains(&new_entry) {
                    counter += 1;
                    counter2 += fill(
                        &new_entry,
                        &stones,
                        &mut interior,
                        &mut exterior,
                        max_in_any_dim,
                    );
                }
            }
        }
    }
    println!("{}", counter);
    println!("{}", counter2);
}

fn fill(
    input: &Vec<i32>,
    stones: &HashSet<Vec<i32>>,
    interior: &mut HashSet<Vec<i32>>,
    exterior: &mut HashSet<Vec<i32>>,
    max_in_any_dim: i32,
) -> i32 {
    let mut should_be_filled = Vec::new();
    let mut filled = HashSet::new();
    should_be_filled.push(input.clone());
    while let Some(entry) = should_be_filled.pop() {
        for i in 0..entry.len() {
            for d in [-1, 1] {
                let mut new_entry = entry.clone();
                new_entry[i] += d;
                if stones.contains(&new_entry) {
                    continue;
                }
                if interior.contains(&new_entry) {
                    for i in filled.into_iter() {
                        interior.insert(i);
                    }
                    while let Some(x) = should_be_filled.pop() {
                        interior.insert(x);
                    }
                    return 0;
                }
                if exterior.contains(&new_entry)
                    || new_entry.iter().any(|&s| s < 0 || s > max_in_any_dim)
                {
                    for i in filled.into_iter() {
                        exterior.insert(i);
                    }
                    while let Some(x) = should_be_filled.pop() {
                        exterior.insert(x);
                    }
                    return 1;
                }
                if !should_be_filled.contains(&new_entry) && !filled.contains(&new_entry) {
                    should_be_filled.push(new_entry);
                }
            }
        }
        filled.insert(entry);
    }
    for i in filled.into_iter() {
        interior.insert(i);
    }
    return 0;
}
