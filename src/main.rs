use std::{
    cmp::Ordering,
    collections::{hash_map, HashMap, HashSet, VecDeque},
    env::current_exe,
    hash::Hash,
    iter,
    ops::Neg,
    path,
    str::FromStr,
    thread::current,
    vec,
};

fn main() {
    let input = include_str!("input.txt");
    let sum: i64 = input
        .lines()
        .map(|line| {
            let game = line.split_once(": ").unwrap().1;
            let (winning, have) = game.split_once(" | ").unwrap();
            let mut winning_hash = winning.split(" ").fold(HashSet::new(), |mut acc, s| {
                if s != "" {
                    acc.insert(i64::from_str(s).unwrap());
                }
                acc
            });
            have.split(" ").fold(0i64, |acc, s| {
                if s == "" {
                    return acc;
                }
                let num = i64::from_str(s).unwrap();
                if winning_hash.contains(&num) {
                    if acc == 0 {
                        1i64
                    } else {
                        acc * 2
                    }
                } else {
                    acc
                }
            })
        })
        .sum();
    let winnings: Vec<_> = input
        .lines()
        .map(|line| {
            let game = line.split_once(": ").unwrap().1;
            let (winning, have) = game.split_once(" | ").unwrap();
            let mut winning_hash = winning.split(" ").fold(HashSet::new(), |mut acc, s| {
                if s != "" {
                    acc.insert(i64::from_str(s).unwrap());
                }
                acc
            });
            winning_hash
        })
        .collect();
    let mut multiplies = vec![1;input.lines().count()];
    let new_sum:i64 = input
        .lines().enumerate()
        .map(|(index, line)| {
            let game = line.split_once(": ").unwrap().1;
            let (winning, have) = game.split_once(" | ").unwrap();

            let sum = have.split(" ").filter(|s| {
                if *s == "" {
                    return false;
                }
                let num = i64::from_str(s).unwrap();
                if winnings[index].contains(&num) {
                    true
                } else {
                    false
                }
            }).count();
            for d in 1..=sum {
                multiplies[index+d]+=multiplies[index];
            }
            multiplies[index]
        }).sum();
    println!("{}", sum);
    println!("{}", new_sum);
}
