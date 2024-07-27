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
    // (t - x) * x > d
    // -x^2 +tx - d > 0
    // (t +- sqrt(t^2 -4d)) / 2
    let mut iter = input.lines().map(|s| {
        s.split(" ")
            .skip(1)
            .filter(|s| !s.is_empty())
            .map(|s| u64::from_str(s).unwrap())
    });
    let time = iter.next().unwrap();
    let dist = iter.next().unwrap();
    let p: u64 = time
        .zip(dist)
        .map(|(time, dist)| {
            let first_intersection =
                ((time as f64 - ((time * time - 4 * dist) as f64).sqrt()) / 2.).floor() as u64;
            let second_intersection =
                ((time as f64 + ((time * time - 4 * dist) as f64).sqrt()) / 2.).ceil() as u64;
            let mut diff = second_intersection - first_intersection + 1;
            if (time - first_intersection) * first_intersection <= dist {
                diff -= 1
            }
            if (time - second_intersection) * second_intersection <= dist {
                diff -= 1
            }
            diff
        })
        .product();
    println!("{}", p);
    let mut iter2 = input.lines().map(|s| {
        u64::from_str(
            s.split(" ")
                .skip(1)
                .filter(|s| !s.is_empty())
                .fold(String::new(), |acc, s| acc + s)
                .as_str(),
        )
        .unwrap()
    });
    let time = iter2.next().unwrap();
    let dist = iter2.next().unwrap();

    let p: u64 = {
        let first_intersection =
            ((time as f64 - ((time * time - 4 * dist) as f64).sqrt()) / 2.).floor() as u64;
        let second_intersection =
            ((time as f64 + ((time * time - 4 * dist) as f64).sqrt()) / 2.).ceil() as u64;
        let mut diff = second_intersection - first_intersection + 1;
        if (time - first_intersection) * first_intersection <= dist {
            diff -= 1
        }
        if (time - second_intersection) * second_intersection <= dist {
            diff -= 1
        }
        diff
    };
    println!("{}", p);
}
