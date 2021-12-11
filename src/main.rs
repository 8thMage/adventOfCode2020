use core::panic;
use std::collections::*;
mod helpers;
use helpers::*;
fn main() {
    let input = include_str!("input.txt");
    let nums: Vec<Vec<char>> = input.lines().map(|s| s.chars().collect()).collect();
    let mut oxygen = nums.clone();
    let mut co2 = nums.clone();
    let mut newnum:i32 = 0;
    let mut newnum_not:i32 = 0;
    for i in 0..nums[0].len() {
        let mut positivity = 0;
        for n in 0..nums.len() {
            if nums[n][i] == '0' {
                positivity -= 1;
            } else if nums[n][i] == '1' {
                positivity += 1;
            }
        }
        if positivity > 0 {
            newnum += 1<<(nums[0].len() - i - 1);
        } else if positivity < 0 {
            newnum_not += 1<<(nums[0].len() - i - 1);
        } else {
            panic!();
        }
    }
    let n = oxygen[0].len();
    for i in 0..n {
        if (oxygen.len() == 1) {
            break;
        }

        let mut positivity = 0;
        for n in 0..oxygen.len() {
            if oxygen[n][i] == '0' {
                positivity -= 1;
            } else if oxygen[n][i] == '1' {
                positivity += 1;
            }
        }
        let bit = if positivity >= 0 {
            '1'
        } else {
            '0'
        }; 
        oxygen = oxygen.into_iter().filter(|s| s[i] == bit).collect();
    }

    for i in 0..n {
        if (co2.len() == 1) {
            break;
        }
        let mut positivity = 0;
        for n in 0..co2.len() {
            if co2[n][i] == '0' {
                positivity -= 1;
            } else if co2[n][i] == '1' {
                positivity += 1;
            }
        }
        let bit = if positivity >= 0 {
            '0'
        } else {
            '1'
        }; 
        co2 = co2.into_iter().filter(|s| s[i] == bit).collect();
    }
    let co2N = co2[0].iter().enumerate().fold(0, |acc, (index, &c)|{ if c == '1' {
        acc + (1<<(n - index - 1))
    } else {
        acc
    }
}
    );
    let oxygenN = oxygen[0].iter().enumerate().fold(0, |acc, (index, &c)|{ if c == '1' {
        acc + (1<<(n - index - 1))
    } else {
        acc
    }
}
    );
    println!("{}", newnum * newnum_not);
    println!("{}", oxygenN * co2N);
}