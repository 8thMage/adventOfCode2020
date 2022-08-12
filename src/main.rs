use std::{collections::HashMap, default, hash::Hash, iter::repeat};

#[derive(Clone, PartialEq, Hash, Eq)]
struct Positions {
    pos: [Vec<(i8, i32, i32)>; 4],
}

fn main() {
    let input = include_str!("input.txt");
    let mut pos = HashMap::new();
    let mut a_count = 0;
    let mut b_count = 0;
    let mut c_count = 0;
    let mut d_count = 0;
    for (line_number, line) in input.lines().enumerate() {
        if line_number == 1 || line_number == 2 || line_number == 3 || line_number == 4 || line_number == 5 {
            for (char_number, char) in line.chars().enumerate() {
                if char == 'A' {
                    pos.insert(
                        ('A', a_count),
                        (0, char_number as i32 - 1, line_number as i32 - 1),
                    );
                    a_count += 1;
                }

                if char == 'B' {
                    pos.insert(
                        ('B', b_count),
                        (0, char_number as i32 - 1, line_number as i32 - 1),
                    );
                    b_count += 1;
                }

                if char == 'C' {
                    pos.insert(
                        ('C', c_count),
                        (0, char_number as i32 - 1, line_number as i32 - 1),
                    );
                    c_count += 1;
                }

                if char == 'D' {
                    pos.insert(
                        ('D', d_count),
                        (0, char_number as i32 - 1, line_number as i32 - 1),
                    );
                    d_count += 1;
                }
            }
        }
    }
    let x =0;
    // let map = Positions {
    //     pos: [
    //         vec![*pos.get(&('A', 0)).unwrap(), *pos.get(&('A', 1)).unwrap()],
    //         vec![*pos.get(&('B', 0)).unwrap(), *pos.get(&('B', 1)).unwrap()],
    //         vec![*pos.get(&('C', 0)).unwrap(), *pos.get(&('C', 1)).unwrap()],
    //         vec![*pos.get(&('D', 0)).unwrap(), *pos.get(&('D', 1)).unwrap()],
    //     ],
    // };
    // println!(
    //     "{}",
    //     find_min_energy(&map, 0, &mut HashMap::new(), 2).unwrap()
    // );

    // for pos in pos.iter_mut() {
    //     if pos.1 .2 == 2 {
    //         pos.1 .2 = 4
    //     }
    // }
    // let map2 = Positions {
    //     pos: [
    //         vec![
    //             *pos.get(&('A', 0)).unwrap(),
    //             *pos.get(&('A', 1)).unwrap(),
    //             (0, 8, 2),
    //             (0, 6, 3),
    //         ],
    //         vec![
    //             *pos.get(&('B', 0)).unwrap(),
    //             *pos.get(&('B', 1)).unwrap(),
    //             (0, 6, 2),
    //             (0, 4, 3),
    //         ],
    //         vec![
    //             *pos.get(&('C', 0)).unwrap(),
    //             *pos.get(&('C', 1)).unwrap(),
    //             (0, 4, 2),
    //             (0, 8, 3),
    //         ],
    //         vec![
    //             *pos.get(&('D', 0)).unwrap(),
    //             *pos.get(&('D', 1)).unwrap(),
    //             (0, 2, 2),
    //             (0, 2, 3),
    //         ],
    //     ],
    // };

    let map2 = Positions {
        pos: [
            vec![
                *pos.get(&('A', 0)).unwrap(),
                *pos.get(&('A', 1)).unwrap(),
                *pos.get(&('A', 2)).unwrap(),
                *pos.get(&('A', 3)).unwrap(),
            ],
            vec![
                *pos.get(&('B', 0)).unwrap(),
                *pos.get(&('B', 1)).unwrap(),
                *pos.get(&('B', 2)).unwrap(),
                *pos.get(&('B', 3)).unwrap(),
            ],
            vec![
                *pos.get(&('C', 0)).unwrap(),
                *pos.get(&('C', 1)).unwrap(),
                *pos.get(&('C', 2)).unwrap(),
                *pos.get(&('C', 3)).unwrap(),
            ],
            vec![
                *pos.get(&('D', 0)).unwrap(),
                *pos.get(&('D', 1)).unwrap(),
                *pos.get(&('D', 2)).unwrap(),
                *pos.get(&('D', 3)).unwrap(),
            ],
        ],
    };

    println!(
        "{}",
        find_min_energy(&map2, 0, &mut HashMap::new(), 4).unwrap()
    );
}

fn find_min_energy(
    map: &Positions,
    current_score: i64,
    hash: &mut HashMap<Positions, Option<i64>>,
    last_y: i32,
) -> Option<i64> {
    if hash.contains_key(&map) {
        return hash.get(&map).unwrap().map(|m| m + current_score);
    }
    'letter: for letter in 0..map.pos.len() {
        for number in 0..map.pos[letter].len() {
            let (moves, current_x, current_y) = map.pos[letter][number];
            if current_x != 2 + letter as i32 * 2 || current_y == 0 {
                break 'letter;
            }
        }
        if letter == map.pos.len() - 1 {
            return Some(current_score);
        }
    }
    let mut minimum = None;
    for letter in 0..map.pos.len() {
        if map.pos[letter].iter().all(|pos| pos.1 == letter as i32 * 2 + 2)          
        {
            continue;
        }
        'next_number: for number in 0..map.pos[letter].len() {
            if current_score == 0 || current_score == 3000 {
                let z = 0;
            }
            let mut new_map = map.clone();
            if map.pos[letter][number].2 != 0 {
                let (moves, current_x, current_y) = map.pos[letter][number];
                if moves == 2 {
                    continue;
                }
                'next_x: for x in 0..11 {
                    if x == 2 || x == 4 || x == 6 || x == 8 {
                        continue;
                    }
                    for &(_, other_x, other_y) in map.pos.iter().map(|x| x.iter()).flatten() {
                        if other_y == 0
                            && (other_x == x
                                || ((other_x <= x && other_x > current_x)
                                    || (other_x >= x && other_x < current_x)))
                        {
                            continue 'next_x;
                        } else if other_x == current_x && current_y > other_y {
                            continue 'next_number;
                        }
                    }
                    new_map.pos[letter][number] = (moves + 1, x, 0);
                    let new_score = current_score
                        + (((x - current_x).abs() + current_y) as i64) * 10i64.pow(letter as u32);
                    if let Some(score) = find_min_energy(&new_map, new_score, hash, last_y) {
                        minimum = Some(minimum.unwrap_or(i64::max_value()).min(score));
                    }
                }
            } else {
                let (moves, current_x, current_y) = map.pos[letter][number];
                if moves == 2 {
                    continue;
                }
                let mut next_y = last_y;
                let next_x = letter as i32 * 2 + 2;
                for (other_letter, &(moves, other_x, other_y)) in map
                    .pos
                    .iter()
                    .enumerate()
                    .map(|x| repeat(x.0).zip(x.1.iter()))
                    .flatten()
                {
                    if other_y == 0
                        && ((other_x < next_x && other_x > current_x)
                            || (other_x > next_x && other_x < current_x))
                    {
                        continue 'next_number;
                    } else if other_x == next_x {
                        if other_letter == letter {
                            next_y = next_y.min(other_y - 1);
                        } else {
                            continue 'next_number;
                        }
                    }
                }
                new_map.pos[letter][number] = (2, next_x, next_y);
                let new_score = current_score
                    + (((next_x - current_x).abs() + next_y) as i64) * 10i64.pow(letter as u32);
                if let Some(score) = find_min_energy(&new_map, new_score, hash, last_y) {
                    minimum = Some(minimum.unwrap_or(i64::max_value()).min(score));
                }
            }
        }
    }
    hash.insert(map.clone(), minimum.map(|m| m - current_score));
    return minimum;
}
