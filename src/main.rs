use core::panic;
use std::collections::{btree_map::Iter, HashMap, HashSet, VecDeque};
fn main() {
    let start_time = std::time::Instant::now();
    let input = include_str!("input.txt");
    let time = std::time::Instant::now();
    for len in 1000000+1..1000000+2 {
        // let len = 1 << loglen;
        let mut circle = Vec::new();
        let mut proto_circle: Vec<usize> = input.chars().map(|c| c as usize - '0' as usize).collect();
        for v in proto_circle.len() + 1..len {
            proto_circle.push(v);
        }
        circle.resize(len, 0);
        for i in 0..proto_circle.len() {
            circle[proto_circle[i]] = proto_circle[(i + 1) % proto_circle.len()];
        }
        let mut current_cup_index = proto_circle[0];
        for turn in 0..10000000 {
            let current_cup = current_cup_index;
            let mut new_current_cup = current_cup;
            let mut found = [true, false, false, false, false]; 
            for _ in 1..4 {
                new_current_cup = circle[new_current_cup];
                if (current_cup + len - 1 - new_current_cup) % (len - 1) <= 3 {
                    found[(current_cup + len - 1 - new_current_cup) % (len - 1)] = true;
                }
            }
            new_current_cup = (current_cup + len - 1 - found.iter().position(|c| !c).unwrap()) % (len - 1);
            if new_current_cup == 0 {
                new_current_cup = len - 1;
            }
            let t = circle[current_cup];
            let f = circle[new_current_cup];
            let k = circle[circle[circle[current_cup]]];
            circle[current_cup] = circle[k];
            circle[new_current_cup] = t;
            circle[k] = f;
            current_cup_index = circle[current_cup];
            // if (turn % 100 == 0) {
              //     let onePos = (circle.iter().position(|&c| c == 1).unwrap() + 1)% circle.len();
              //     let onePosPastOne = (circle.iter().position(|&c| c == 1).unwrap() + 2)% circle.len();
              //     // println!("{} {} {}", turn, circle[onePos], circle[onePosPastOne]);
              // }
        }
        let mut current_cup_index = 1;
        // for pos in 0..len - 2 {
        //     // let onePos = (circle.iter().position(|&c| c == 1).unwrap() + 1) % circle.len();
        //     // let onePosPastOne = (circle.iter().position(|&c| c == 1).unwrap() + 2) % circle.len();
        //     current_cup_index = circle[current_cup_index];
        //     print!("{}", current_cup_index);
        // }
        // println!("");
        let newTime = std::time::Instant::now();
        println!("{}", newTime.duration_since(time).as_millis());
        println!("{}", circle[1]);
        println!("{}", circle[circle[1]]);
        println!("{}", circle[1] * circle[circle[1]]);

    }
}
