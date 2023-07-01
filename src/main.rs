use std::{
    collections::{HashMap, HashSet, VecDeque},
    hash::Hash,
    str::FromStr,
    thread::current,
    vec,
};

fn main() {
    let input = include_str!("input.txt");
    let row_y = 2000000;
    // let row_y = 10;
    // let mut impossible_pos = HashSet::new();
    let mut balls = vec![];
    let mut count_beacon_in_row = vec![];
    let mut beacons = HashSet::new();
    for line in input.lines() {
        let mut iter = line
            .split(['=', ',', ':'])
            .filter_map(|s| i32::from_str(s).ok());
        let sx = iter.next().unwrap();
        let sy = iter.next().unwrap();
        let bx = iter.next().unwrap();
        let by = iter.next().unwrap();
        if by == row_y {
            if !count_beacon_in_row
                .iter()
                .any(|&(bx0, by0)| bx == bx0 && by == by0)
            {
                count_beacon_in_row.push((bx, by))
            }
        }
        beacons.insert((bx, by));
        let dist = (sx - bx).abs() + (sy - by).abs();
        balls.push((sx, sy, dist));
    }
    let mut current_x = balls
        .iter()
        .map(|(bx, by, dist)| bx - dist + (row_y - by).abs() - 10)
        .min()
        .unwrap();
    let mut max_x = balls
        .iter()
        .map(|(bx, by, dist)| bx + dist - (row_y - by).abs() + 10)
        .max()
        .unwrap();
    let mut current_y = row_y;
    let mut count_impossible = 0;
    loop {
        if current_x > max_x {

            break;
        }
        if let Some(&(bx, by, dist)) = balls.iter().find(|&(bx, by, dist)| {
            (bx - current_x).abs() + (by - current_y).abs() <= *dist
        }) {
            let new_x = bx + (dist - (by - current_y).abs())+1;
            count_impossible += new_x - current_x ;
            current_x = new_x;
            was_not_in_ball = 0;
            let x = 0;
            continue;
        } else {
        }
        current_x += 1;
    }
    println!("{}", count_impossible as usize - count_beacon_in_row.len());
    let max_x = 4000000;
    // let max_x = 20;
    let mut current_y = 0;
    loop {
        let mut current_x = 0;
        if current_y > max_x {
            break;
        }
        loop {
            if current_x > max_x {
                break;
            }
            if let Some(&(bx, by, dist)) = balls.iter().find(|&(bx, by, dist)| {
                (bx - current_x).abs() + (by - current_y).abs() <= *dist
            }) {
                let new_x = bx + (dist - (by - current_y).abs()) + 1;
                current_x = new_x;
                continue;
            }
                println!("{}", current_x as i64 * 4000000 as i64 + current_y as i64);
                break;
        }
        current_y += 1;
    }
}
