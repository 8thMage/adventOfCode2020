use std::{
    cmp::Ordering,
    collections::{HashMap, HashSet, VecDeque},
    hash::Hash,
    iter, path,
    str::FromStr,
    thread::current,
    vec,
};

#[derive(PartialEq, Clone, Copy, Debug)]
enum RegValue {
    Simple(i128),
    Affine { mul: i128, add: i128, div: i128 },
    None,
}

fn main() {
    let input = include_str!("input.txt");

    let (map_str, instructions) = input.split_once("\n\n").unwrap();
    let mut map = vec![];
    for line in map_str.lines() {
        let start = line.chars().take_while(|c| *c == ' ').count();
        let line = line
            .chars()
            .skip(start)
            .map(|c| if c == '#' { 0 } else { 1 })
            .collect::<Vec<_>>();
        map.push((start, line));
    }

    for (start, l) in &map {
        for _ in 0..*start {
            print!(" ");
        }
        for c in l {
            if *c == 0 {
                print!("#");
            } else {
                print!(".");
            }
        }
        println!("");
    }

    let (mut current_position_x, mut current_position_y) = (map[0].0, 0usize);
    let number_iter = instructions
        .split(|c: char| c.is_alphabetic())
        .map(|s| u64::from_str(s).unwrap());
    let char_iter = instructions
        .split(|c: char| c.is_numeric())
        .filter(|s| s.len() > 0);
    let (mut dx, mut dy) = (1usize, 0usize);
    let mut wrap_around_rules = HashMap::new();
    let width = map.iter().map(|s| s.0 + s.1.len()).max().unwrap();
    let min_y_of_x = |x| {
        map.iter()
            .position(|s| s.0 <= x && s.0 + s.1.len() > x)
            .unwrap()
    };
    let max_y_of_x = |x| {
        map.iter()
            .rposition(|s| s.0 <= x && s.0 + s.1.len() > x)
            .unwrap()
    };
    for x in 0..width / 3 {
        assert!(wrap_around_rules
            .insert(
                (min_y_of_x(x).wrapping_sub(1), x, (0, usize::MAX)),
                (width / 3 + x, map[width / 3 + x].0, (1, 0)),
            )
            .is_none());
        assert!(wrap_around_rules
            .insert(
                (
                    min_y_of_x(x + width / 3).wrapping_sub(1),
                    x + width / 3,
                    (0, usize::MAX)
                ),
                (3 * width / 3 + x, map[3 * width / 3 + x].0, (1, 0)),
            )
            .is_none());

        assert!(wrap_around_rules
            .insert(
                (
                    min_y_of_x(width / 3 * 2 + x).wrapping_sub(1),
                    width / 3 * 2 + x,
                    (0, usize::MAX)
                ),
                (max_y_of_x(x), x, (0, usize::MAX)),
            )
            .is_none());

        assert!(wrap_around_rules
            .insert(
                (max_y_of_x(x) + 1, x, (0, 1)),
                (min_y_of_x(width / 3 * 2 + x), width / 3 * 2 + x, (0, 1)),
            )
            .is_none());

        assert!(wrap_around_rules
            .insert(
                (max_y_of_x(x + width / 3) + 1, x + width / 3, (0, 1)),
                (
                    3 * width / 3 + x,
                    map[3 * width / 3 + x].0 + map[3 * width / 3 + x].1.len() - 1,
                    (usize::MAX, 0),
                ),
            )
            .is_none());

        assert!(wrap_around_rules
            .insert(
                (max_y_of_x(width / 3 * 2 + x) + 1, width / 3 * 2 + x, (0, 1)),
                (
                    width / 3 + x,
                    map[width / 3 + x].0 + map[width / 3 + x].1.len() - 1,
                    (usize::MAX, 0),
                ),
            )
            .is_none());

        assert!(wrap_around_rules
            .insert(
                (x, map[x].0.wrapping_sub(1), (usize::MAX, 0)),
                (width / 3 * 3 - x - 1, map[width / 3 * 3 - x - 1].0, (1, 0)),
            )
            .is_none());

        assert!(wrap_around_rules
            .insert(
                (
                    width / 3 + x,
                    map[width / 3 + x].0.wrapping_sub(1),
                    (usize::MAX, 0)
                ),
                (min_y_of_x(x), x, (0, 1)),
            )
            .is_none());

        assert!(wrap_around_rules
            .insert(
                (
                    2 * width / 3 + x,
                    map[2 * width / 3 + x].0.wrapping_sub(1),
                    (usize::MAX, 0)
                ),
                (width / 3 - x - 1, map[width / 3 - x - 1].0, (1, 0)),
            )
            .is_none());

        assert!(wrap_around_rules
            .insert(
                (
                    3 * width / 3 + x,
                    map[3 * width / 3 + x].0.wrapping_sub(1),
                    (usize::MAX, 0)
                ),
                (min_y_of_x(x + width / 3), x + width / 3, (0, 1)),
            )
            .is_none());

        assert!(wrap_around_rules
            .insert(
                (x, map[x].0 + map[x].1.len(), (1, 0)),
                (
                    3 * width / 3 - x - 1,
                    map[3 * width / 3 - x - 1].0 + map[3 * width / 3 - x - 1].1.len() - 1,
                    (usize::MAX, 0),
                ),
            )
            .is_none());

        assert!(wrap_around_rules
            .insert(
                (
                    width / 3 + x,
                    map[width / 3 + x].0 + map[width / 3 + x].1.len(),
                    (1, 0)
                ),
                (
                    max_y_of_x(2 * width / 3 + x),
                    2 * width / 3 + x,
                    (0, usize::MAX),
                ),
            )
            .is_none());

        assert!(wrap_around_rules
            .insert(
                (
                    3 * width / 3 - x - 1,
                    map[3 * width / 3 - x - 1].0 + map[3 * width / 3 - x - 1].1.len(),
                    (1, 0)
                ),
                (x, map[x].0 + map[x].1.len() - 1, (usize::MAX, 0)),
            )
            .is_none());

        assert!(wrap_around_rules
            .insert(
                (
                    3 * width / 3 + x,
                    map[3 * width / 3 + x].0 + map[3 * width / 3 + x].1.len(),
                    (1, 0)
                ),
                (max_y_of_x(width / 3 + x), width / 3 + x, (0, usize::MAX)),
            )
            .is_none());
    }
    let mut checked = HashSet::new();
    for (number, char) in number_iter.zip(char_iter.chain(iter::repeat(" "))) {
        for _ in 0..number {
            let (mut next_x, mut next_y) = (
                current_position_x.wrapping_add(dx),
                current_position_y.wrapping_add(dy),
            );
            let mut next_dir = (dx, dy);
            if let Some(&(ny, nx, dir)) = wrap_around_rules.get(&(next_y, next_x, (dx, dy))) {
                if !checked
                    .get(&(next_y / (width / 3), next_x / (width / 3)))
                    .is_some()
                {
                    println!(
                        "({},{}) => ({},{}, dx {} dy {})",
                        next_y as i32,
                        next_x as i32,
                        ny as i32,
                        nx as i32,
                        dir.0 as i32,
                        dir.1 as i32
                    );
                    checked.insert((next_y / (width / 3), next_x / (width / 3)));
                }
                let &s = wrap_around_rules
                    .get(&(
                        ny.wrapping_sub(dir.1),
                        nx.wrapping_sub(dir.0),
                        (dir.0.wrapping_neg(), dir.1.wrapping_neg()),
                    ))
                    .unwrap();
                assert!(
                    s.1 == current_position_x
                        && s.0 == current_position_y
                        && s.2 .0 == dx.wrapping_neg()
                        && s.2 .1 == dy.wrapping_neg(),
                    "({},{}) => ({},{}, dx {} dy {})",
                    next_y as i32,
                    next_x as i32,
                    ny as i32,
                    nx as i32,
                    dir.0 as i32,
                    dir.1 as i32
                );
                next_x = nx;
                next_y = ny;
                next_dir = dir;
            }
            // next_y = (next_y.wrapping_add(map.len())) % map.len();
            // if next_x < map[current_position_y].0 || next_x == usize::MAX {
            //     next_x = map[current_position_y].0 + map[current_position_y].1.len() - 1;
            // }
            // if next_x >= map[current_position_y].0 + map[current_position_y].1.len() {
            //     next_x = map[current_position_y].0;
            // }
            // while next_x >= map[next_y].0 + map[next_y].1.len() || next_x < map[next_y].0 {
            //     next_y = next_y.wrapping_add(dy);
            //     next_y = (next_y.wrapping_add(map.len())) % map.len();
            // }
            if map[next_y].1[next_x - map[next_y].0] == 0 {
                break;
            }
            current_position_x = next_x;
            current_position_y = next_y;
            dx = next_dir.0;
            dy = next_dir.1;
        }
        (dx, dy) = match char {
            "L" => (dy, dx.wrapping_neg()),
            "R" => (dy.wrapping_neg(), dx),
            " " => (dx, dy),
            _ => panic!(),
        };
    }
    println!(
        "{}",
        1000 * (current_position_y + 1)
            + 4 * (current_position_x + 1)
            + match (dx, dy) {
                (1, 0) => 0,
                (0, 1) => 1,
                (usize::MAX, 0) => 2,
                (0, usize::MAX) => 3,
                _ => panic!(),
            }
    );
}
