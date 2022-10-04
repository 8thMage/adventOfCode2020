use std::{
    collections::{HashMap, HashSet},
    thread::current,
};

fn main() {
    let input = include_str!("input.txt");
    let mut map = vec![];
    for (_line_number, line) in input.lines().enumerate() {
        map.push(vec![]);
        for (_char_number, char) in line.chars().enumerate() {
            if char == '.' {
                map.last_mut().unwrap().push(0);
            } else if char == '>' {
                map.last_mut().unwrap().push(1);
            } else if char == 'v' {
                map.last_mut().unwrap().push(2);
            }
        }
    }
    let mut counter = 0;
    loop {
        let map2 = map_move_left(&map);
        let map3 = map_move_down(&map2);
        if map3
            .iter()
            .flat_map(|v| v.iter())
            .zip(map.iter().flat_map(|v| v.iter()))
            .all(|(e, e2)| e == e2)
        {
            println!("{}", counter + 1);
            break;
        }
         counter += 1;
        map = map3;
    }
}

fn map_move_left(map: &Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    let mut new_map = vec![vec![0; map[0].len()]; map.len()];
    for (y, v) in map.iter().enumerate() {
        for (x, e) in v.iter().enumerate() {
            if new_map[y][x] != 0 {
                continue;
            }
            if *e != 1 {
                new_map[y][x] = map[y][x];
            } else {
                if map[y][(x + 1) % map[0].len()] == 0 {
                    new_map[y][(x + 1) % map[0].len()] = 1;
                } else {
                    new_map[y][x] = 1;
                }
            }
        }
    }
    new_map
}

fn map_move_down(map: &Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    let mut new_map = vec![vec![0; map[0].len()]; map.len()];
    for (y, v) in map.iter().enumerate() {
        for (x, e) in v.iter().enumerate() {
            if new_map[y][x] != 0 {
                continue;
            }
            if *e != 2 {
                new_map[y][x] = map[y][x];
            } else {
                if map[(y + 1) % map.len()][x] == 0 {
                    new_map[(y + 1) % map.len()][x] = 2;
                } else {
                    new_map[y][x] = 2;
                }
            }
        }
    }
    new_map
}
