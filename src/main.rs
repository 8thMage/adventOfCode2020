use std::{collections::{HashMap, HashSet}, thread::current};

fn main() {
    let input = include_str!("input.txt");
    let mut parameters0 = vec![];
    let mut parameters1 = vec![];
    let mut parameters2 = vec![];
    for (line_number, line) in input.lines().enumerate() {
        let other_line = input.lines().nth(line_number % 18).unwrap();
        if line_number % 18 == 4 || line_number % 18 == 5 || line_number % 18 == 15 {
            assert_eq!(
                line.split_once(" ").unwrap().0,
                other_line.split_once(" ").unwrap().0
            );

            assert_eq!(
                line.split_once(" ").unwrap().1.split_once(" ").unwrap().0,
                other_line
                    .split_once(" ")
                    .unwrap()
                    .1
                    .split_once(" ")
                    .unwrap()
                    .0
            );
            if line_number % 18 == 4 {
                parameters2.push(
                    i64::from_str_radix(
                        line.split_once(" ").unwrap().1.split_once(" ").unwrap().1,
                        10,
                    )
                    .unwrap(),
                )
            } else if line_number % 18 == 5 {
                parameters0.push(
                    i64::from_str_radix(
                        line.split_once(" ").unwrap().1.split_once(" ").unwrap().1,
                        10,
                    )
                    .unwrap(),
                )
            } else {
                parameters1.push(
                    i64::from_str_radix(
                        line.split_once(" ").unwrap().1.split_once(" ").unwrap().1,
                        10,
                    )
                    .unwrap(),
                )
            }
        } else if line_number % 18 == 0 {
            assert_eq!(line, "inp w");
        } else {
            assert_eq!(line, other_line);
        }
    }
    let max = u64::max_value();
    let mut set = HashSet::new();
    println!(
        "{}",
        tree(
            &[
                parameters0.clone(),
                parameters1.clone(),
                parameters2.clone()
            ],
            0,
            0,
            0,
            &mut set,
        )
        .unwrap()
    );
    println!("run");
    for input_value in (11111111111111..100000000000000i64).rev() {
        let mut reg_w = 0;
        let mut reg_z = 0;
        let mut input_char = 0;
        let mut div = 10000000000000i64;
        for i in 0..14 {
            reg_w = (input_value / div) % 10;
            div /= 10;
            reg_z = if reg_z % 26 + parameters0[i] != reg_w {
                (reg_z / parameters2[i]) * 26 + reg_w + parameters1[i]
            } else {
                reg_z / parameters2[i]
            }
        }
        if reg_z == 0 {
            let x = 0;
        }
    }

    // let mut map = HashMap::new();
    let mut input_lines = vec![];
    for input_value in (11111111111111..100000000000000i64).rev() {
        let mut regs = [0; 4];
        let input_value = input_value.to_string();
        let mut input_char = 0;
        let mut known_values = 0;
        let mut skip_number = 0;
        // while map.contains_key(&input_value[0..known_values + 1]) {
        //     skip_number = input_lines[known_values + 1] + 1;
        //     input_char += 1;
        //     regs = map[&input_value[0..known_values + 1]];
        //     known_values += 1;
        // }
        for (line_number, line) in input.lines().enumerate().skip(skip_number) {
            match line.split_once(" ").unwrap().0 {
                "inp" => {
                    let register = get_register(line.split_once(" ").unwrap().1);
                    regs[register] = i64::from_str_radix(
                        &input_value.chars().nth(input_char).unwrap().to_string(),
                        10,
                    )
                    .unwrap();
                    input_char += 1;
                    // if !map.contains_key(&input_value[0..input_char]) {
                    //     map.insert(input_value[0..input_char].to_owned(), regs);
                    // }
                    if input_lines.len() < 14 {
                        input_lines.push(line_number);
                    }
                }

                "add" => {
                    let register =
                        get_register(line.split_once(" ").unwrap().1.split_once(" ").unwrap().0);
                    let second_str = line.split_once(" ").unwrap().1.split_once(" ").unwrap().1;
                    let value = i64::from_str_radix(&second_str, 10).unwrap_or_else(|_| {
                        let second_reg = get_register(
                            line.split_once(" ").unwrap().1.split_once(" ").unwrap().1,
                        );
                        regs[second_reg]
                    });
                    regs[register] = regs[register] + value;
                }
                "mul" => {
                    let register =
                        get_register(line.split_once(" ").unwrap().1.split_once(" ").unwrap().0);
                    let second_str = line.split_once(" ").unwrap().1.split_once(" ").unwrap().1;
                    let value = i64::from_str_radix(&second_str, 10).unwrap_or_else(|_| {
                        let second_reg = get_register(
                            line.split_once(" ").unwrap().1.split_once(" ").unwrap().1,
                        );
                        regs[second_reg]
                    });
                    regs[register] = regs[register] * value;
                }
                "div" => {
                    let register =
                        get_register(line.split_once(" ").unwrap().1.split_once(" ").unwrap().0);
                    let second_str = line.split_once(" ").unwrap().1.split_once(" ").unwrap().1;
                    let value = i64::from_str_radix(&second_str, 10).unwrap_or_else(|_| {
                        let second_reg = get_register(
                            line.split_once(" ").unwrap().1.split_once(" ").unwrap().1,
                        );
                        regs[second_reg]
                    });
                    regs[register] = regs[register] / value;
                }

                "mod" => {
                    let register =
                        get_register(line.split_once(" ").unwrap().1.split_once(" ").unwrap().0);
                    let second_str = line.split_once(" ").unwrap().1.split_once(" ").unwrap().1;
                    let value = i64::from_str_radix(&second_str, 10).unwrap_or_else(|_| {
                        let second_reg = get_register(
                            line.split_once(" ").unwrap().1.split_once(" ").unwrap().1,
                        );
                        regs[second_reg]
                    });
                    regs[register] = regs[register] % value;
                }
                "eql" => {
                    let register =
                        get_register(line.split_once(" ").unwrap().1.split_once(" ").unwrap().0);
                    let second_str = line.split_once(" ").unwrap().1.split_once(" ").unwrap().1;
                    let value = i64::from_str_radix(&second_str, 10).unwrap_or_else(|_| {
                        let second_reg = get_register(
                            line.split_once(" ").unwrap().1.split_once(" ").unwrap().1,
                        );
                        regs[second_reg]
                    });
                    regs[register] = (regs[register] == value) as i64;
                }

                _ => {
                    panic!()
                }
            }
        }
        if regs[3] == 0 {
            println!("{}", input_value);
            break;
        }
    }
}

fn get_register(str: &str) -> usize {
    match str {
        "w" => 0,
        "x" => 1,
        "y" => 2,
        "z" => 3,
        _ => panic!(),
    }
}

fn tree(
    parameters: &[Vec<i64>; 3],
    index: usize,
    current_z: i64,
    current_value: i64,
    map: &mut HashSet<(i64, usize)>,
) -> Option<i64> {
    if map.contains(&(current_z, index)) {
        return None;
    }
    if index == 14 {
        // print_number(current_z);
        // println!("");
        if current_z == 0 {
            return Some(current_value);
        }
        map.insert((current_z, index));
        return None;
    }
    for i in (1i64..10) {
        let reg_z = if current_z % 26 + parameters[0][index] != i {
            (current_z / parameters[2][index]) * 26 + i + parameters[1][index]
        } else {
            current_z / parameters[2][index]
        };

        let new_current_value = 10 * current_value + i;
        if let Some(value) = tree(parameters, index + 1, reg_z, new_current_value, map) {
            return Some(value);
        }
    }
    map.insert((current_z, index));
    return None;
}

fn print_number(s: i64) {
    if s == 0 {
        return;
    }
    print_number(s/26);
    print!(
        "{}",
        String::from_utf8(vec![('a' as u8 + (s % 26) as u8)]).unwrap()
    );
}