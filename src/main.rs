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
    let mut global_sum = 0;
    for line in input.lines() {
        let mut sum = 0;
        let mut mul = 1;
        for char in line.chars().rev() {
            match char {
                '0' => {}
                '1' => sum += mul,
                '2' => sum += 2 * mul,
                '-' => sum -= mul,
                '=' => sum -= 2 * mul,
                _ => unreachable!(),
            }
            mul *= 5;
        }
        assert!(line == to_special(sum));
        global_sum += sum;
    }
    println!("{}", to_special(global_sum));
}

fn to_special(mut s: i64) -> String {
    let mut vec = vec![];
    let mut remainder = 0;
    while s != 0 {
        let digit = (s % 5) + remainder;
        if digit >= 3 {
            remainder = 1;
        } else {
            remainder = 0;
        }
        vec.push(match digit {
            0 => '0',
            1 => '1',
            2 => '2',
            3 => '=',
            4 => '-',
            5 => '0',
            6 => '1',
            _ => unreachable!(),
        });
        s /= 5;
    }
    if remainder == 1 {
        vec.push('1');
    }
    vec.reverse();
    vec.iter().fold(String::new(), |mut s, c| {
        s.push(*c);
        s
    })
}
