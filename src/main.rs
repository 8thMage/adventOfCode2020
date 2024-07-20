use std::{
    cmp::Ordering,
    collections::{hash_map, HashMap, HashSet, VecDeque},
    env::current_exe,
    hash::Hash,
    iter,
    ops::Neg,
    path,
    str::FromStr,
    thread::current,
    vec,
};

fn main() {
    let input = include_str!("input.txt");
    let map: Vec<Vec<_>> = input.lines().map(|line| line.chars().collect()).collect();
    let mut sum: i64 = 0;
    // let mut parts = HashMap::new();
    let mut gears_sum: i64 = 0;
    for y in 0..map.len() {
        for x in 0..map[y].len() {
            if map[y][x] == '*' {
                let mut ratio = 1;
                let is_left = map[y]
                    .get(x.wrapping_sub(1))
                    .map_or(0, |c| c.is_digit(10) as i32);
                let mut sum_numbers = is_left;
                let is_right = map[y]
                    .get(x.wrapping_add(1))
                    .map_or(0, |c| c.is_digit(10) as i32);
                let mut top_left = 0;
                let mut top_right = 0;
                let mut bottom_left = 0;
                let mut bottom_right = 0;
                sum_numbers += is_right;
                let single_above_number = map
                    .get(y.wrapping_sub(1))
                    .map(|a| a[x])
                    .map_or(0, |c| c.is_digit(10) as i32);
                let single_below_number = map
                    .get(y.wrapping_add(1))
                    .map(|a| a[x])
                    .map_or(0, |c| c.is_digit(10) as i32);
                sum_numbers += single_above_number + single_below_number;
                if single_above_number == 0 {
                    top_right = map
                        .get(y.wrapping_sub(1))
                        .and_then(|a| a.get(x.wrapping_add(1)))
                        .map_or(0, |c| c.is_digit(10) as i32);
                    sum_numbers += top_right;
                    top_left = map
                        .get(y.wrapping_sub(1))
                        .and_then(|a| a.get(x.wrapping_sub(1)))
                        .map_or(0, |c| c.is_digit(10) as i32);
                    sum_numbers += top_left;
                }
                if single_below_number == 0 {
                    bottom_right = map
                        .get(y.wrapping_add(1))
                        .and_then(|a| a.get(x.wrapping_add(1)))
                        .map_or(0, |c| c.is_digit(10) as i32);
                    sum_numbers += bottom_right;
                    bottom_left = map
                        .get(y.wrapping_add(1))
                        .and_then(|a| a.get(x.wrapping_sub(1)))
                        .map_or(0, |c| c.is_digit(10) as i32);
                    sum_numbers += bottom_left;
                }
                if sum_numbers == 2 {
                    if is_left == 1 {
                        ratio *= get_number_from_xy(&map, y, x - 1)
                    }
                    if is_right == 1 {
                        ratio *= get_number_from_xy(&map, y, x + 1)
                    }
                    if single_above_number == 1 {
                        ratio *= get_number_from_xy(&map, y - 1, x)
                    }
                    if top_left == 1 {
                        ratio *= get_number_from_xy(&map, y - 1, x - 1)
                    }
                    if top_right == 1 {
                        ratio *= get_number_from_xy(&map, y - 1, x + 1)
                    }
                    if single_below_number == 1 {
                        ratio *= get_number_from_xy(&map, y + 1, x)
                    }
                    if bottom_left == 1 {
                        ratio *= get_number_from_xy(&map, y + 1, x - 1)
                    }
                    if bottom_right == 1 {
                        ratio *= get_number_from_xy(&map, y + 1, x + 1)
                    }
                    gears_sum += ratio;
                } else {
                    println!("faulty gear {} {}", y+1, x+1)
                }
            }
        }
    }
    for y in 0..map.len() {
        let mut current_num = 0;
        let mut valid = false;
        for x in 0..map[y].len() {
            if map[y][x] == '.' || !map[y][x].is_digit(10) {
                if valid {
                    sum += current_num;
                }
                current_num = 0;
                valid = false;

                continue;
            }
            if map[y][x].is_digit(10) {
                current_num = current_num * 10 + ((map[y][x] as i64) - '0' as i64);
                for dy in -1i32..=1 {
                    for dx in -1i32..=1 {
                        if map
                            .get(y.wrapping_add(dy as usize))
                            .and_then(|a| a.get(x.wrapping_add(dx as usize)))
                            .map_or(false, |c| *c != '.' && !c.is_digit(10) && *c != '\n')
                        {
                            valid = true;
                        }
                    }
                }
            }
        }
        if valid {
            sum += current_num;
        }
    }
    println!("{}", sum);
    println!("{}", gears_sum);
}

fn get_number_from_xy(numbers: &Vec<Vec<char>>, y: usize, x: usize) -> i64 {
    if !numbers[y][x].is_digit(10) {
        return 0;
    }
    let mut current_sum = 0;
    let mut start_x = 0;
    for dx in 0..=x {
        if !numbers[y]
            .get(x.wrapping_sub(dx))
            .map_or(false, |c| c.is_digit(10))
        {
            start_x = x - dx + 1;
            break;
        }
    }
    for xtag in start_x..numbers[y].len() {
        if !numbers[y][xtag].is_digit(10) {
            break;
        }
        current_sum = current_sum * 10 + numbers[y][xtag] as i64 - '0' as i64
    }
    current_sum
}
