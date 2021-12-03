use core::panic;
use std::collections::*;
mod helpers;
use helpers::*;
fn main() {
    let input = include_str!("input.txt");
    let numbers = input.lines().map(|s| s.parse::<i32>().unwrap()).collect::<Vec<i32>>();
    let mut acc = 0;
    for (i, _) in numbers.iter().enumerate() {
        if ( i >= 3 && numbers[i] > numbers[i - 3]) {
            acc +=1;
        }
    }
    print!("{}",acc);
}