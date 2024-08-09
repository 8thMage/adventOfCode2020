use core::num;
use std::{
    cmp::Reverse,
    collections::{BinaryHeap, HashMap, HashSet, VecDeque},
    fmt::Write,
    hash::Hash,
    iter::Cycle,
    str::FromStr,
};

#[derive(PartialEq)]
enum Type {
    Broadcast,
    FlipFlop,
    Conjuction,
}

fn main() {
    let input = include_str!("input.txt");
    let mut rules = HashMap::new();
    let mut conjuction_inputs = HashMap::new();
    for line in input.lines() {
        let (signal, dest) = line.split_once(" -> ").unwrap();
        let vec = dest.split(", ").collect::<Vec<_>>();
        let (signal_type, name) = match signal.chars().next().unwrap() {
            '%' => (Type::FlipFlop, &signal[1..]),
            '&' => (Type::Conjuction, &signal[1..]),
            _ => (Type::Broadcast, signal),
        };
        rules.insert(name, (signal_type, vec));
    }
    for (key, conj) in rules.iter_mut().filter(|(a, b)| b.0 == Type::Conjuction) {
        conjuction_inputs.insert(key.clone(), HashMap::new());
    }

    for (key, conj) in rules.iter() {
        for dest in conj.1.iter() {
            if conjuction_inputs.contains_key(dest) {
                conjuction_inputs.get_mut(dest).unwrap().insert(key, false);
            }
        }
    }
    let mut signal_current = HashMap::new();
    let mut count_high = 0;
    let mut count_low = 0;
    let mut next_to_calc = VecDeque::new();
    let mut index = 0;
    loop {
        index += 1;
        if index % 100000 == 0 {
            println!("heartbeat {}", index);
        }
        next_to_calc.push_back(("button", "broadcaster", false));
        while let Some((source, signal, high)) = next_to_calc.pop_front() {
            if high {
                count_high += 1;
            } else {
                count_low += 1;
            }
            if signal == "ls" && high {
                println!("{} {} {}", index, source, high);
            }
            if signal == "rx" {
                if !high {
                    println!("{}", index - 1);
                    return;
                }
            }
            let Some((signal_type, dest)) = rules.get(&signal) else {
                continue;
            };
            let next_signal = match signal_type {
                Type::Broadcast => {
                    for dest in dest {
                        next_to_calc.push_back((signal, dest, high));
                    }
                }
                Type::FlipFlop => {
                    if !high {
                        let entry = signal_current.entry(signal).or_insert(false);
                        *entry = !*entry;
                        for dest in dest {
                            next_to_calc.push_back((signal, dest, *entry));
                        }
                    }
                }
                Type::Conjuction => {
                    *conjuction_inputs
                        .get_mut(&signal)
                        .unwrap()
                        .get_mut(&source)
                        .unwrap() = high;
                    let res = conjuction_inputs.get(&signal).unwrap().values().all(|s| *s);
                    for dest in dest {
                        next_to_calc.push_back((signal, dest, !res));
                    }
                }
            };
        }
    }
    println!("{}", count_high * count_low);
}
