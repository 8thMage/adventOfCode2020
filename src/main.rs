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
    let map = input
        .lines()
        .map(|s| s.chars().map(|c| c as usize - '0' as usize).collect())
        .collect::<Vec<Vec<_>>>();
    let current_position = (0, 0);
    let mut histories = BinaryHeap::new();
    histories.push(Entry {
        x: 0,
        y: 0,
        amount_moved_forward: 0,
        direction_forward: (1, 0),
        cost: 0,
    });
    let mut cost_to = HashMap::new();
    let mut min_cost = usize::MAX;
    while let Some(history) = histories.pop() {
        if history.y >= map.len() || history.x >= map[0].len() {
            continue;
        }
        if cost_to
            .get(&(
                history.x,
                history.y,
                history.direction_forward,
                history.amount_moved_forward,
            ))
            .map_or(false, |c| *c <= history.cost)
        {
            continue;
        }
        if min_cost < history.cost {
            // break;
            continue;
        }
        if history.y == map.len() - 1 && history.x == map[0].len() - 1 {
            min_cost = (history.cost + map[history.y][history.x] - map[0][0]).min(min_cost);
            continue;
        }
        *cost_to
            .entry((
                history.x,
                history.y,
                history.direction_forward,
                history.amount_moved_forward,
            ))
            .or_insert(history.cost) = history.cost;
        if history.amount_moved_forward < 10 {
            histories.push(Entry {
                x: history.x.wrapping_add(history.direction_forward.0 as usize),
                y: history.y.wrapping_add(history.direction_forward.1 as usize),
                direction_forward: history.direction_forward,
                amount_moved_forward: history.amount_moved_forward + 1,
                cost: history.cost + map[history.y][history.x],
            })
        }
        histories.push(Entry {
            x: history
                .x
                .wrapping_add((history.direction_forward.1 as usize).wrapping_mul(4)),
            y: history
                .y
                .wrapping_add((history.direction_forward.0 as usize).wrapping_mul(4)),
            direction_forward: (history.direction_forward.1, history.direction_forward.0),
            amount_moved_forward: 4,
            cost: history.cost
                + (0..4)
                    .map(|i| {
                        let new_x = history
                            .x
                            .wrapping_add(history.direction_forward.1.wrapping_mul(i) as usize);
                        let new_y = history
                            .y
                            .wrapping_add(history.direction_forward.0.wrapping_mul(i) as usize);

                        if new_x < map[0].len() && new_y < map.len() {
                            map[new_y][new_x]
                        } else {
                            0
                        }
                    })
                    .sum::<usize>(),
        });
        histories.push(Entry {
            x: history
                .x
                .wrapping_add((-history.direction_forward.1 as usize).wrapping_mul(4)),
            y: history
                .y
                .wrapping_add((-history.direction_forward.0 as usize).wrapping_mul(4)),
            direction_forward: (-history.direction_forward.1, -history.direction_forward.0),
            amount_moved_forward: 4,

            cost: history.cost
                + (0..4)
                    .map(|i| {
                        let new_x = history
                            .x
                            .wrapping_add(-history.direction_forward.1.wrapping_mul(i) as usize);
                        let new_y = history
                            .y
                            .wrapping_add(-history.direction_forward.0.wrapping_mul(i) as usize);

                        if new_x < map[0].len() && new_y < map.len() {
                            map[new_y][new_x]
                        } else {
                            0
                        }
                    })
                    .sum::<usize>(),
        });
    }
    println!("{}", min_cost);
}
