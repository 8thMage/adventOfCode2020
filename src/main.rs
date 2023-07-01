use std::{
    collections::{HashMap, HashSet, VecDeque},
    hash::Hash,
    str::FromStr,
    vec,
};

fn main() {
    let input = include_str!("input.txt");
    let mut stone = HashSet::new();
    let mut max_y = 0;
    for line in input.lines() {
        let mut last_coord = None;
        for coords in line.split(" -> ") {
            let coords = coords.split_once(",").unwrap();
            let x = i32::from_str(coords.0).unwrap();
            let y = i32::from_str(coords.1).unwrap();
            max_y = max_y.max(y);
            if let Some((x0, y0)) = last_coord {
                if x != x0 {
                    for x1 in x.min(x0)..=x0.max(x) {
                        stone.insert((x1, y0));
                    }
                }
                if y != y0 {
                    for y1 in y.min(y0)..=y0.max(y) {
                        stone.insert((x0, y1));
                    }
                }
            }
            last_coord = Some((x, y));
        }
    }

    let mut path = vec![(500, 0)];
    let mut count = 0;
    loop {
        let Some(&(x,y)) = path.last() else {
            break;
        };
        if stone.contains(&(x, y)) {
            path.pop();
            continue;
        }
        // if y >= max_y {
        //     break;
        // }
        if y < max_y + 1 {
            if !stone.contains(&(x, y + 1)) {
                path.push((x, y + 1));
                continue;
            }
            if !stone.contains(&(x - 1, y + 1)) {
                path.push((x - 1, y + 1));
                continue;
            }
            if !stone.contains(&(x + 1, y + 1)) {
                path.push((x + 1, y + 1));
                continue;
            }
        }
        count += 1;
        stone.insert((x, y));
    }
    println!("{}", count);
}
