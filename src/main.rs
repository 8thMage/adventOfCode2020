use std::{
    cmp::Ordering,
    collections::{HashMap, HashSet, VecDeque},
    hash::Hash,
    iter, path,
    str::FromStr,
    thread::current,
    vec,
};

fn main() {
    let input = include_str!("input.txt");
    let array = input
        .lines()
        .map(|s| i64::from_str(s).unwrap() * 811589153)
        .collect::<Vec<_>>();
    // let array = vec![1, 2, -3, 0, 3, 7, -2];
    let mut current_array = array.iter().enumerate().collect::<Vec<_>>();
    let len = array.len() - 1;
    let arr_len = array.len();

    for _ in 0..10 {
        for i in 0..array.len() {
            // let element = 7;
            let position = current_array.iter().position(|&e| e.0 == i).unwrap();
            let mut new_position =
                ((current_array[position].1 % len as i64) + position as i64) % len as i64;
            if new_position < 0 {
                new_position += len as i64;
            }
            let element = current_array[position];
            current_array.remove(position);
            current_array.insert(new_position as usize, element);
        }
    }
    // assert!(array.iter().all(|e| current_array.contains(e)));
    let position_of_zero = current_array.iter().position(|&e| *e.1 == 0).unwrap();
    let first = current_array[(position_of_zero + 1000) % array.len()].1;
    let second = current_array[(position_of_zero + 2000) % array.len()].1;
    let third = current_array[(position_of_zero + 3000) % array.len()].1;
    println!("{}", first + second + third);
}
