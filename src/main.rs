use core::panic;
use std::collections::*;
mod helpers;
use helpers::*;
fn main() {
    let input = include_str!("input.txt");
    let mut depth = 0;
    let mut horizontal = 0;
    let mut aim = 0;
    let numbers = input.lines().for_each(|s|
        if (s.starts_with("forward ")) {
            let x = s.trim_start_matches("forward ").parse::<i32>().unwrap();
            horizontal += x;
            depth += aim * x;
        }
        else if (s.starts_with("up ")) {
            aim -= s.trim_start_matches("up ").parse::<i32>().unwrap();
        }
        else if (s.starts_with("down ")) {
            aim += s.trim_start_matches("down ").parse::<i32>().unwrap();
        }
    );
    print!("{}", depth * horizontal);
}