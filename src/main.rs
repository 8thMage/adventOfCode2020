use core::panic;
use std::collections::*;
mod helpers;
use helpers::*;
fn main() {
    let input = include_str!("input.txt");
    let mut nums = input
        .split(",")
        .map(|s| s.parse::<i32>().unwrap())
        .collect::<Vec<i32>>();
    let average:i32 = nums.iter().sum::<i32>() / nums.len() as i32;
    // println!("{}", average);
    let mut min = i64::max_value();
    for i in 0.. *nums.iter().max().unwrap() {
        let newSum = nums.iter().fold(0i64, |a, s| a + ((s - i).abs() * ((s - i).abs() + 1)) as i64 /2 );
        if newSum < min {
            min = newSum;
        }
    }
    println!("{}", min);
}

fn same_slope(a: &Vec<i64>, b: &Vec<i64>) -> bool {
    return (a[0] - a[2]) * (b[1] - b[3]) - (b[0] - b[2]) * (a[1] - a[3]) == 0;
}
