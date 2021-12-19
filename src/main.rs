use core::panic;
use std::collections::*;
mod helpers;
use helpers::*;
fn main() {
    let input = include_str!("input.txt");
    let mut points = HashMap::new();
    points.insert(')', 3);
    points.insert(']', 57);
    points.insert('}', 1197);
    points.insert('>', 25137);
    let mut sum = 0;
    let mut newScores = vec![];
    'line: for line in input.lines() {
        let mut stack = vec![];
        for c in line.chars() {
            match c {
                '(' | '{' | '[' | '<' => {
                    stack.push(c);
                }
                ')' => {
                    let d = stack.pop();
                    if d.unwrap_or('(') != '(' {
                        sum += points[&')'];
                        continue 'line;
                    }
                }
                '}' => {
                    let d = stack.pop();
                    if d.unwrap_or('{') != '{' {
                        sum += points[&'}'];
                        continue 'line;
                    }
                }
                ']' => {
                    let d = stack.pop();
                    if d.unwrap_or('[') != '[' {
                        sum += points[&']'];
                        continue 'line;
                    }
                }
                '>' => {
                    let d = stack.pop();
                    if d.unwrap_or('<') != '<' {
                        sum += points[&'>'];
                        continue 'line;
                    }
                }
                _ => {
                    panic!();
                }
            }
        }
        let mut newSum = 0i64;
        for s in stack.iter().rev() {
            newSum *= 5;
            newSum +=
                match s {
                    '(' => 1,
                    '[' => 2,
                    '{' => 3,
                    '<' => 4,
                    _ => 0
                };
            // println!("{}", newSum);
     
        }
        newScores.push(newSum);
    }
    println!("{}", sum);
    newScores.sort();
    println!("{}", newScores[newScores.len() / 2]);

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
