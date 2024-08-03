use std::{
    cmp::Reverse,
    collections::{BinaryHeap, HashMap},
};

fn main() {
    let input = include_str!("input.txt");
    let map = input
        .lines()
        .map(|c| c.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();
    let mut min_dist = HashMap::new();
    let mut wavefront = BinaryHeap::new();
    let start = map
        .iter()
        .enumerate()
        .find_map(|(y, line)| {
            line.iter()
                .enumerate()
                .find(|(x, &c)| c == 'S')
                .map(|(x, c)| (y, x))
        })
        .unwrap();
    wavefront.push((Reverse(0), start.0, start.1));
    let mut options = HashMap::new();
    options.insert('|', vec![[1, 0], [-1, 0]]);
    options.insert('-', vec![[0, 1], [0, -1]]);
    options.insert('L', vec![[-1, 0], [0, 1]]);
    options.insert('J', vec![[-1, 0], [0, -1]]);
    options.insert('7', vec![[1, 0], [0, -1]]);
    options.insert('F', vec![[1, 0], [0, 1]]);
    options.insert('.', vec![]);
    options.insert('S', vec![[1, 0], [0, 1], [-1, 0], [0, -1]]);

    while !wavefront.is_empty() {
        let (dist, y, x) = wavefront.pop().unwrap();
        let pipe = map[y][x];
        if min_dist.contains_key(&(y, x)) {
            continue;
        }
        min_dist.insert((y, x), dist);
        let Some(specific_options) = options.get(&pipe) else {
            continue;
        };
        for option in specific_options {
            let new_y = y.wrapping_add(option[0] as usize);
            let new_x = x.wrapping_add(option[1] as usize);
            if new_y < map.len() && new_x < map[0].len() {
                if options[&map[new_y][new_x]].contains(&[-option[0], -option[1]]) {
                    wavefront.push((Reverse(dist.0 + 1), new_y, new_x));
                }
            }
        }
    }
    let mut count = 0;
    for y in 0..map.len() {
        let mut polarity = 0;
        let mut last_up = None;
        for x in 0..map[0].len() {
            if let Some(dist) = min_dist.get(&(y, x)) {
                let connected_up = min_dist
                    .get(&(y.wrapping_sub(1), x))
                    .map_or(false, |v| (dist.0 as i32).abs_diff(v.0 as i32) == 1);
                let connected_down = min_dist
                    .get(&(y + 1, x))
                    .map_or(false, |v| (dist.0 as i32).abs_diff(v.0 as i32) == 1);
                if connected_up && connected_down {
                    polarity ^= 1;
                } else if last_up.is_none() {
                    last_up = Some(connected_up);
                } else if connected_down || connected_up {
                    if last_up != Some(connected_up) {
                        polarity ^= 1;
                    }
                }
            } else if polarity == 1 {
                count += 1;
            }
        }
    }
    let max = min_dist.values().min().unwrap();
    println!("{}", max.0);
    println!("{}", count);
}
