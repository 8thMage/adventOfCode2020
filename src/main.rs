use std::{
    cmp::Ordering,
    collections::{hash_map, HashMap, HashSet, VecDeque},
    env::current_exe,
    hash::Hash,
    iter,
    ops::{Neg, RangeBounds},
    path,
    str::FromStr,
    thread::current,
    vec,
};

fn main() {
    let input = include_str!("input.txt");
    let mut vec = input
        .lines()
        .map(|s| {
            let a = s.split_once(" ").unwrap();
            (
                a.0.chars()
                    .map(|c| {
                        vec![
                            'A', 'K', 'Q', 'T', '9', '8', '7', '6', '5', '4', '3', '2', 'J',
                        ]
                        .iter()
                        .position(|a| *a == c)
                        .map(|i| 12 - i)
                        .unwrap()
                    })
                    .collect::<Vec<_>>(),
                u64::from_str(a.1).unwrap(),
            )
        })
        .collect::<Vec<_>>();
    vec.sort_by(|a, b| {
        let ordering = max_hand_type(&a.0).cmp(&max_hand_type(&b.0));
        if ordering == Ordering::Equal {
            a.0.cmp(&b.0)
        } else {
            ordering
        }
    });
    let s = vec
        .iter()
        .enumerate()
        .map(|s| (s.0 as u64 + 1) * s.1 .1)
        .sum::<u64>();
    for line in vec {
        println!("{:?}", line);
    }
    println!("{:?}", s);
}

fn max_hand_type(s: &Vec<usize>) -> i64 {
    (1..13)
        .map(|c| {
            let mut k = s.clone();
            k.iter_mut().for_each(|l| {
                if *l == 0 {
                    *l = c
                }
            });
            hand_type(&k)
        })
        .max()
        .unwrap()
}

fn hand_type(s: &Vec<usize>) -> i64 {
    let mut vec = s.clone();
    vec.sort();
    let mut current_num = 0;
    let mut positions = vec
        .windows(2)
        .map(|s| s[0] == s[1])
        .enumerate()
        .filter_map(|s| (!s.1).then_some(s.0 + 1))
        .collect::<Vec<_>>();
    positions.push(0);
    positions.push(vec.len());
    positions.sort();
    let mut lengths = positions
        .windows(2)
        .map(|s| s[1] - s[0])
        .collect::<Vec<_>>();
    lengths.sort_by_key(|s| -(*s as i64));
    if lengths[0] == 5 {
        return 27;
    }
    if lengths[0] == 4 {
        return 26;
    }
    if lengths[0] == 3 && lengths[1] == 2 {
        return 25;
    }
    if lengths[0] == 3 {
        return 24;
    }
    if lengths[0] == 2 && lengths[1] == 2 {
        return 23;
    }
    if lengths[0] == 2 {
        return 22;
    }
    return 21;
}
