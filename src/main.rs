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
    let mut sum = 0;
    // for line in input.lines() {
    //     let (springs, numbers) = line.split_once(' ').unwrap();
    //     let numbers = numbers
    //         .split(',')
    //         .map(|s| usize::from_str_radix(s, 10).unwrap())
    //         .collect::<Vec<_>>();
    //     let springs = springs.chars().map(|c| c as u8).collect::<Vec<_>>();
    //     let count = springs.iter().filter(|c| **c == b'?').count();
    //     for index in 0..1<<count {
    //         let mut new_springs = springs.clone();
    //         let mut curr = 0;
    //         for i in new_springs.iter_mut() {
    //             if *i == b'?' {
    //                 if (index >> curr) & 1 == 1 {
    //                     *i = b'#';
    //                 } else {
    //                     *i = b'.';
    //                 }
    //                 curr += 1;
    //             }
    //         }
    //         let string = String::from_utf8(new_springs).unwrap();
    //         let groups = string
    //             .split('.')
    //             .map(|s| s.len())
    //             .filter(|s| *s != 0)
    //             .collect::<Vec<_>>();
    //         if groups == numbers {
    //             sum += 1;
    //         }
    //     }
    // }
    println!("{}", sum);
    for line in input.lines() {
        println!("{}", sum);
        let (springs, numbers) = line.split_once(' ').unwrap();
        let numbers = numbers
            .split(',')
            .map(|s| usize::from_str_radix(s, 10).unwrap())
            .collect::<Vec<_>>();
        let numbers = numbers.repeat(5);
        let mut springs = springs.chars().map(|c| c as u8).collect::<Vec<_>>();
        springs.push(b'?');
        let mut springs = springs.repeat(5);
        springs.pop();
        let string = String::from_utf8(springs).unwrap();
        sum += option_starting_from_position(&mut HashMap::new(), string.as_str(), &numbers);
    }
    println!("{}", sum);
}

fn option_starting_from_position(
    known: &mut HashMap<(String, Vec<usize>), u64>,
    s: &str,
    numbers: &[usize],
) -> u64 {
    let s = s.trim_start_matches('.');
    let first_split = s.split_once('.');
    if first_split.is_none() || numbers.is_empty() {
        return options_curr_split(known, s, numbers);
    }
    let (start, tail) = first_split.unwrap();
    if let Some((prefix, _)) = start.split_once('?') {
        if prefix.len() > numbers[0] {
            return 0;
        }
    }
    let max_len = start.len();
    let mut curr_len = u64::MAX;
    let mut current_sum = 0;
    for split_numbers in 0..=numbers.len() {
        let options_curr_split = options_curr_split(known, start, &numbers[0..split_numbers]);
        if options_curr_split != 0 {
            if split_numbers == numbers.len() {
                if !tail.contains('#') {
                    current_sum += options_curr_split;
                }
            } else {
                current_sum += options_curr_split
                    * option_starting_from_position(known, tail, &numbers[split_numbers..]);
            }
        }
        if split_numbers == numbers.len() {
            break;
        }
        curr_len = curr_len.wrapping_add(numbers[split_numbers] as u64 + 1);
        if curr_len > max_len as u64 {
            break;
        }
    }
    return current_sum;
}

fn options_curr_split(
    known: &mut HashMap<(String, Vec<usize>), u64>,
    s: &str,
    numbers: &[usize],
) -> u64 {
    let v = Vec::from(numbers);
    let string = String::from_str(s).unwrap();
    if let Some(known) = known.get(&(string, v)) {
        return *known;
    }
    if numbers.is_empty() {
        return !s.contains('#') as u64;
    }
    if s.len() == numbers[0] {
        return (numbers.len() == 1) as u64;
    }
    if s.len() < numbers[0] {
        return 0;
    }
    let mut curr_sum = 0;
    for start in 0..=s.len() - numbers[0] {
        if start > 0 && s.chars().nth(start - 1).unwrap() != '?' {
            break;
        }
        if s.chars()
            .nth(numbers[0] + start)
            .map_or(false, |c| c == '#')
        {
            continue;
        }
        if s.len() <= start + numbers[0] + 1 {
            curr_sum += (numbers.len() == 1) as u64;
        } else {
            curr_sum += options_curr_split(known, &s[start + numbers[0] + 1..], &numbers[1..]);
        }
    }
    let v = Vec::from(numbers);
    let string = String::from_str(s).unwrap();

    known.insert((string, v), curr_sum);
    return curr_sum;
}
