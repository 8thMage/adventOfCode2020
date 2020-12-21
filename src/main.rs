use core::panic;
use std::collections::{HashMap, HashSet};
fn main() {
    let start_time = std::time::Instant::now();
    let input = include_str!("input.txt");

    let rulesStr = input
        .lines()
        .filter(|s| s.contains(":"))
        .map(|s| (s.split_terminator(": ").nth(0).unwrap().parse::<usize>().unwrap(), s.split_terminator(": ").nth(1).unwrap()))
        .map(|(s1, s2)| (s1, s2.split_terminator("| ").collect::<Vec<&str>>()))
        .collect::<HashMap<usize, Vec<&str>>>();
    let messages = input.lines().filter(|s| !s.is_empty() && !s.contains(":"));
    let count = messages.filter(|message| messageFits(message, &rulesStr, 0).contains(&message.len())).count();
    println!("{}", count);
}

fn messageFits(message: &str, rules: &HashMap<usize, Vec<&str>>, i: usize) -> Vec<usize> {
    let mut res = vec![];
    if message.is_empty() {
        return res;
    }
    'rule: for &rule in &rules[&i] {
        if rule == "\"a\"" || rule == "\"b\"" {
            if message[0..1] == rule[1..2] {
                res.push(1);
            } else {
                return vec![];
            }
        } else {
            let mut starts = vec![0];
            for ruleSpec in rule.split_whitespace() {
                let mut newStarts = vec![];
                for start in starts {
                    for diff in messageFits(&message[start..], rules, ruleSpec.parse().unwrap()) {
                        if (diff + start <= message.len()) {
                            newStarts.push(diff + start);
                        }
                    }
                }
                starts = newStarts;
            }
            res.append(&mut starts);
        }
    }
    return res;
}
