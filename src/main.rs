use core::num;
use std::{
    cmp::Reverse,
    collections::{BinaryHeap, HashMap, HashSet, VecDeque},
    fmt::Write,
    hash::Hash,
    iter::{Cycle, FromIterator},
    str::FromStr,
};

#[derive(PartialEq)]
enum Type {
    Broadcast,
    FlipFlop,
    Conjuction,
}

fn main() {
    let input = include_str!("input.txt");
    let mut map = input
        .lines()
        .map(|l| {
            let iter = l
                .split(|c| c == '~' || c == ',')
                .map(|s| i64::from_str(s).unwrap());
            Vec::from_iter(iter)
        })
        .collect::<Vec<Vec<_>>>();
    map.sort_by_key(|v| v[2]);
    let mut end_height = vec![];
    for (i, brick) in map.iter().enumerate() {
        let brick_end_height = map
            .iter()
            .take(i)
            .enumerate()
            .filter(|(j, v)| {
                !(v[0] > brick[3] || v[1] > brick[4] || brick[0] > v[3] || brick[1] > v[4])
            })
            .map(|(j, v)| v[5] - v[2] + 1 + end_height[j])
            .max()
            .unwrap_or(1);
        end_height.push(brick_end_height);
    }
    let mut sum = 0;
    let mut disintegrateble = HashSet::new();
    'i_loop: for (i, brick) in map.iter().enumerate() {
        let mut supporting = HashSet::new();
        let directly_above = map
            .iter()
            .enumerate()
            .skip(i)
            .filter(|(j, v)| supported_by(&end_height, *j, v, i, brick));
        for (k, brick_directly_above) in directly_above {
            if would_fall(&map, k, &mut supporting, &end_height, brick_directly_above) {
                continue 'i_loop;
            }
        }
        disintegrateble.insert(i);
    }
    let mut chain = 0;
    for (i, brick) in map.iter().enumerate() {
        let mut supporting = HashSet::new();
        supporting.insert(i);
        'j_loop: for (j, brick_b) in map.iter().enumerate().skip(i) {
            for (k, brick_c) in map.iter().enumerate().take(j) {
                if !supporting.contains(&k) && supported_by(&end_height, j, brick_b, k, brick_c) {
                    continue 'j_loop;
                }
            }
            if supporting
                .iter()
                .any(|k| supported_by(&end_height, j, brick_b, *k, &map[*k]))
            {
                supporting.insert(j);
            }
        }
        chain += supporting.len() - 1;
    }

    println!("{}", disintegrateble.len());
    println!("{}", chain);
}

fn would_fall(
    map: &Vec<Vec<i64>>,
    k: usize,
    supporting: &mut HashSet<usize>,
    end_height: &Vec<i64>,
    brick_directly_above: &Vec<i64>,
) -> bool {
    map.iter()
        .enumerate()
        .take(k)
        .filter(|(j, v)| supported_by(end_height, k, brick_directly_above, *j, v))
        .count()
        == 1
}

fn supported_by(
    end_height: &[i64],
    j: usize,
    brick_a: &Vec<i64>,
    i: usize,
    brick_b: &Vec<i64>,
) -> bool {
    if end_height[j] != brick_b[5] - brick_b[2] + end_height[i] + 1 {
        return false;
    }
    if brick_a[0] > brick_b[3]
        || brick_a[1] > brick_b[4]
        || brick_b[0] > brick_a[3]
        || brick_b[1] > brick_a[4]
    {
        return false;
    }
    return true;
}
