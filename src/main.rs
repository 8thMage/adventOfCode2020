use core::panic;
use std::collections::*;
mod helpers;
use helpers::*;
fn main() {
    let input = include_str!("input.txt");
    // let lines = input.lines().map(|s| s.split(" | ").skip(1).next().unwrap()).flat_map(|s| s.split(" ").filter(|s| s.len() == 2 || s.len() == 3|| s.len() == 4|| s.len() == 7));
    // println!("{}", lines.count());
    let validMappings = vec!["abcefg", "cf", "acdeg", "acdfg", "bcdf", "abdfg", "abdefg", "acf", "abcdefg", "abcdfg"];
    let mut allPermutations:Vec<Vec<char>> = vec![];
    permutation(&mut vec!['a', 'b','c','d','e','f', 'g'], 0, &mut allPermutations);
    let mut sum = 0;
    for line in input.lines() {        
        for p in &allPermutations {
        let ok = line.split(" ").filter(|s| *s!= "|").fold(true, |a, s| {
            let mut newString = String::new();
            for c in s.chars() {
                newString.push(p[c as usize - 'a' as usize]);
            }
            for m in &validMappings {
                if newString.len() != m.len() {
                    continue;
                } else {
                    if(m.chars().fold(true, |a, c| newString.contains(c) && a)) {
                        return a;
                    };
                }
            }
            return false;
        });
        if ok {
            let n = line.split(" | ").skip(1).next().unwrap().split(" ").fold(0, |a, s|  {
                let mut newString = String::new();
                for c in s.chars() {
                    newString.push(p[c as usize - 'a' as usize]);
                }
                for (i, m) in validMappings.iter().enumerate() {
                    if newString.len() != m.len() {
                        continue;
                    } else {
                        if m.chars().fold(true, |a, c| newString.contains(c) && a) {
                            return a * 10 + i;
                        }
                    }
                } 
                return 0  
            });
            sum += n;
        }
    }
}
println!("{}", sum);
}

fn permutation(current_permutation:&mut Vec<char>, i:usize, result: &mut Vec<Vec<char>>) {
    if i == current_permutation.len() {
        result.push(current_permutation.clone());
        return;
    }
    permutation(current_permutation, i+1, result);
    for j in i+1..current_permutation.len() {
        current_permutation.swap(i, j);
        permutation(current_permutation, i+1, result);
        current_permutation.swap(i, j);
    }
}