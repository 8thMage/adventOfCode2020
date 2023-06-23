use std::{
    collections::{HashMap, HashSet, VecDeque},
    str::FromStr,
};

fn main() {
    let input = include_str!("input.txt");
    let trees: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();
    let mut sum = 0;
    let mut max = 0;
    for y in 0..trees.len() {
        for x in 0..trees[0].len() {
            let tree_height = trees[y][x];
            let mut tests = vec![true; 4];
            let mut sums = vec![0; 4];
            for y1 in (0..y).rev() {
                sums[0] += 1;
                if trees[y1][x] >= tree_height {
                    tests[0] = false;
                    break;
                }
            }

            for y1 in y + 1..trees.len() {
                sums[1] += 1;

                if trees[y1][x] >= tree_height {
                    tests[1] = false;
                    break;
                }
            }
            for x1 in (0..x).rev() {
                sums[2] += 1;

                if trees[y][x1] >= tree_height {
                    tests[2] = false;
                    break;
                }
            }
            for x1 in x + 1..trees.len() {
                sums[3] += 1;

                if trees[y][x1] >= tree_height {
                    tests[3] = false;
                    break;
                }
            }
            if tests.iter().any(|x| *x) {
                sum += 1;
            }
            max = sums.iter().fold(1, |x, y| y * x).max(max);
        }
    }
    println!("{}", sum);
    println!("{}", max);
}
