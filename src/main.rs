use std::{
    cmp::Ordering,
    collections::{hash_map, HashMap, HashSet, VecDeque},
    hash::Hash,
    iter,
    ops::Neg,
    path,
    str::FromStr,
    thread::current,
    vec,
};

fn main() {
    let input = include_str!("input.txt");
    let value = input
        .lines()
        .enumerate()
        .map(|(index, line)| {
            let game = line.split_once(": ").unwrap().1;
            let subsets = game.split("; ");
            let mut valid = true;
            let mut min_blue = 0;
            let mut min_red = 0;
            let mut min_green = 0;

            for set in subsets {
                let cubes = set.split(", ");
                for cube in cubes {
                    let (num, color) = cube.split_once(" ").unwrap();
                    let num = i32::from_str(num).unwrap();
                    match color {
                        "blue" => {
                            min_blue = min_blue.max(num);
                            valid &= num <= 14;
                        }
                        "green" => {
                            min_green = min_green.max(num);
                            valid &= num <= 13;
                        }
                        "red" => {
                            min_red = min_red.max(num);
                            valid &= num <= 12;
                        }
                        _ => unreachable!(),
                    }
                }
            }
            min_blue * min_green * min_red
            // if valid {
            //     index + 1
            // } else {
            //     0
            // }
        })
        .sum::<i32>();
    println!("{}", value)
}
