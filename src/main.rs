use std::{
    collections::{HashMap, HashSet, VecDeque},
    str::FromStr,
};

enum Operation {
    Mul(u32),
    Add(u32),
    Sqr,
}

struct Monkey {
    items: Vec<u128>,
    operation: Operation,
    divisible_by: u32,
    dests: [usize; 2],
}

fn main() {
    let input = include_str!("input.txt");
    let mut monkeys = vec![];
    for line in input.lines() {
        if line.starts_with("Monkey") {
            monkeys.push(Monkey {
                items: vec![],
                operation: Operation::Add(0),
                divisible_by: 0,
                dests: [0, 0],
            });
            continue;
        }
        if line.starts_with("  Starting items: ") {
            let line = line.split_once("  Starting items: ").unwrap().1;
            monkeys.last_mut().unwrap().items = line
                .split(", ")
                .filter(|s| !s.is_empty())
                .map(|s| u128::from_str(s).unwrap())
                .collect();
            continue;
        }
        if line.starts_with("  Operation: new = ") {
            let line = line.split_once("Operation: new = ").unwrap().1;
            if line == "old * old" {
                monkeys.last_mut().unwrap().operation = Operation::Sqr;
            } else {
                if line.starts_with("old * ") {
                    let mul = u32::from_str(line.split_once("old * ").unwrap().1).unwrap();
                    monkeys.last_mut().unwrap().operation = Operation::Mul(mul);
                }
                if line.starts_with("old + ") {
                    let adder = u32::from_str(line.split_once("old + ").unwrap().1).unwrap();
                    monkeys.last_mut().unwrap().operation = Operation::Add(adder);
                }
            }
            continue;
        }
        if line.starts_with("  Test: divisible by ") {
            let divisor = u32::from_str(line.split_once("Test: divisible by ").unwrap().1).unwrap();
            monkeys.last_mut().unwrap().divisible_by = divisor;
            continue;
        }
        if line.starts_with("    If true: throw to monkey ") {
            let dest0 =
                usize::from_str(line.split_once("If true: throw to monkey ").unwrap().1).unwrap();
            monkeys.last_mut().unwrap().dests[0] = dest0;
            continue;
        }

        if line.starts_with("    If false: throw to monkey ") {
            let dest1 =
                usize::from_str(line.split_once("If false: throw to monkey ").unwrap().1).unwrap();
            monkeys.last_mut().unwrap().dests[1] = dest1;
            continue;
        }
    }
    let gcd:u32 = monkeys.iter().map(|m| m.divisible_by).product();

    let mut sums = vec![0; monkeys.len()];
    for _ in 0..10000 {
        for index in 0..monkeys.len() {
            sums[index] += monkeys[index].items.len();
            for item in monkeys[index].items.clone().iter() {
                let new_item = match monkeys[index].operation {
                    Operation::Sqr => item * item,
                    Operation::Mul(mul) => item * mul as u128,
                    Operation::Add(adder) => item + adder as u128,
                } % gcd as u128 ;
                let pick = (new_item % monkeys[index].divisible_by as u128 != 0) as usize;
                let dest = monkeys[index].dests[pick];
                monkeys[dest].items.push(new_item);
            }
            monkeys[index].items.clear();
        }
    }
    sums.sort();
    println!("{}", sums[sums.len() - 1] * sums[sums.len() - 2]);
}
