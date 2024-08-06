use core::num;
use std::{
    cmp::Reverse,
    collections::{BinaryHeap, HashMap},
    fmt::Write,
    iter::Cycle,
    str::FromStr,
};

fn main() {
    let input = include_str!("input.txt");
    let mut current = vec![];
    for line in input.lines() {
        current.push(line.chars().collect::<Vec<_>>());
    }
    let mut sum = 0;
    for x in 0..current[0].len() {
        let mut next = current.len();
        for y in 0..current.len() {
            if current[y][x] == 'O' {
                sum += next;
                next -= 1;
            }
            if current[y][x] == '#' {
                next = current.len() - y - 1;
            }
        }
    }
    let mut next = vec![vec!['.'; current.len()]; current.len()];
    for cycle in 0..1000000000 {
        for i in 0..4 {
            next.iter_mut().for_each(|v| v.fill('.'));
            for x in 0..current[0].len() {
                let mut write_next = 0;
                for y in 0..current.len() {
                    if current[y][x] == 'O' {
                        next[write_next][x] = 'O';
                        write_next += 1;
                    }
                    if current[y][x] == '#' {
                        next[y][x] = '#';
                        write_next = y + 1;
                    }
                }
            }
            // current
            //     .iter_mut()
            //     .enumerate()
            //     .for_each(|(y, v)| v.copy_from_slice(&next[y]));
            // next.iter_mut().for_each(|v| v.fill('.'));
            // for y in 0..current[0].len() {
            //     let mut write_next = 0;
            //     for x in 0..current.len() {
            //         if current[y][x] == 'O' {
            //             next[y][write_next] = 'O';
            //             write_next += 1;
            //         }
            //         if current[y][x] == '#' {
            //             next[y][x] = '#';
            //             write_next = x + 1;
            //         }
            //     }
            // }
            // current
            //     .iter_mut()
            //     .enumerate()
            //     .for_each(|(y, v)| v.copy_from_slice(&next[y]));
            // next.iter_mut().for_each(|v| v.fill('.'));
            // for x in 0..current[0].len() {
            //     let mut write_next = current.len() - 1;
            //     for y in (0..current.len()).rev() {
            //         if current[y][x] == 'O' {
            //             next[write_next][x] = 'O';
            //             write_next -= 1;
            //         }
            //         if current[y][x] == '#' {
            //             next[y][x] = '#';
            //             write_next = y.saturating_sub(1);
            //         }
            //     }
            // }
            // current
            //     .iter_mut()
            //     .enumerate()
            //     .for_each(|(y, v)| v.copy_from_slice(&next[y]));
            // next.iter_mut().for_each(|v| v.fill('.'));
            // for y in 0..current[0].len() {
            //     let mut write_next = current.len() - 1;
            //     for x in (0..current.len()).rev() {
            //         if current[y][x] == 'O' {
            //             next[y][write_next] = 'O';
            //             write_next -= 1;
            //         }
            //         if current[y][x] == '#' {
            //             next[y][x] = '#';
            //             write_next = x.saturating_sub(1);
            //         }
            //     }
            // }
            // current
            //     .iter_mut()
            //     .enumerate()
            //     .for_each(|(y, v)| v.copy_from_slice(&next[y]));
            // next.iter_mut().for_each(|v| v.fill('.'));
            // println!("shifted");
            // for y in 0..current.len() {
            //     for x in 0..current.len() {
            //         print!("{}", next[y][x]);
            //     }
            //     println!("");
            // }
            // println!("");
            current.iter_mut().for_each(|v| v.fill('.'));
            let half = (current.len() - 1) as i32;
            for y in 0..current.len() {
                for x in 0..current[0].len() {
                    let dx = (x * 2) as i32 - half;
                    let dy = (y * 2) as i32 - half;
                    current[(half + dx) as usize >> 1][(half - dy) as usize >> 1] =
                        next[(half + dy) as usize >> 1][(half + dx) as usize >> 1]
                }
            }
            // println!("rotated");
            // for y in 0..current.len() {
            //     for x in 0..current.len() {
            //         print!("{}", current[y][x]);
            //     }
            //     println!("");
            // }
            // println!("");
        }
        let mut sum = 0;
        // for y in 0..current.len() {
        //     for x in 0..current.len() {
        //         print!("{}", current[y][x]);
        //     }
        //     println!("");
        // }
        // println!("");
        for x in 0..current[0].len() {
            let mut next = current.len();
            for y in 0..current.len() {
                if current[y][x] == 'O' {
                    sum += current.len() - y;
                }
            }
        }
        println!("{} {}", cycle+1, sum);
    }

    // println!("{}", sum);
}
