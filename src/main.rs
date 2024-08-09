use core::num;
use std::{
    cmp::Reverse,
    collections::{BinaryHeap, HashMap, HashSet},
    fmt::Write,
    hash::Hash,
    iter::Cycle,
    str::FromStr,
};

struct Rule {
    check: &'static str,
    bge: bool,
    amount: u64,
    next: &'static str,
}

fn main() {
    let input = include_str!("input.txt");
    let mut lines = input.lines();
    let mut all_rules = HashMap::new();
    for line in lines.by_ref() {
        if line.is_empty() {
            break;
        }
        let (name, rules) = line.split_once("{").unwrap();
        let rules = rules.trim_end_matches('}');
        let rules = rules
            .split(",")
            .map(|s| {
                if let Some((a, b)) = s.split_once(':') {
                    if a.contains('<') {
                        let (check, max) = a.split_once('<').unwrap();
                        Rule {
                            check: check,
                            bge: false,
                            amount: u64::from_str(max).unwrap(),
                            next: b,
                        }
                    } else {
                        let (check, max) = a.split_once('>').unwrap();
                        Rule {
                            check: check,
                            bge: true,
                            amount: u64::from_str(max).unwrap(),
                            next: b,
                        }
                    }
                } else {
                    Rule {
                        check: "x",
                        bge: false,
                        amount: u64::MAX,
                        next: s,
                    }
                }
            })
            .collect::<Vec<_>>();
        all_rules.insert(name, rules);
    }
    let mut sum = 0;
    for line in lines {
        let line = line.trim_end_matches('}');
        let line = line.trim_end_matches('{');
        let part = line
            .split(',')
            .map(|s| u64::from_str(s.split_once('=').unwrap().1).unwrap())
            .collect::<Vec<_>>();
        let mut current_rule = "in";
        'l: loop {
            let current_rule_vec = &all_rules[current_rule];
            for rule in current_rule_vec {
                let index = match rule.check {
                    "x" => 0,
                    "m" => 1,
                    "a" => 2,
                    "s" => 3,
                    _ => unreachable!(),
                };
                let part_amount = part[index];
                let accept = if rule.bge {
                    part_amount > rule.amount
                } else {
                    part_amount < rule.amount
                };
                if accept {
                    if rule.next == "R" {
                        break 'l;
                    }
                    if rule.next == "A" {
                        sum += part.iter().sum::<u64>();
                        break 'l;
                    }
                    current_rule = rule.next;
                    continue 'l;
                }
            }
        }
    }
    println!("{}", sum);
    println!(
        "{}",
        calculate_combinations(vec![(1, 4000); 4], &all_rules, "in")
    );
}

fn calculate_combinations(
    mut parts: Vec<(u64, u64)>,
    rules: &HashMap<&'static str, Vec<Rule>>,
    current_rule: &str,
) -> u64 {
    if parts.iter().any(|(a, b)| b < a) {
        return 0;
    }
    if current_rule == "R" {
        return 0;
    }
    if current_rule == "A" {
        return parts.iter().map(|(a, b)| b - a + 1).product::<u64>();
    }
    let current_rule_vec = &rules[current_rule];
    let mut sum = 0;
    for rule in current_rule_vec {
        let index = match rule.check {
            "x" => 0,
            "m" => 1,
            "a" => 2,
            "s" => 3,
            _ => unreachable!(),
        };
        let current_part = if rule.bge {
            let mut curr = parts.clone();
            curr[index].0 = (rule.amount + 1).max(curr[index].0);
            parts[index].1 = curr[index].0-1;
            curr
        } else {
            let mut curr = parts.clone();
            curr[index].1 = (rule.amount - 1).min(curr[index].1);
            parts[index].0 = curr[index].1 + 1;
            curr
        };
        sum += calculate_combinations(current_part, rules, &rule.next);
    }
    return sum;
}
