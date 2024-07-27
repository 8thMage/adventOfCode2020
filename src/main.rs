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
    let mut splitter = input.split("\n\n");
    let seeds = splitter
        .next()
        .unwrap()
        .split(" ")
        .skip(1)
        .map(|s| u64::from_str(s).unwrap());
    let maps = splitter
        .map(|s| {
            s.lines()
                .skip(1)
                .map(|s| {
                    s.split(" ")
                        .map(|s| u64::from_str(s).unwrap())
                        .collect::<Vec<_>>()
                })
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    let min = seeds
        .clone()
        .map(|seed| {
            let mut current = seed;
            for map in &maps {
                current = map
                    .iter()
                    .find(|s| (s[1]..s[1] + s[2]).contains(&current))
                    .map_or(current, |s| s[0] + current - s[1]);
            }
            // println!("{}", current);
            current
        })
        .min()
        .unwrap();
    let mut splitter = input.split("\n\n");

    let new_seeds = splitter
        .next()
        .unwrap()
        .split(" ")
        .skip(1)
        .map(|s| u64::from_str(s).unwrap())
        .collect::<Vec<_>>();
    let mut new_seeds: Vec<[u64; 2]> = new_seeds
        .windows(2)
        .step_by(2)
        .map(|s| [s[0], s[1]])
        .collect::<Vec<_>>();
    let first_sum = new_seeds.iter().map(|s| s[1]).sum::<u64>();
    for map in &maps {
        assert_eq!(new_seeds.iter().map(|s| s[1]).sum::<u64>(), first_sum);
        let mut new_seeds_spliced = vec![];
        for seed in &new_seeds {
            let mut points_in_segment = vec![seed[0], seed[0] + seed[1]];
            let mut found_map = false;
            for specific_map in map {
                if (seed[0]..seed[0] + seed[1]).contains(&specific_map[1]) {
                    points_in_segment.push(specific_map[1]);
                } else if (seed[0]..seed[0] + seed[1])
                    .contains(&(specific_map[1] + specific_map[2]))
                {
                    points_in_segment.push(specific_map[1] + specific_map[2]);
                }
            }
            points_in_segment.sort();
            points_in_segment.dedup();
            for window in points_in_segment.windows(2) {
                new_seeds_spliced.push([window[0], window[1] - window[0]]);
            }
        }
        assert_eq!(new_seeds_spliced.iter().map(|s| s[1]).sum::<u64>(), first_sum);
        new_seeds.clear();
        new_seeds = new_seeds_spliced
            .iter()
            .map(|s| {
                map.iter()
                    .find(|map| (map[1]..map[1] + map[2]).contains(&s[0]))
                    .map_or(*s, |map| {
                        [s[0] - map[1] + map[0], s[1]]
                    })
            })
            .collect();
        }

    println!("{}", min);
    println!("{}", new_seeds.iter().min().unwrap()[0]);
}
