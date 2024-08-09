use core::num;
use std::{
    cmp::Reverse,
    collections::{BinaryHeap, HashMap, HashSet},
    fmt::Write,
    hash::Hash,
    iter::Cycle,
    str::FromStr,
};

#[derive(Clone, Copy, Hash, PartialEq, Eq)]
struct Entry {
    x: usize,
    y: usize,
    amount_moved_forward: usize,
    direction_forward: (i64, i64),
    cost: usize,
}

impl Entry {
    fn heuristic(&self) -> i64 {
        self.cost as i64 - self.x as i64 - self.y as i64
    }
}

impl PartialOrd for Entry {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(
            (self.cost as i64 - self.x as i64 - self.y as i64)
                .cmp(&(other.cost as i64 - other.x as i64 - other.y as i64))
                .reverse(),
        )
    }
}

impl Ord for Entry {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        (self.cost as i64 - self.x as i64 - self.y as i64)
            .cmp(&(other.cost as i64 - other.x as i64 - other.y as i64))
            .reverse()
    }
}

fn main() {
    let input = include_str!("input.txt");
    let mut current_point = (0, 0);
    let mut current_point_2 = (0, 0);
    let mut current_sum: i64 = 0;
    let mut current_sum_2: i64 = 0;
    let mut count = 0;
    let mut count_2 = 0;
    for line in input.lines() {
        let (dir, (amount, color)) = line
            .split_once(' ')
            .map(|(a, b)| (a, b.split_once(' ').unwrap()))
            .unwrap();
        let delta = match dir {
            "D" => (0, 1),
            "U" => (0, -1),
            "L" => (-1, 0),
            "R" => (1, 0),
            _ => unreachable!(),
        };
        for _ in 0..usize::from_str_radix(amount, 10).unwrap() {
            count += 1;
            let next_point_x = current_point.0 + delta.0;
            let next_point_y = current_point.1 + delta.1;
            current_sum += (next_point_y + current_point.1) * (next_point_x - current_point.0) / 2;
            current_point = (next_point_x, next_point_y);
        }
        let amount = u64::from_str_radix(&color[2..2 + 5], 16).unwrap();
        let dir = u64::from_str_radix(&color[2 + 5..2 + 5 + 1], 16).unwrap();
        let delta = match dir {
            1 => (0, 1),
            3 => (0, -1),
            2 => (-1, 0),
            0 => (1, 0),
            _ => unreachable!(),
        };
        count_2 += amount as i64;
        let next_point_x = current_point_2.0 + delta.0 * amount as i64;
        let next_point_y = current_point_2.1 + delta.1 * amount as i64;
        current_sum_2 +=
            (next_point_y + current_point_2.1) * (next_point_x - current_point_2.0) / 2;
        current_point_2 = (next_point_x, next_point_y);
    }
    current_sum += (0 + current_point.1) * (0 - current_point.0) / 2;
    current_sum = current_sum.abs() + count / 2 + 1;
    println!("{}", current_sum);
    current_sum_2 += (0 + current_point_2.1) * (0 - current_point_2.0) / 2;
    current_sum_2 = current_sum_2.abs() + count_2 / 2 + 1;
    println!("{}", current_sum_2);
}
