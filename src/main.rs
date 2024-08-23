use core::{f64, num};
use std::{
    cell::RefCell,
    cmp::Reverse,
    collections::{BinaryHeap, HashMap, HashSet, VecDeque},
    fmt::Write,
    hash::Hash,
    i64,
    iter::{repeat, Cycle, FromIterator},
    mem::swap,
    rc::Rc,
    str::FromStr,
    thread::current,
};

#[derive(Clone, Copy, Debug)]
struct Stone {
    x: i64,
    y: i64,
    z: i64,
    vx: i64,
    vy: i64,
    vz: i64,
}

#[derive(Clone, Copy, Debug)]
struct StoneWindow {
    start: Stone,
    size: u64,
}

fn main() {
    let input = include_str!("input.txt");
    let mut hail_stones = input
        .lines()
        .map(|l| {
            let mut iter = l
                .split(' ')
                .filter_map(|s| i64::from_str(s.trim_end_matches([','])).ok());
            Stone {
                x: iter.next().unwrap(),
                y: iter.next().unwrap(),
                z: iter.next().unwrap(),
                vx: iter.next().unwrap(),
                vy: iter.next().unwrap(),
                vz: iter.next().unwrap(),
            }
        })
        .collect::<Vec<_>>();
    let mut count = 0;
    for i in 0..hail_stones.len() {
        for j in i + 1..hail_stones.len() {
            let a = hail_stones[i];
            let b = hail_stones[j];
            let t = ((a.vx * (b.y - a.y) - a.vy * (b.x - a.x)) as f64)
                / ((a.vy * b.vx - a.vx * b.vy) as f64);
            let s = (b.x as f64 - a.x as f64 + b.vx as f64 * t) / a.vx as f64;
            if t < 0. || s < 0. {
                continue;
            }
            let intersection_x = b.x as f64 + t * b.vx as f64;
            if intersection_x < 200000000000000. || intersection_x > 400000000000000. {
                continue;
            }
            let intersection_y = b.y as f64 + t * b.vy as f64;
            if intersection_y < 200000000000000. || intersection_y > 400000000000000. {
                continue;
            }
            count += 1;
        }
    }
    println!("{}", count);
    let mut stones = hail_stones
        .iter()
        .map(|stone| {
            vec![
                stone.x as f64 / 1e13,
                stone.y as f64 / 1e13,
                stone.z as f64 / 1e13,
                stone.vx as f64,
                stone.vy as f64,
                stone.vz as f64,
            ]
        })
        .collect::<Vec<_>>();
    let mut current_min_value = 1e100;
    let mut current_min = vec![];
    for i in 0..10i32.pow(6) {
        let a0 = i % 10;
        let a1 = (i / 10) % 10;
        let a2 = (i / 10i32.pow(2)) % 10;
        let a3 = (i / 10i32.pow(3)) % 10;
        let a4 = (i / 10i32.pow(4)) % 10;
        let a5 = (i / 10i32.pow(5)) % 10;
        let stone = vec![
            50. * (a0 as f64 - 5.),
            50. * (a1 as f64 - 5.),
            50. * (a2 as f64 - 5.),
            50. * (a3 as f64 - 5.),
            50. * (a4 as f64 - 5.),
            50. * (a5 as f64 - 5.),
        ];
        let v = calculate_current_function(&stone, &stones);
        if current_min_value > v {
            current_min_value = v;
            current_min = stone.clone();
        }
    }
    let mut current_stone = current_min;
    let beta = 1. - 1e-4;
    let mut current_gradient = vec![0.0; 6];
    let mut index = 0;
    loop {
        if index % 1 == 0 {
            println!(
                "index {} current_value {}",
                index,
                calculate_current_function(&current_stone, &stones).log2()
            );
        }
        index += 1;
        let delta_value_x =
            calculate_delta_stone_x_coord(&current_stone, &stones, 0, 0, stones.len());
        let delta_value_y =
            calculate_delta_stone_x_coord(&current_stone, &stones, 1, 0, stones.len());
        let delta_value_z =
            calculate_delta_stone_x_coord(&current_stone, &stones, 2, 0, stones.len());
        let delta_value_vx =
            calculate_delta_stone_p_coord(&current_stone, &stones, 0, 0, stones.len());
        let delta_value_vy =
            calculate_delta_stone_p_coord(&current_stone, &stones, 1, 0, stones.len());
        let delta_value_vz =
            calculate_delta_stone_p_coord(&current_stone, &stones, 2, 0, stones.len());
        let max = delta_value_x
            .abs()
            .max(delta_value_y.abs())
            .max(delta_value_z.abs())
            .max(delta_value_vx.abs())
            .max(delta_value_vy.abs())
            .max(delta_value_vz.abs())
            / 10.;
        for ((x, y), z) in current_gradient
            .iter_mut()
            .zip(current_stone.iter_mut())
            .zip(
                [
                    delta_value_x,
                    delta_value_y,
                    delta_value_z,
                    delta_value_vx,
                    delta_value_vy,
                    delta_value_vz,
                ]
                .iter(),
            )
        {
            *x = *x * beta - (1. - beta) * z;
            *y = *y + *x / 10.;
        }
    }
}

fn calculate_current_function(current_stone: &Vec<f64>, stones: &[Vec<f64>]) -> f64 {
    let value = stones
        .iter()
        .map(|stone| {
            let diff = current_stone
                .iter()
                .zip(stone.iter())
                .map(|(x, y)| x - y)
                .collect::<Vec<_>>();
            (diff[0] * diff[4]).abs()
                + (diff[0] * diff[5]).abs()
                + (diff[1] * diff[3]).abs()
                + (diff[1] * diff[5]).abs()
                + (diff[2] * diff[3]).abs()
                + (diff[2] * diff[4]).abs()
        })
        .sum::<f64>();
    return value;
}

fn calculate_index_of_max(current_stone: &Vec<f64>, stones: &[Vec<f64>]) -> usize {
    let value = stones
        .iter()
        .map(|stone| {
            let diff = current_stone
                .iter()
                .zip(stone.iter())
                .map(|(x, y)| x - y)
                .collect::<Vec<_>>();
            (diff[0] * diff[4]).abs()
                + (diff[0] * diff[5]).abs()
                + (diff[1] * diff[3]).abs()
                + (diff[1] * diff[5]).abs()
                + (diff[2] * diff[3]).abs()
                + (diff[2] * diff[4]).abs()
        })
        .enumerate()
        .max_by_key(|(i, x)| *x as i64);
    return value.unwrap().0;
}

fn calculate_delta_stone_x_coord(
    current_stone: &Vec<f64>,
    stones: &[Vec<f64>],
    coord: usize,
    start_index: usize,
    len: usize,
) -> f64 {
    let delta_coord_1 = (coord + 1) % 3;
    let delta_coord_2 = (coord + 2) % 3;
    let index = calculate_index_of_max(current_stone, stones);
    let grad = stones
        .iter()
        .cycle()
        .skip(index)
        .take(len)
        .map(|stone| {
            let diff = current_stone
                .iter()
                .zip(stone.iter())
                .map(|(x, y)| x - y)
                .collect::<Vec<_>>();
            diff[coord].signum() * (diff[delta_coord_1 + 3].abs() + diff[delta_coord_2 + 3].abs())
        })
        .sum::<f64>();
    return grad;
}

fn calculate_delta_stone_p_coord(
    current_stone: &Vec<f64>,
    stones: &[Vec<f64>],
    coord: usize,
    start_index: usize,

    len: usize,
) -> f64 {
    let coord = coord + 3;
    let delta_coord_1 = ((coord + 1) % 3) + 3;
    let delta_coord_2 = ((coord + 2) % 3) + 3;
    let index = calculate_index_of_max(current_stone, stones);
    let grad = stones
        .iter()
        .cycle()
        .skip(index)
        .take(len)
        .map(|stone| {
            let diff = current_stone
                .iter()
                .zip(stone.iter())
                .map(|(x, y)| x - y)
                .collect::<Vec<_>>();
            diff[coord].signum() * (diff[delta_coord_1 - 3].abs() + diff[delta_coord_2 - 3].abs())
        })
        .sum::<f64>();
    return grad;
}
