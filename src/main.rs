use std::{
    cmp::Ordering,
    collections::{hash_map, HashMap, HashSet, VecDeque},
    env::current_exe,
    hash::Hash,
    iter,
    ops::{Neg, RangeBounds},
    path,
    str::FromStr,
    thread::current,
    vec,
};

fn main() {
    let input = include_str!("input.txt");
    let mut lines = input.lines();
    let mut order = lines
        .next()
        .unwrap()
        .chars()
        .map(|c| {
            if c == 'R' {
                1
            } else if c == 'L' {
                0
            } else {
                panic!()
            }
        })
        .cycle();
    lines.next();
    let map = lines
        .map(|line| {
            let (key, pair) = line.split_once(" = (").unwrap();
            let (left, right) = pair.split_once(", ").unwrap();
            let right = right.split_once(")").unwrap().0;
            (key, left, right)
        })
        .fold(HashMap::new(), |mut acc, (key, left, right)| {
            acc.insert(key, (left, right));
            acc
        });
    // let mut current = "AAA";
    // let mut count = 0;
    // while current != "ZZZ" {
    //     let key = order.next().unwrap();
    //     let pair = map[current];
    //     if key == 0 {
    //         current = pair.0;
    //     } else {
    //         current = pair.1;
    //     }
    //     count += 1;
    // }
    // println!("{}", count);
    let mut lines = input.lines();
    let mut order = lines
        .next()
        .unwrap()
        .chars()
        .map(|c| {
            if c == 'R' {
                1
            } else if c == 'L' {
                0
            } else {
                panic!()
            }
        })
        .cycle();
    let mut current = vec![];
    for key in map.keys() {
        if key.ends_with("A") {
            current.push(*key);
        }
    }
    let mut count = 0;
    let mut minhit = vec![u64::MAX; current.len()];
    while !current.iter().all(|s| s.ends_with("Z")) {
        let key = order.next().unwrap();
        count += 1;
        current.iter_mut().enumerate().for_each(|(index, current)| {
            let pair = map[*current];
            if key == 0 {
                *current = pair.0;
            } else {
                *current = pair.1;
            }
            if current.ends_with("Z") {
                minhit[index] = minhit[index].min(count);
            }
        });
        if minhit.iter().all(|count| *count != u64::MAX) {
            break;
        }
    }
    let lcm = minhit.iter().fold(1, |acc, b| acc * *b / gcd(acc, *b));
    println!("{}", lcm);
}

fn gcd(a: u64, b: u64) -> u64 {
    if a < b {
        return gcd(b, a);
    }
    if b == 0 {
        return a
    }
    gcd(b, a.rem_euclid(b))
}
