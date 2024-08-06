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
struct Beam {
    x: usize,
    y: usize,
    vx: i32,
    vy: i32,
}

fn main() {
    let input = include_str!("input.txt");
    let map = input
        .lines()
        .map(|s| s.chars().collect())
        .collect::<Vec<Vec<_>>>();
    let mut max = 0;
    for y in 0..map.len() {
        let beams = vec![Beam {
            x: 0,
            y,
            vx: 1,
            vy: 0,
        }];
        max = max.max(calc_energized(&map, beams));
        let beams = vec![Beam {
            x: map[0].len() - 1,
            y,
            vx: -1,
            vy: 0,
        }];
        max = max.max(calc_energized(&map, beams));
    }
    for x in 0..map[0].len() {
        let beams = vec![Beam {
            x,
            y: 0,
            vx: 0,
            vy: 1,
        }];
        max = max.max(calc_energized(&map, beams));
        let beams = vec![Beam {
            x,
            y: map.len() - 1,
            vx: 0,
            vy: -1,
        }];
        max = max.max(calc_energized(&map, beams));
    }
    println!("{}", max);
}

fn calc_energized(map: &Vec<Vec<char>>, mut beams: Vec<Beam>) -> u32 {
    let mut happened = HashSet::new();
    let mut energized = vec![vec![false; map[0].len()]; map.len()];
    let push_if_possible = |beams: &mut Vec<_>, x, y, vx, vy| {
        let nx = ((x as i32) + vx) as usize;
        let ny = ((y as i32) + vy) as usize;
        if nx >= map[0].len() || ny >= map.len() {
            return;
        }
        beams.push(Beam {
            x: nx,
            y: ny,
            vx,
            vy,
        });
    };
    while !beams.is_empty() {
        let beam = beams.pop().unwrap();
        if happened.contains(&beam) {
            continue;
        }
        happened.insert(beam);
        energized[beam.y][beam.x] = true;
        match map[beam.y][beam.x] {
            '/' => {
                push_if_possible(&mut beams, beam.x, beam.y, -beam.vy, -beam.vx);
            }
            '\\' => {
                push_if_possible(&mut beams, beam.x, beam.y, beam.vy, beam.vx);
            }
            '|' => {
                if beam.vx != 0 {
                    push_if_possible(&mut beams, beam.x, beam.y, 0, 1);
                    push_if_possible(&mut beams, beam.x, beam.y, 0, -1);
                } else {
                    push_if_possible(&mut beams, beam.x, beam.y, 0, beam.vy);
                }
            }
            '-' => {
                if beam.vy != 0 {
                    push_if_possible(&mut beams, beam.x, beam.y, 1, 0);
                    push_if_possible(&mut beams, beam.x, beam.y, -1, 0);
                } else {
                    push_if_possible(&mut beams, beam.x, beam.y, beam.vx, 0);
                }
            }
            '.' => {
                push_if_possible(&mut beams, beam.x, beam.y, beam.vx, beam.vy);
            }
            _ => unreachable!(),
        }
    }
    let res = energized
        .iter()
        .map(|v| v.iter().map(|b| *b as u32).sum::<u32>())
        .sum::<u32>();
    res
}
