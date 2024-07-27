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
    let mut sum = 0;
    let mut sum2 = 0;
    for line in lines {
        let mut deltas = vec![line
            .split(" ")
            .map(|s| i64::from_str(s).unwrap())
            .collect::<Vec<_>>()];
        while deltas.last().unwrap().iter().any(|x| *x != 0) {
            deltas.push(
                deltas
                    .last()
                    .unwrap()
                    .windows(2)
                    .map(|c| c[1] - c[0])
                    .collect(),
            );
        }
        sum += deltas.iter().map(|arr| arr.last().unwrap()).sum::<i64>();
        sum2 += deltas
            .iter().rev()
            .map(|arr| arr.first().unwrap())
            .fold(0, |acc, x| x - acc);
    }
    println!("{}", sum);
    println!("{}", sum2);
}
