use core::panic;
use std::collections::{btree_map::Iter, HashMap, HashSet};
fn main() {
    let start_time = std::time::Instant::now();
    let input = include_str!("input.txt");

    let tiles = read_message(input);
    let tile_borders: HashMap<usize, [usize; 8]> = tiles
        .iter()
        .map(|(id, tile)| (*id, get_borders(tile)))
        .collect();

    let mut mul = 1usize;
    let mut corners = Vec::new();
    for (id, borders) in &tile_borders {
        let mut number_border = 0;
        for border in borders {
            let mut find = 0;
            for (otherid, otherborders) in &tile_borders {
                if otherid != id && otherborders.contains(border) {
                    find += 1;
                }
            }
            number_border += (find != 0) as usize;
        }
        if number_border == 4 {
            corners.push(*id);
            mul *= id;
        }
    }
    for &reverse in [Reverse::normal, Reverse::reverse].iter() {
        for rotation in 0..4 {
            let orientation = orientations([0, 1, 2, 3, 4, 5, 6, 7], reverse, rotation);
            println!(
                "{} {} {} {}",
                orientation[0], orientation[1], orientation[2], orientation[3]
            );
        }
    }
    let mut new_map: Vec<Vec<(usize, Reverse, usize)>> = Vec::new();
    let mut count = 0;
    let monsterInput = "                  # 
#    ##    ##    ###
 #  #  #  #  #  #   ";
    let monsterPositions = monsterInput
        .lines()
        .enumerate()
        .flat_map(|(y, line)| {
            line.chars()
                .enumerate()
                .filter_map(move |(x, c)| if (c == '#') { Some((y, x)) } else { None })
        })
        .collect::<Vec<(usize, usize)>>();
    for &corner in &corners {
        // for &corner in [1951].iter() {
        for rotation in 0..4 {
            'new_map: for &reverse in [Reverse::normal, Reverse::reverse].iter() {
                // 'new_map: for &reverse in [Reverse::normal].iter() {
                let mut new_map: Vec<Vec<(usize, Reverse, usize)>> = Vec::new();
                new_map.push(Vec::new());
                new_map[0].push((corner, reverse, rotation));
                while new_map[0].len() == 1 || !corners.contains(&new_map[0].last().unwrap().0) {
                    let tile = new_map[0].last().unwrap();
                    let orientation = orientations(tile_borders[&tile.0], tile.1, tile.2);
                    let right = tile_borders[&tile.0][orientation[1]];
                    let new_tile = tile_borders
                        .iter()
                        .find(|s| *s.0 != tile.0 && s.1.contains(&right));
                    if new_tile.is_none() {
                        continue 'new_map;
                    }
                    let new_tile = new_tile.unwrap();
                    let orientation = orientations_from_left(
                        tile_borders[new_tile.0]
                            .iter()
                            .position(|s| *s == right)
                            .unwrap(),
                    );
                    new_map[0].push((*new_tile.0, orientation.0, orientation.1));
                    // println!("right {} reverse {}", right, right.reverse_bits()>>(63 - 9));
                }
                while new_map.len() == 1 || !corners.contains(&new_map.last().unwrap()[0].0) {
                    let tile = &new_map.last().unwrap()[0];
                    if (new_map.len() == 1) {
                        // println!("{} {}" ,tile.0,corner);
                    }
                    let orientation = orientations(tile_borders[&tile.0], tile.1, tile.2);
                    let bottom = tile_borders[&tile.0][orientation[2]];
                    let new_tile = tile_borders
                        .iter()
                        .find(|s| *s.0 != tile.0 && s.1.contains(&bottom));
                    if new_tile.is_none() {
                        continue 'new_map;
                    }
                    let new_tile = new_tile.unwrap();
                    let orientation = orientations_from_top(
                        tile_borders[new_tile.0]
                            .iter()
                            .position(|s| *s == bottom)
                            .unwrap(),
                    );
                    new_map.push(Vec::new());
                    let len = new_map[0].len();
                    let mut new_row = new_map.last_mut().unwrap();
                    new_row.push((*new_tile.0, orientation.0, orientation.1));
                    // println!("bottom {} reverse {}", bottom, bottom.reverse_bits()>>(63 - 9));
                    // println!("{}", new_row[0].0);

                    for x in 1..len {
                        let tile = &new_row.last().unwrap();
                        let orientation = orientations(tile_borders[&tile.0], tile.1, tile.2);
                        let right = tile_borders[&tile.0][orientation[1]];
                        let new_tile = tile_borders
                            .iter()
                            .find(|s| *s.0 != tile.0 && s.1.contains(&right));
                        if new_tile.is_none() {
                            // println!("{}",right);
                            continue 'new_map;
                        }
                        let new_tile = new_tile.unwrap();
                        let orientation = orientations_from_left(
                            tile_borders[new_tile.0]
                                .iter()
                                .position(|s| *s == right)
                                .unwrap(),
                        );
                        new_row.push((*new_tile.0, orientation.0, orientation.1));
                    }
                }
                let mut assembled = Vec::new();
                assembled.resize(new_map.len() * 8, {
                    let mut v = Vec::new();
                    v.resize(new_map[0].len() * 8, 0);
                    v
                });
                for (r, tileRow) in new_map.iter().enumerate() {
                    for (c, tile) in tileRow.iter().enumerate() {
                        println!(
                            "({} {} {}) ",
                            tile.0,
                            (tile.1 == Reverse::reverse) as usize,
                            tile.2
                        );
                        if (tile.0 == 3079) {
                            let b = 0;
                        }
                        for y in 1..9 {
                            for x in 1..9 {
                                let (newY, newX) = recoordinates(tile.1, tile.2, y, x);
                                let Ay = r * 8 + y - 1;
                                let Ax = c * 8 + x - 1;
                                assembled[Ay][Ax] = tiles[&tile.0][newY][newX];
                            }
                        }
                    }
                }
                println!("");
                let mut newAssembled = Vec::new();
                for (rowIndex, tileRow) in assembled.iter().enumerate() {
                    if (rowIndex % 8 == 0) {
                        println!(" ");
                    }
                    newAssembled.push(Vec::new());
                    for (index, tile) in tileRow.iter().enumerate() {
                        if (index % 8 == 0) {
                            print!(" ");
                        }
                        print!("{}", tile);
                        newAssembled.last_mut().unwrap().push(*tile);
                    }
                    println!("");
                }
                for y in 0..assembled.len() {
                    'x: for x in 0..assembled[0].len() {
                        for position in &monsterPositions {
                            let newY = position.0 + y;
                            let newX = position.1 + x;
                            let row = assembled.get(newY);
                            let char = row.map(|r| r.get(newX)).flatten();
                            if (char.is_none() || *char.unwrap() != 1) {
                                continue 'x;
                            }
                        }
                        count += 1;
                        for position in &monsterPositions {
                            let newY = position.0 + y;
                            let newX = position.1 + x;
                            newAssembled[newY][newX] = 0;
                        }
                    }
                }
                if (count != 0) {
                    println!("{}", newAssembled.iter().flat_map(|i| i.iter()).filter(|&&c| c!=0).count());
                    let b = 0;
                }
            }
        }
    }
    println!("{}", count);
}

fn read_message(input: &str) -> HashMap<usize, Vec<Vec<usize>>> {
    input
        .split_terminator("\n\n")
        .map(|s| {
            (
                s.lines()
                    .next()
                    .unwrap()
                    .split_whitespace()
                    .nth(1)
                    .unwrap()
                    .split_terminator(":")
                    .next()
                    .unwrap()
                    .parse::<usize>()
                    .unwrap(),
                s.lines()
                    .skip(1)
                    .map(|line| {
                        line.chars()
                            .map(|c| (c == '#') as usize)
                            .collect::<Vec<usize>>()
                    })
                    .collect::<Vec<Vec<usize>>>(),
            )
        })
        .collect()
}

fn get_borders(tile: &Vec<Vec<usize>>) -> [usize; 8] {
    let border0 = vector_to_num(tile[0].iter().cloned());
    let border1 = vector_to_num(tile.last().unwrap().iter().cloned());
    let border2 = vector_to_num(tile.iter().map(|v| v[0]));
    let border3 = vector_to_num(tile.iter().map(|v| v.last().unwrap()).cloned());
    let border4 = vector_to_num(tile[0].iter().cloned().rev());
    let border5 = vector_to_num(tile.last().unwrap().iter().cloned().rev());
    let border6 = vector_to_num(tile.iter().map(|v| v[0]).rev());
    let border7 = vector_to_num(tile.iter().map(|v| v.last().unwrap()).cloned().rev());
    [
        border0, border1, border2, border3, border4, border5, border6, border7,
    ]
}

fn vector_to_num<iter>(border: iter) -> usize
where
    iter: Iterator<Item = usize>,
{
    border.fold(0, |acc, s| (acc << 1) + s)
}

#[derive(Copy, Clone, PartialEq)]
enum Reverse {
    normal,
    reverse,
}

fn orientations(borders: [usize; 8], reverse: Reverse, rotate: usize) -> [usize; 4] {
    let mut collectedArray = match reverse {
        Reverse::normal => [0, 3, 1, 2],
        Reverse::reverse => [4, 2, 5, 3],
    };
    for _ in 0..rotate {
        collectedArray = [
            (collectedArray[3] + 4) % 8,
            collectedArray[0],
            (collectedArray[1] + 4) % 8,
            collectedArray[2],
        ];
    }
    return collectedArray;
}

fn orientations_from_top(border_index: usize) -> (Reverse, usize) {
    return match border_index {
        0 => (Reverse::normal, 0),
        1 => (Reverse::reverse, 2),
        2 => (Reverse::reverse, 3),
        3 => (Reverse::normal, 3),
        4 => (Reverse::reverse, 0),
        5 => (Reverse::normal, 2),
        6 => (Reverse::normal, 1),
        7 => (Reverse::reverse, 1),
        _ => panic!(),
    };
}

fn orientations_from_left(border_index: usize) -> (Reverse, usize) {
    return match border_index {
        0 => (Reverse::reverse, 3),
        1 => (Reverse::normal, 1),
        2 => (Reverse::normal, 0),
        3 => (Reverse::reverse, 0),
        4 => (Reverse::normal, 3),
        5 => (Reverse::reverse, 1),
        6 => (Reverse::reverse, 2),
        7 => (Reverse::normal, 2),
        _ => panic!(),
    };
}

fn recoordinates(reverse: Reverse, rotate: usize, y: usize, x: usize) -> (usize, usize) {
    let mut rotate2 = rotate;
    let (y1, x1) = match reverse {
        Reverse::normal => (y, x),
        Reverse::reverse => {
            rotate2 = 4 - rotate;
            (y, 9 - x)
        }
    };
    let mut y = y1;
    let mut x = x1;
    for _ in 0..rotate2 {
        let tx = y;
        y = 9 - x;
        x = tx;
    }
    return (y, x);
}
