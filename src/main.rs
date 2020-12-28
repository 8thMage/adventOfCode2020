use core::panic;
use std::collections::{btree_map::Iter, HashMap, HashSet, VecDeque};
fn main() {
    let start_time = std::time::Instant::now();
    let input = include_str!("input.txt");
    let time = std::time::Instant::now();
    let mut coordinates = (0,0); 
    let mut flipped:HashSet<(isize, isize)> = HashSet::new();
    for line in input.lines() {
        coordinates = (0,0); 
        let chars_vector:Vec::<char> = line.chars().collect();
        let mut start = 0;
        while start < chars_vector.len() {
            if chars_vector[start] == 's' {
                coordinates.1 -= 1;
                if chars_vector[start + 1] == 'e' {
                    coordinates.0 += 1;
                }
                start += 2;
                continue;
            }
            if chars_vector[start] == 'n' {
                coordinates.1 += 1;
                if chars_vector[start + 1] == 'w' {
                    coordinates.0 -= 1;
                }
                start += 2;
                continue;
            }
            if chars_vector[start] == 'w' {
                coordinates.0 -= 1
            }
            if chars_vector[start] == 'e' {
                coordinates.0 += 1;
            }
            start += 1;
        }
        if flipped.contains(&(coordinates)) {
            flipped.remove(&(coordinates));
        } else {
            flipped.insert(coordinates);
        }
    }
    println!("{}", flipped.len());

    for turn in 0..100 {
        let mut new_flipped = HashSet::new();
        let mut maxX = isize::MIN;
        let mut maxY = isize::MIN;
        let mut minX = isize::MAX;
        let mut minY = isize::MAX;
        for v in &flipped {
            maxX = maxX.max(v.0);
            minX = minX.min(v.0);
            maxY = maxY.max(v.1);
            minY = minY.min(v.1);
        }
        for x in minX-1..maxX + 2 {
            for y in minY-1..maxY + 2 {
                let mut countBlack = 0;
                countBlack += flipped.contains(&(x - 1, y)) as isize;
                countBlack += flipped.contains(&(x + 1, y)) as isize;
                countBlack += flipped.contains(&(x - 1, y + 1)) as isize;
                countBlack += flipped.contains(&(x + 1, y - 1)) as isize;
                countBlack += flipped.contains(&(x, y + 1)) as isize;
                countBlack += flipped.contains(&(x, y - 1)) as isize;
                let c = flipped.contains(&(x, y));
                if c && countBlack > 0 && countBlack <= 2 {
                    new_flipped.insert((x, y));
                } else if !c && countBlack == 2 {
                    new_flipped.insert((x, y));
                }
            }
        }

        flipped = new_flipped;
        println!("{}", flipped.len());

    }
}
