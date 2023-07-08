use std::{
    collections::{HashMap, HashSet, VecDeque},
    hash::Hash,
    iter, path,
    str::FromStr,
    thread::current,
    vec,
};

fn main() {
    let input = include_str!("input.txt");
    let mut dirs = input.chars().map(|c| if c == '<' { -1 } else { 1 }).cycle();
    let rocks_str = include_str!("rocks.rs");
    let iter = rocks_str.split("\n\n");
    let mut rocks = vec![];
    for rock_str in iter {
        let mut rock = vec![];
        for line in rock_str.lines() {
            rock.push(
                line.chars()
                    .map(|c| if c == '#' { 1 } else { 0 })
                    .collect::<Vec<_>>(),
            )
        }
        rock.reverse();
        rocks.push(rock);
    }
    let mut map = vec![vec![true; 9]];
    let mut counter = -1;
    // let mut delta_height = vec![];
    for rock in rocks.iter().cycle().take(1002220) {
        counter += 1;
        let mut y = map.len() + 3;
        let mut x = 3;
        loop {
            let dir = dirs.next().unwrap();
            let new_x = (x as i32 + dir) as usize;
            if !get_rock_iter(rock, new_x, y).any(|(ry, rx)| {
                rx == 8
                    || rx == 0
                    || (map.get(ry).is_some()
                        && map.get(ry).unwrap().get(rx).is_some()
                        && *map.get(ry).unwrap().get(rx).unwrap())
            }) {
                x = new_x;
            }
            let should_stop = get_rock_iter(rock, x, y).any(|(ry, rx)| {
                map.get(ry - 1).is_some()
                    && map.get(ry - 1).unwrap().get(rx).is_some()
                    && *map.get(ry - 1).unwrap().get(rx).unwrap()
            });
            if should_stop {
                let prev_len = map.len();
                for (ry, rx) in get_rock_iter(rock, x, y) {
                    if map.get(ry).is_none() {
                        map.push(vec![false; 9]);
                        map[ry][0] = true;
                        map[ry][8] = true;
                    }
                    map[ry][rx] = true;
                }
                if counter >= 10000 {
                    // delta_height.push(map.len() - prev_len);
                    // println!("{} {}", counter, map.len() - prev_len);
                    if counter % 10000 == 0 {
                        println!("{} {}",counter,  map.len() - 1);
                    }
                }

                break;
            }
            y = y - 1;
        }
    }
    // for y in 0..1000 {
    //     print!("{}\t", y);
    //     for x in 0..map[y].len() {
    //         print!("{}", if map[y][x] { "#" } else { "." })
    //     }
    //     print!("\t");
    //     for x in 0..map[y].len() {
    //         print!("{}", if map[y][x] == map[y + 400][x] { "#" } else { "." })
    //     }
    //     println!("");
    // }
    println!("{}", map.len() - 1);
}

fn get_rock_iter(
    rock: &Vec<Vec<i8>>,
    x: usize,
    y: usize,
) -> impl Iterator<Item = (usize, usize)> + '_ {
    rock.iter().enumerate().flat_map(move |(ry, arr)| {
        iter::repeat(y + ry).zip(arr.iter().enumerate().filter_map(move |(rx, v)| {
            if *v == 1 {
                Some(x + rx)
            } else {
                None
            }
        }))
    })
}
