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
    {
        let mut registers = HashMap::new();
        let mut operations = vec![];
        'big: for line in input.lines() {
            let (monkey, op) = line.split_once(": ").unwrap();
            for oper in [" + ", " - ", " * ", " / "] {
                if op.contains(oper) {
                    let (monkey1, monkey2) = op.split_once(oper).unwrap();
                    operations.push((false, monkey, monkey1, monkey2, oper));
                    registers.insert(monkey, None);
                    continue 'big;
                }
            }
            let value = i64::from_str(op).unwrap();
            registers.insert(monkey, Some(value));
        }

        loop {
            for (passed, monkey, monkey1, monkey2, op) in operations.iter_mut() {
                if !*passed && registers[monkey1].is_some() && registers[monkey2].is_some() {
                    let value = match *op {
                        " + " => registers[monkey1].unwrap() + registers[monkey2].unwrap(),
                        " - " => registers[monkey1].unwrap() - registers[monkey2].unwrap(),
                        " * " => registers[monkey1].unwrap() * registers[monkey2].unwrap(),
                        " / " => registers[monkey1].unwrap() / registers[monkey2].unwrap(),
                        _ => panic!(""),
                    };
                    registers.insert(monkey, Some(value));
                    *passed = true;
                }
            }
            if let Some(Some(v)) = registers.get("root") {
                println!("{}", v);
                break;
            }
        }
    }

    {
        let mut registers = HashMap::new();
        let mut operations = vec![];
        'big: for line in input.lines() {
            let (monkey, op) = line.split_once(": ").unwrap();
            for oper in [" + ", " - ", " * ", " / "] {
                if op.contains(oper) {
                    let (monkey1, monkey2) = op.split_once(oper).unwrap();
                    operations.push((false, monkey, monkey1, monkey2, oper));
                    registers.insert(monkey, RegValue::None);
                    continue 'big;
                }
            }
            if monkey != "humn" {
                let value = i128::from_str(op).unwrap();
                registers.insert(monkey, RegValue::Simple(value));
            } else {
                registers.insert(
                    monkey,
                    RegValue::Affine {
                        mul: 1,
                        add: 0,
                        div: 1,
                    },
                );
            }
        }

        loop {
            for (passed, monkey, monkey1, monkey2, op) in operations.iter_mut() {
                if !*passed
                    && registers[monkey1] != RegValue::None
                    && registers[monkey2] != RegValue::None
                {
                    let monkey_1_val = registers[monkey1];
                    let monkey_2_val = registers[monkey2];
                    if monkey == &"root" {
                        println!(
                            "monkey1 {:?} monkey2 {:?}",
                            registers[monkey1], registers[monkey2]
                        );
                        match (monkey_1_val, monkey_2_val) {
                            (
                                RegValue::Affine {
                                    mut mul,
                                    mut add,
                                    mut div,
                                },
                                RegValue::Simple(monkey_2_val),
                            ) => {
                                let v = (monkey_2_val * div - add) / mul;
                                println!("{}", v);
                            }
                            _ => panic!(""),
                        }
                    }
                    match (monkey_1_val, monkey_2_val) {
                        (RegValue::Simple(monkey_1_val), RegValue::Simple(monkey_2_val)) => {
                            let value = match *op {
                                " + " => monkey_1_val + monkey_2_val,
                                " - " => monkey_1_val - monkey_2_val,
                                " * " => monkey_1_val * monkey_2_val,
                                " / " => monkey_1_val / monkey_2_val,
                                _ => panic!(""),
                            };
                            registers.insert(monkey, RegValue::Simple(value));
                            *passed = true;
                        }
                        (
                            RegValue::Affine {
                                mut mul,
                                mut add,
                                mut div,
                            },
                            RegValue::Simple(monkey_2_val),
                        ) => {
                            match *op {
                                " + " => add = add + monkey_2_val * div,
                                " - " => add = add - monkey_2_val * div,
                                " * " => {
                                    mul = mul * monkey_2_val;
                                    add *= monkey_2_val
                                }
                                " / " => div *= monkey_2_val,
                                _ => panic!(""),
                            };
                            registers.insert(monkey, RegValue::Affine { mul, add, div });
                            *passed = true;
                        }
                        (
                            RegValue::Simple(monkey_1_val),
                            RegValue::Affine {
                                mut mul,
                                mut add,
                                mut div,
                            },
                        ) => {
                            match *op {
                                " + " => add = add + monkey_1_val * div,
                                " - " => {
                                    add = monkey_1_val * div - add;
                                    mul *= -1
                                }
                                " * " => {
                                    mul = mul * monkey_1_val;
                                    add *= monkey_1_val
                                }
                                " / " => {
                                    panic!("")
                                }
                                _ => panic!(""),
                            };
                            registers.insert(monkey, RegValue::Affine { mul, add, div });
                            *passed = true;
                        }
                        (_, _) => panic!(""),
                    }
                }
            }
            // if let RegValue::Affine(Some(v)) = registers.get("root") {
            //     println!("{}", v);
            //     break;
            // }
        }
    }
}
