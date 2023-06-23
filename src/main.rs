use std::{
    collections::{HashMap, HashSet, VecDeque},
    str::FromStr,
};

fn main() {
    let input = include_str!("input.txt");
    let mut pos = vec![(0i32, 0i32); 10];
    let mut visited = HashSet::new();
    for line in input.lines() {
        let (dir, number) = line.split_once(" ").unwrap();
        for _ in 0..u32::from_str(number).unwrap() {
            match dir {
                "R" => {
                    pos[0].0 += 1;
                }
                "L" => pos[0].0 -= 1,
                "U" => pos[0].1 -= 1,
                "D" => pos[0].1 += 1,
                _ => panic!(),
            }
            for i in 0..pos.len() - 1 {
                if (pos[i].0 - pos[i + 1].0).abs() > 1 || (pos[i].1 - pos[i+1].1).abs() > 1 {
                    pos[i + 1].0 += (pos[i].0 - pos[i + 1].0).signum();
                    pos[i + 1].1 += (pos[i].1 - pos[i + 1].1).signum();
                }
            }
            visited.insert(pos[pos.len() - 1]);
        }
    }
    println!("{}", visited.len());
}
