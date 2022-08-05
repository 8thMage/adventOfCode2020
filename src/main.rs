// use core::panic;
use std::{collections::*, thread::current, cmp::Reverse};
// mod helpers;
// use helpers::*;
fn main() {
    let input = include_str!("input.txt");
    let mut map: Vec<Vec<i32>> = input
        .lines()
        .map(|s| s.chars().map(|c| c as i32 - '0' as i32).collect())
        .collect();
    let mut new_map: Vec<Vec<i32>> = vec![vec![0; 5 * map.len()]; 5 * map.len()];
    for i in 0..map.len() * 5 {
        for j in 0..map.len() * 5 {
            new_map[i][j] =
                map[i % map.len()][j % map.len()] + (i / map.len() + j / map.len()) as i32;
            if new_map[i][j] > 9 {
                new_map[i][j] -= 9;
            }
        }
    }
    let mut hash_map = HashSet::new();
    let mut vec = BinaryHeap::new();
    vec.push(Reverse((0, 0, 0)));
    let result = bfs(&(0, 0), &mut hash_map, &new_map, &mut vec);
    println!("{}", result);
}

fn bfs(
    position: &(i32, i32),
    hash_map: &mut HashSet<(i32, i32)>,
    map: &Vec<Vec<i32>>,
    free_queue: &mut BinaryHeap<Reverse<(i64, i32, i32)>>,
) -> i64 {
    while !free_queue.is_empty() {
        let position = free_queue.pop().unwrap();
        if !hash_map.get(&(position.0.1, position.0.2)).is_none() {
            continue;
        }

        if (position.0.1 as usize == map.len() - 1 && position.0.2 as usize == map.len() - 1) {
            return position.0.0;
        }
        // println!("{} {} {}", position.0.0, position.0.1, position.0.2);
        hash_map.insert((position.0.1, position.0.2));
        if position.0.1 < map.len() as i32 - 1 {
            if hash_map.get(&(position.0.1 + 1, position.0.2)).is_none() {
                free_queue.push(Reverse((
                    position.0.0 + map[position.0.1 as usize + 1][position.0.2 as usize] as i64,
                    position.0.1 + 1,
                    position.0.2,
                )));
            }
        }
        if position.0.2 < map.len() as i32 - 1 {
            if hash_map.get(&(position.0.1, position.0.2 + 1)).is_none() {
                free_queue.push(Reverse((
                    position.0.0 + map[position.0.1 as usize][position.0.2 as usize + 1] as i64,
                    position.0.1,
                    position.0.2 + 1,
                )));
            }
        }
        if position.0.1 > 0 {
            if hash_map.get(&(position.0.1 - 1, position.0.2)).is_none() {
                free_queue.push(Reverse((
                    position.0.0 + map[position.0.1 as usize - 1][position.0.2 as usize] as i64,
                    position.0.1 - 1,
                    position.0.2,
                )));
            }
        }
        if position.0.2 > 0 {
            if hash_map.get(&(position.0.1, position.0.2 - 1)).is_none() {
                free_queue.push(Reverse((
                    position.0.0 + map[position.0.1 as usize][position.0.2 as usize - 1] as i64,
                    position.0.1,
                    position.0.2 - 1,
                )));
            }
        }
    }
    return 0;
}
