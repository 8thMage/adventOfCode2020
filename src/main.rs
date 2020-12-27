use core::panic;
use std::collections::{btree_map::Iter, HashMap, HashSet, VecDeque};
fn main() {
    let start_time = std::time::Instant::now();
    let input = include_str!("input.txt");
    let mut players = input.split_terminator("\n\n");
    let mut player_one = players
        .next()
        .unwrap()
        .lines()
        .skip(1)
        .map(|s| s.parse::<usize>().unwrap())
        .collect::<std::collections::VecDeque<usize>>();
    let mut player_two = players
        .next()
        .unwrap()
        .lines()
        .skip(1)
        .map(|s| s.parse::<usize>().unwrap())
        .collect::<std::collections::VecDeque<usize>>();
    recursiveGame(&mut player_one, &mut player_two);
    // while player_one.len() != 0 && player_two.len() != 0 {
    //     let p1 = player_one.pop_front().unwrap();
    //     let p2 = player_two.pop_front().unwrap();
    //     if p1 < p2 {
    //         player_two.push_back(p2);
    //         player_two.push_back(p1);
    //     } else {
    //         player_one.push_back(p1);
    //         player_one.push_back(p2);
    //     }
    // }
    let player = {
        if player_one.len() == 0 {
            player_two
        } else {
            player_one
        }
    };
    let score = player
        .iter()
        .rev()
        .enumerate()
        .fold(0, |acc, (index, p)| acc + (index + 1) * p);
    println!("{}", score);
    for p in player {
    }
}

fn recursiveGame(
    player_one: &mut VecDeque<usize>,
    player_two: &mut VecDeque<usize>,
) -> bool {
    let mut control = HashSet::new();
    while player_one.len() != 0 && player_two.len() != 0 {
        let mut c = "".to_string();
        for p in player_one.iter() {
            c = c + " " + &p.to_string();
        }
        c = c + "|";
        for p in player_two.iter() {
            c = c + " " + &p.to_string();
        }
        if control.contains(&c) {
            return false;
        } 
        control.insert(c);
        let p1 = player_one.pop_front().unwrap();
        let p2 = player_two.pop_front().unwrap();
        let mut win = p1 < p2;
        let len1 = player_one.len();
        let len2 = player_two.len();
        if p1 <= player_one.len() && p2 <= player_two.len() {
            let mut newPlayerOne = player_one
                .iter()
                .cloned()
                .take(p1)
                .collect::<VecDeque<usize>>();
            let mut newPlayerTwo = player_two
                .iter()
                .cloned()
                .take(p2)
                .collect::<VecDeque<usize>>();
            win = recursiveGame(&mut newPlayerOne, &mut newPlayerTwo);
        }
        if win {
            player_two.push_back(p2);
            player_two.push_back(p1);
        } else {
            player_one.push_back(p1);
            player_one.push_back(p2);
        }
    }
    if player_one.len() == 0 {
        return true;
    } else {
        return false;
    }
}
