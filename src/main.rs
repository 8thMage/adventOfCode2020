use core::panic;
use std::collections::*;
mod helpers;
use helpers::*;
fn main() {
    let input = include_str!("input.txt");
    let mut lines = input.lines();
    let mut firstLine = lines.next().unwrap();
    lines.next();
    let mut hash_map = HashMap::new();
    for line in lines {
        let (pair, res) = line.split_once(" -> ").unwrap();
        let mut sum = vec![0; 26];
        // sum[res.chars().next().unwrap() as usize - 'A' as usize] = 1;
        sum[pair.chars().next().unwrap()as usize - 'A' as usize] = 1;
        hash_map.insert(
            (
                pair.chars().next().unwrap(),
                pair.chars().skip(1).next().unwrap(),
                0,
            ),
            sum,
        );
    }
    let mut rule = HashMap::new();
    for line in input.lines().skip(2) {
        let (pair, res) = line.split_once(" -> ").unwrap();
        rule.insert(
            (
                pair.chars().next().unwrap(),
                pair.chars().skip(1).next().unwrap(),
            ),
            res.chars().next().unwrap(),
        );
    }
    let stages = 40 ;
    let mut sum = vec![0; 26];
    for pair in firstLine.chars().zip(firstLine.chars().skip(1)) {
        bfs(&(pair.0, pair.1), stages, &rule, &mut hash_map);
    }
    for pair in firstLine.chars().zip(firstLine.chars().skip(1)) {
        let a = &hash_map[&(pair.0, pair.1, stages)];
        sum = a.iter().zip(sum.iter()).map(|(a, b)| a + b).collect();
    }
    sum[firstLine.chars().last().unwrap() as usize - 'A' as usize] += 1;
    for s in &sum {
        print!("{} ", s);
    }
    println!("");
    let minSum = sum.iter().filter(|&&a| a != 0).min().unwrap();
    let maxSum = sum.iter().filter(|&&a| a != 0).max().unwrap();
    println!("{}", maxSum - minSum);
}

fn bfs(
    string: &(char, char),
    layer: i32,
    rule: &HashMap<(char, char), char>,
    hash_map: &mut HashMap<(char, char, i32), Vec<i64>>,
) {
    println!("{}", layer);
    println!("{} {}", string.0, string.1);
    if let Some(arr) = hash_map.get(&(string.0, string.1, layer)) {
        return;
    } else {
        let mut newString: (char, char) = ('a', 'a');
        newString.0 = string.0;
        newString.1 = rule[&string];
        bfs(&newString, layer - 1, rule, hash_map);
        let mut newString2: (char, char) = ('a', 'a');
        newString2.0 = rule[&string];
        newString2.1 = string.1;
        bfs(&newString2, layer - 1, rule, hash_map);
        let a = hash_map
            .get(&(newString.0, newString.1, layer - 1))
            .unwrap();
        for s in a {
            print!("{} ", s);
        }
        println!("");

        let b = hash_map
            .get(&(newString2.0, newString2.1, layer - 1))
            .unwrap();
        for s in b {
            print!("{} ", s);
        }
        println!("");

        let mut v: Vec<i64> = a.iter().zip(b.iter()).map(|(a, b)| a + b).collect();
        // v[string.0 as usize - 'A' as usize] += 1;
        for s in &v {
            print!("{} ", s);
        }
        println!("");
        hash_map.insert((string.0, string.1, layer), v);
    }
}
