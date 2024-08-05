use core::num;
use std::{
    cmp::Reverse,
    collections::{BinaryHeap, HashMap},
    fmt::Write,
    iter::Cycle,
    str::FromStr,
};

fn main() {
    let input = include_str!("input.txt");
    let mut current = vec![];
    let mut sum = 0;
    for line in input.lines().chain([""]) {
        if !line.is_empty() {
            current.push(line.chars().collect::<Vec<_>>());
        } else {
            let mut number_y = 0;
            'y: for y in 0..current.len() - 1 {
                for dy in 0..(y + 1).min(current.len() - y - 1) {
                    if current[y + dy + 1] != current[y - dy] {
                        continue 'y;
                    }
                }
                sum += 100 * (y + 1);
            }

            'x: for x in 0..current[0].len() - 1 {
                for dx in 0..(x + 1).min(current[0].len() - x - 1) {
                    for y in 0..current.len() {
                        if current[y][x + dx + 1] != current[y][x - dx] {
                            continue 'x;
                        }
                    }
                }
                sum += (x + 1);
            }

            current = vec![];
        }
    }
    println!("{}", sum);

    let mut current = vec![];
    let mut sum = 0;
    for line in input.lines().chain([""]) {
        if !line.is_empty() {
            current.push(line.chars().collect::<Vec<_>>());
        } else {
            let mut number_y = 0;
            'y: for y in 0..current.len() - 1 {
                let mut count = 0;
                for dy in 0..(y + 1).min(current.len() - y - 1) {
                    count += current[y + dy + 1]
                        .iter()
                        .zip(current[y - dy].iter())
                        .filter(|(a, b)| **a != **b)
                        .count();
                    if count > 1 {
                        continue 'y;
                    }
                }
                if count == 1 {
                    sum += 100 * (y + 1);
                }
            }

            'x: for x in 0..current[0].len() - 1 {
                let mut count = 0;
                for dx in 0..(x + 1).min(current[0].len() - x - 1) {
                    for y in 0..current.len() {
                        if current[y][x + dx + 1] != current[y][x - dx] {
                            count += 1;
                            if count > 1 {
                                continue 'x;
                            }
                        }
                    }
                }
                if count == 1 {
                    sum += (x + 1);
                }
            }

            current = vec![];
        }
    }
    println!("{}", sum);
}
