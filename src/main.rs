use core::panic;
use std::collections::*;
mod helpers;
use helpers::*;
fn main() {
    let input = include_str!("input.txt");
    let mut map: Vec<Vec<usize>> = input
        .lines()
        .map(|s| s.chars().map(|c| c as usize - '0' as usize).collect())
        .collect();
    let mut sum = 0;
    let mut threeMax = [0,0,0];
    for y in 0..map.len() {
        for x in 0..map[0].len() {
            if y > 0 && map[y - 1][x] <= map[y][x] {
                continue;
            }
            if x > 0 && map[y][x - 1] <= map[y][x] {
                continue;
            }
            if x < map[0].len() - 1 && map[y][x + 1] <= map[y][x] {
                continue;
            }
            if y < map.len() - 1 && map[y + 1][x] <= map[y][x] {
                continue;
            }
            sum += map[y][x] + 1;
            let mut set = std::collections::hash_set::HashSet::new();
            set.insert((x,y));
            sizeOfBasin(x, y, &map, &mut set);
            if set.len() > threeMax[0] {
                threeMax[2] = threeMax[1];
                threeMax[1] = threeMax[0];
                threeMax[0] = set.len();
            }
            else if set.len() > threeMax[1] {
                threeMax[2] = threeMax[1];
                threeMax[1] = set.len();
            }
            else if set.len() > threeMax[2] {
                threeMax[2] = set.len();
            }
        }
    }
    println!("{}", sum);
    println!("{}", threeMax[0] * threeMax[1] * threeMax[2]);
}

fn sizeOfBasin(x:usize, y:usize, map:&Vec<Vec<usize>>, set:&mut std::collections::HashSet<(usize, usize)>) {
    let current = map[y][x];
    if y > 0 && map[y - 1][x] > current && map[y-1][x] < 9 && !set.contains(&(x,y-1)) {
        set.insert((x, y-1));
        sizeOfBasin(x, y-1, map, set);
    }
    if x > 0 && map[y][x - 1] > current && map[y][x - 1] < 9 && !set.contains(&(x-1,y)) {
        set.insert((x - 1, y));
        sizeOfBasin(x-1, y, map, set);
    }
    if !set.contains(&(x+1,y)) && x < map[0].len() - 1 && map[y][x + 1] > current && map[y][x + 1] < 9 {
        set.insert((x + 1, y));
        sizeOfBasin(x+1, y, map, set);
    }
    if !set.contains(&(x,y+1)) && y < map.len() - 1 && map[y+1][x] > current && map[y+1][x] < 9 {
        set.insert((x, y + 1));
        sizeOfBasin(x, y + 1, map, set);
    }
}

