use core::panic;
use std::collections::{HashMap, HashSet};
fn main() {
    let start_time = std::time::Instant::now();
    let input = include_str!("input.txt");
        // let input = "((2 + 4 * 9) * (6 + 9 * 8 + 6) + 6) + 2 + 4 * 2";
    let res: isize = input.lines().map(|s| evaluateExpression1(s)).sum();
    println!("{}", res);
    // for line in input.lines() {
    //     println!("{}", evaluateExpression2(line));
    // }
}
fn evaluateExpression(s: &str) -> isize {
    let mut acc = 0isize;
    let mut operator = None;
    let mut c = 0;
    while (c < s.len()) {
        match s.chars().nth(c).unwrap() {
            '0'..='9' => {
                let endC = s[c..]
                    .chars()
                    .position(|c| c < '0' || c > '9')
                    .unwrap_or(s.len() - c);
                let operand = s[c..c + endC].parse::<isize>().unwrap();
                match operator {
                    None => acc = operand,
                    Some('+') => acc = acc + operand,
                    Some('*') => acc = acc * operand,
                    _ => panic!(),
                }
                c += endC + 1;
            }
            '+' | '*' => {
                operator = Some(s.chars().nth(c).unwrap());
                c += 2;
            }
            '(' => {
                let mut openParen = 0;
                let mut endC = c + 1;
                for pos in c + 1..s.len() {
                    match s.chars().nth(pos).unwrap() {
                        '(' => {
                            openParen += 1;
                            continue;
                        }
                        ')' => {
                            if openParen == 0 {
                                endC = pos;
                                break;
                            }
                            openParen -= 1;
                            continue;
                        }
                        _ => continue,
                    }
                    panic!();
                }
                let operand = evaluateExpression(&s[c + 1..endC]);
                match operator {
                    None => acc = operand,
                    Some('+') => acc = acc + operand,
                    Some('*') => acc = acc * operand,
                    _ => panic!(),
                }
                c = endC + 1;
            }
            ' ' => c += 1,
            _ => panic!(),
        }
    }
    return acc;
}

fn evaluateExpression1(s: &str) -> isize {
    let mut newSiginificantPos = 0;
    let mut significance = 0;
    let mut openParen = 0;
    let mut operand = None;
    for significantPos in 0..s.len() {
        let ch = s.chars().nth(significantPos).unwrap();
        match ch {
            '*' => {
                if openParen == 0 {
                    operand = Some(ch);
                    significance = 2;
                    newSiginificantPos = significantPos;
                }
            }
            '+' => {
                if openParen == 0 {
                    operand = Some(ch);
                    significance = 2;
                    newSiginificantPos = significantPos;
                }
            }
            '(' => {
                if (openParen == 0) && significance < 1 {
                    significance = 1;
                    newSiginificantPos = significantPos;
                }
                openParen += 1;
            }
            ')' => {
                openParen -= 1;
            }
            _ => continue,
        }
    }
    if significance == 0 {
        return s.parse().unwrap();
    }
    if significance == 1 {
        return evaluateExpression2(&s[1..s.len() - 1]);
    }
    if significance == 2 {
        match operand {
            Some('+') =>         return evaluateExpression2(&s[0..newSiginificantPos - 1])
            + evaluateExpression2(&s[newSiginificantPos + 2..s.len()]),
            Some('*') =>         return evaluateExpression2(&s[0..newSiginificantPos - 1])
            * evaluateExpression2(&s[newSiginificantPos + 2..s.len()]),
            _=>panic!(),
        }
    }
    panic!();
}

fn evaluateExpression2(s: &str) -> isize {
    let mut newSiginificantPos = 0;
    let mut significance = 0;
    let mut openParen = 0;
    for significantPos in 0..s.len() {
        let ch = s.chars().nth(significantPos).unwrap();
        match ch {
            '*' => {
                if openParen == 0 && significance < 3 {
                    significance = 3;
                    newSiginificantPos = significantPos;
                }
            }
            '+' => {
                if openParen == 0 && significance < 2 {
                    significance = 2;
                    newSiginificantPos = significantPos;
                }
            }
            '(' => {
                if (openParen == 0) && significance < 1 {
                    significance = 1;
                    newSiginificantPos = significantPos;
                }
                openParen += 1;
            }
            ')' => {
                openParen -= 1;
            }
            _ => continue,
        }
    }
    if significance == 0 {
        return s.parse().unwrap();
    }
    if significance == 1 {
        return evaluateExpression2(&s[1..s.len() - 1]);
    }
    if significance == 2 {
        return evaluateExpression2(&s[0..newSiginificantPos - 1])
            + evaluateExpression2(&s[newSiginificantPos + 2..s.len()]);
    }
    if significance == 3 {
        return evaluateExpression2(&s[0..newSiginificantPos - 1])
            * evaluateExpression2(&s[newSiginificantPos + 2..s.len()]);
    }
    panic!();
}
