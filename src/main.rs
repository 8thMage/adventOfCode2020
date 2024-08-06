use core::num;
use std::{
    cmp::Reverse,
    collections::{BinaryHeap, HashMap},
    fmt::Write,
    hash::Hash,
    iter::Cycle,
    str::FromStr,
};

fn main() {
    let input = include_str!("input.txt");
    println!(
        "{}",
        input
            .split(',')
            .map(|s| { s.bytes().fold(0, |s, b| ((s as u32 + b as u32) * 17) % 256) })
            .sum::<u32>()
    );
    let mut lenses = vec![vec![]; 256];
    input.split(',').for_each(|s| {
        if s.contains('=') {
            let (label, lens) = s.split_once('=').unwrap();
            let hash = label
                .bytes()
                .fold(0, |s, b| ((s as u32 + b as u32) * 17) % 256);
            if let Some((_, a)) = lenses[hash as usize].iter_mut().find(|(s, _)| *s == label) {
                *a = i64::from_str(lens).unwrap();
            } else {
                lenses[hash as usize].push((label, i64::from_str(lens).unwrap()));
            }
        }
        if s.contains('-') {
            let label = s.trim_end_matches('-');
            let hash = label
                .bytes()
                .fold(0, |s, b| ((s as u32 + b as u32) * 17) % 256);
            if let Some(pos) = lenses[hash as usize].iter().position(|(s, _)| *s == label) {
                lenses[hash as usize].remove(pos);
            }
        }
    });
    let res = lenses
        .iter()
        .enumerate()
        .map(|(i, v)| {
            v.iter()
                .enumerate()
                .map(|(pos, (_, a))| (pos + 1) as i32 * (i + 1) as i32 * *a as i32)
                .sum::<i32>()
        })
        .sum::<i32>();
    println!("{}", res);
}
