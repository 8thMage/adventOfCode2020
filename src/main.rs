use std::{
    collections::{HashMap, HashSet, VecDeque},
    hash::Hash,
    str::FromStr,
    vec,
};

fn main() {
    let input = include_str!("input.txt");
    let mut s_pos = (0, 0);
    let mut e_pos = (0, 0);
    let map: Vec<Vec<usize>> = input
        .lines()
        .enumerate()
        .map(|(y, line)| {
            line.chars()
                .enumerate()
                .map(|(x, c)| {
                    if c == 'S' {
                        s_pos = (x, y);
                        0
                    } else if c == 'E' {
                        e_pos = (x, y);
                        'z' as usize - 'a' as usize
                    } else {
                        c as usize - 'a' as usize
                    }
                })
                .collect()
        })
        .collect();
    {
        let mut visited_nodes = HashMap::new();
        let mut wavefront = Vec::new();
        wavefront.push((s_pos, 0));
        while !wavefront.is_empty() {
            let (vector, item) = wavefront.remove(0);
            let val = map[vector.1][vector.0];
            println!("{:?} {} {}", vector, item, ('a' as u8 + val as u8) as char);
            if vector == e_pos {
                println!("{}", item);
                break;
            }
            if visited_nodes.contains_key(&vector) {
                continue;
            }
            visited_nodes.insert(vector, item);
            for delta in [(1, 0), (usize::MAX, 0), (0, usize::MAX), (0, 1)] {
                let new_x = vector.0.wrapping_add(delta.0);
                let new_y = vector.1.wrapping_add(delta.1);
                if new_x == usize::MAX
                    || new_y == usize::MAX
                    || new_x >= map[0].len()
                    || new_y >= map.len()
                {
                    continue;
                }
                if map[vector.1][vector.0] + 1 < map[new_y][new_x] {
                    continue;
                }
                if visited_nodes.contains_key(&(new_x, new_y)) {
                    continue;
                }
                let new_price = item + 1;
                wavefront.push(((new_x, new_y), new_price));
            }
        }
    }
    {
        let mut visited_nodes = HashMap::new();
        let mut wavefront = Vec::new();
        wavefront.push((e_pos, 0));
        while !wavefront.is_empty() {
            let (vector, item) = wavefront.remove(0);
            let val = map[vector.1][vector.0];
            println!("{:?} {} {}", vector, item, ('a' as u8 + val as u8) as char);
            if map[vector.1][vector.0] == 0 {
                println!("{}", item);
                break;
            }
            if visited_nodes.contains_key(&vector) {
                continue;
            }
            visited_nodes.insert(vector, item);
            for delta in [(1, 0), (usize::MAX, 0), (0, usize::MAX), (0, 1)] {
                let new_x = vector.0.wrapping_add(delta.0);
                let new_y = vector.1.wrapping_add(delta.1);
                if new_x == usize::MAX
                    || new_y == usize::MAX
                    || new_x >= map[0].len()
                    || new_y >= map.len()
                {
                    continue;
                }
                if map[new_y][new_x] + 1 < map[vector.1][vector.0] {
                    continue;
                }
                if visited_nodes.contains_key(&(new_x, new_y)) {
                    continue;
                }
                let new_price = item + 1;
                wavefront.push(((new_x, new_y), new_price));
            }
        }
    }
}
