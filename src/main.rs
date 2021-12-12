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
    let mut new_fish = vec![];
    let days = 256;
    new_fish.resize(days + 9, 0i64);
    let mut j = days + 9 - 1;
    while j >= 0 {
        let mut sum = 0i64;
        let mut i = j + 9;
        while i < new_fish.len() {
            sum += 1;
            sum += new_fish[i];
            i+=7;
        }
        new_fish[j] = sum;
        if (j == 0){
            break;
        }
        j-=1;
    }
    let mut sum = 0i64;
    for fish in nums {
        sum += new_fish[fish as usize] + 1;
    }
    println!("{}", sum);
}

fn same_slope(a: &Vec<i64>, b: &Vec<i64>) -> bool {
    return (a[0] - a[2]) * (b[1] - b[3]) - (b[0] - b[2]) * (a[1] - a[3]) == 0;
}
