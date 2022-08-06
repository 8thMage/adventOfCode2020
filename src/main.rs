use std::{collections::HashMap, hash::Hash};

fn main() {
    let input = include_str!("input.txt");
    let mut player_one_pos = u64::from_str_radix(
        input.lines().nth(0).unwrap().split_once(": ").unwrap().1,
        10,
    )
    .unwrap();
    let mut player_two_pos = u64::from_str_radix(
        input.lines().nth(1).unwrap().split_once(": ").unwrap().1,
        10,
    )
    .unwrap();
    let mut player_one_score = 0;
    let mut player_two_score = 0;
    let mut turn = false;
    let mut die = 0;
    let mut number_of_die_turn = 0;
    loop {
        die = die + 1;
        if die > 100 {
            die = 1;
        }
        let mut movement = die;
        die = die + 1;
        if die > 100 {
            die = 1;
        }
        movement += die;

        die = die + 1;
        if die > 100 {
            die = 1;
        }
        movement += die;
        number_of_die_turn += 3;
        match turn {
            false => {
                player_one_pos += movement;
                if player_one_pos > 10 {
                    player_one_pos -= 1;
                    player_one_pos %= 10;
                    player_one_pos += 1;
                }
                player_one_score += player_one_pos;
                if player_one_score >= 1000 {
                    println!("{}", player_two_score * number_of_die_turn);
                    break;
                }
            }
            true => {
                player_two_pos += movement;
                if player_two_pos > 10 {
                    player_two_pos -= 1;
                    player_two_pos %= 10;
                    player_two_pos += 1;
                }
                player_two_score += player_two_pos;
                if player_two_score >= 1000 {
                    println!("{}", player_one_score * number_of_die_turn);
                    break;
                }
            }
        }
        turn = !turn;
    }
    let player_one_pos = u64::from_str_radix(
        input.lines().nth(0).unwrap().split_once(": ").unwrap().1,
        10,
    ).unwrap();
    
    let player_two_pos = u64::from_str_radix(
        input.lines().nth(1).unwrap().split_once(": ").unwrap().1,
        10,
    ).unwrap();

    let mut universes_win = HashMap::new();
    let result = calculate_universe_win(
        &mut universes_win,
        Key {
            player_one_score: 0,
            player_two_score: 0,
            player_one_pos: player_one_pos as i64,
            player_two_pos: player_two_pos as i64,
            turn: false,
        },
    );
    println!("{}, {}", result.0, result.1);
    println!("{}", result.0.max(result.1));
}

#[derive(Hash, PartialEq, Eq)]
struct Key {
    player_one_score: i64,
    player_two_score: i64,
    player_one_pos: i64,
    player_two_pos: i64,
    turn: bool,
}

fn calculate_universe_win(universe_win: &mut HashMap<Key, (i128, i128)>, key: Key) -> (i128, i128) {
    if universe_win.contains_key(&key) {
        return universe_win[&key];
    }
    if key.player_one_score >= 21 {
        return (1, 0);
    }
    if key.player_two_score >= 21 {
        return (0, 1);
    }
    let delta = vec![3, 4, 5, 6, 7, 8, 9];
    let freq = vec![1, 3, 6, 7, 6, 3, 1];
    let mut acc = (0, 0);
    for i in 0..delta.len() {
        let new_key = match key.turn {
            false => {
                let mut player_one_pos = key.player_one_pos + delta[i];
                if player_one_pos > 10 {
                    player_one_pos -= 1;
                    player_one_pos %= 10;
                    player_one_pos += 1;
                }
                let player_one_score = key.player_one_score + player_one_pos;
                Key {
                    player_one_score,
                    player_two_score: key.player_two_score,
                    player_one_pos,
                    player_two_pos: key.player_two_pos,
                    turn: true,
                }
            }
            true => {
                let mut player_two_pos = key.player_two_pos + delta[i];
                if player_two_pos > 10 {
                    player_two_pos -= 1;
                    player_two_pos %= 10;
                    player_two_pos += 1;
                }
                let player_two_score = key.player_two_score + player_two_pos;
                Key {
                    player_one_score: key.player_one_score,
                    player_two_score,
                    player_one_pos: key.player_one_pos,
                    player_two_pos,
                    turn: false,
                }
            }
        };
        let result = calculate_universe_win(universe_win, new_key);
        acc.0 += result.0 * freq[i];
        acc.1 += result.1 * freq[i];
    }
    universe_win.insert(key, acc);
    return acc;
}
