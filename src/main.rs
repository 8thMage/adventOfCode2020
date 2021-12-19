use core::panic;
use std::collections::*;
mod helpers;
use helpers::*;
fn main() {
    let input = include_str!("input.txt");
    let mut array: Vec<Vec<i32>> = input
        .lines()
        .map(|s| s.chars().map(|c| c as i32 - '0' as i32).collect())
        .collect();
    let delta = [
        (1, 1),
        (1, 0),
        (1, -1),
        (0, 1),
        (0, -1),
        (-1, 1),
        (-1, 0),
        (-1, -1),
    ];
    let mut sum = 0;
    let mut step = 0;
    loop {
        let mut shined = std::collections::hash_set::HashSet::new();
        let mut need_to_shine = std::collections::hash_set::HashSet::new();
        for (y, l) in array.iter_mut().enumerate() {
            for (x, o) in l.iter_mut().enumerate() {
                *o += 1;
                if (*o > 9) {
                    need_to_shine.insert((y, x));
                }
            }
        }
        while !need_to_shine.is_empty() {
            let &(y, x) = need_to_shine.iter().next().unwrap();
            need_to_shine.remove(&(y, x));
            shined.insert((y, x));
            for d in delta {
                let newY = y as i64 + d.0;
                let newX = x as i64 + d.1;
                if (newY >= 0 && newX >= 0 && (newY as usize) < array.len() && (newX as usize) < array[0].len()) {
                    array[newY as usize][newX as usize] += 1;
                    if array[newY as usize][newX as usize] >= 10 && shined.get(&(newY as usize, newX as usize)) == None {
                        need_to_shine.insert((newY as usize, newX as usize));
                    }
                }
            }
            // for (y, l) in array.iter_mut().enumerate() {
            //     for (x, o) in l.iter_mut().enumerate() {
            //         print!("{} ",o);
            //     }
            //     println!("");
            // }
            // println!("");

        }
        sum += shined.len();
                    // for (y, l) in array.iter_mut().enumerate() {
            //     for (x, o) in l.iter_mut().enumerate() {
            //         print!("{} ",o);
            //     }
            //     println!("");
            // }
            // println!("");

        if (shined.len() == array.len() * array[0].len()) {
            println!("{}", step + 1);
        }
        for (y, x) in shined {
            array[y][x] = 0;
        }

        step += 1;
    }
    for (y, l) in array.iter_mut().enumerate() {
        for (x, o) in l.iter_mut().enumerate() {
            print!("{}",o);
        }
        println!("");
    }
    println!("{}", sum);

}

fn sizeOfBasin(
    x: usize,
    y: usize,
    map: &Vec<Vec<usize>>,
    set: &mut std::collections::HashSet<(usize, usize)>,
) {
    let current = map[y][x];
    if y > 0 && map[y - 1][x] > current && map[y - 1][x] < 9 && !set.contains(&(x, y - 1)) {
        set.insert((x, y - 1));
        sizeOfBasin(x, y - 1, map, set);
    }
    if x > 0 && map[y][x - 1] > current && map[y][x - 1] < 9 && !set.contains(&(x - 1, y)) {
        set.insert((x - 1, y));
        sizeOfBasin(x - 1, y, map, set);
    }
    if !set.contains(&(x + 1, y))
        && x < map[0].len() - 1
        && map[y][x + 1] > current
        && map[y][x + 1] < 9
    {
        set.insert((x + 1, y));
        sizeOfBasin(x + 1, y, map, set);
    }
    if !set.contains(&(x, y + 1))
        && y < map.len() - 1
        && map[y + 1][x] > current
        && map[y + 1][x] < 9
    {
        set.insert((x, y + 1));
        sizeOfBasin(x, y + 1, map, set);
    }
}
