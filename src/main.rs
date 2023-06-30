use std::{
    collections::{HashMap, HashSet, VecDeque},
    hash::Hash,
    str::FromStr,
    vec,
};

#[derive(Clone, Copy)]
enum ValueValue {
    Value,
    Pointer,
}

#[derive(Clone, Copy)]
enum Value {
    Value(ValueValue, i32),
    Pointer(ValueValue, usize),
}

struct Entry {
    values: Vec<Value>,
    parent: usize,
}

fn get_tree(mut line: &str) -> Vec<Entry> {
    let mut arr = vec![Entry {
        values: vec![],
        parent: 0,
    }];
    let mut current_parent = 0;
    line = &line[1..];
    while !line.is_empty() {
        if line.chars().next() == Some('[') {
            arr.push(Entry {
                values: vec![],
                parent: current_parent,
            });
            let len = arr.len();
            arr[current_parent]
                .values
                .push(Value::Pointer(ValueValue::Pointer, len - 1));
            current_parent = arr.len() - 1;
            line = &line[1..];
        } else if line.chars().next() == Some(']') {
            current_parent = arr[current_parent].parent;
            line = &line[1..];
        } else if line.chars().next() == Some(',') {
            line = &line[1..];
        } else {
            let split = line.split_once([',', ']']).unwrap();
            let number = i32::from_str(split.0).unwrap();
            arr.push(Entry {
                values: vec![Value::Value(ValueValue::Value, number)],
                parent: current_parent,
            });

            line = &line[split.0.len()..];
            let len = arr.len() - 1;
            arr[current_parent]
                .values
                .push(Value::Pointer(ValueValue::Pointer, len));
        }
    }
    arr
}

fn compare(
    line_a: &Vec<Entry>,
    line_b: &Vec<Entry>,
    line_a_current: usize,
    line_b_current: usize,
) -> i8 {
    let mut index_a = 0;
    let mut index_b = 0;
    loop {
        if index_a >= line_a[line_a_current].values.len() {
            if index_b == line_b[line_b_current].values.len() {
                return 0;
            } else {
                return -1;
            }
        }
        if index_b >= line_b[line_b_current].values.len() {
            return 1;
        }
        match (
            line_a[line_a_current].values[index_a],
            line_b[line_b_current].values[index_b],
        ) {
            (Value::Pointer(_, pointer_a), Value::Pointer(_, pointer_b)) => {
                let val = compare(line_a, line_b, pointer_a, pointer_b);
                if val != 0 {
                    return val;
                }
                index_a += 1;
                index_b += 1;
            }
            (Value::Value(_, pointer_a), Value::Value(_, pointer_b)) => {
                if pointer_a < pointer_b {
                    return -1;
                }
                if pointer_b < pointer_a {
                    return 1;
                }
                index_a += 1;
                index_b += 1;
            }
            (Value::Value(_, _), Value::Pointer(_, pointer_b)) => {
                let val = compare(line_a, line_b, line_a_current, pointer_b);
                if val != 0 {
                    return val;
                }
                index_a += 1;
                index_b += 1;
            }
            (Value::Pointer(_, pointer_a), Value::Value(_, _)) => {
                let val = compare(line_a, line_b, pointer_a, line_b_current);
                if val != 0 {
                    return val;
                }
                index_b += 1;
                index_a += 1;
            }
        }
    }
}

fn main() {
    let input = include_str!("input.txt");
    let mut lines = input.lines();
    let mut index = 1;
    let mut sum = 0;
    let mut vector = vec![];
    vector.push(get_tree("[[6]]"));
    vector.push(get_tree("[[2]]"));
    loop {
        let line_a = get_tree(lines.next().unwrap());
        let line_b = get_tree(lines.next().unwrap());
        let val = compare(&line_a, &line_b, 0, 0);
        if val == -1 {
            sum += index;
        }
        index += 1;
        vector.push(line_a);
        vector.push(line_b);
        let Some(_line_c) = lines.next() else {
            break;
        };
    }
    vector.sort_by(|a, b| match compare(a, b, 0, 0) {
        0 => std::cmp::Ordering::Equal,
        1 => std::cmp::Ordering::Greater,
        -1 => std::cmp::Ordering::Less,
        _ => panic!(),
    });
    println!("{:?}", lines.next());
    println!("{}", sum);
    let index_a = vector
        .iter()
        .position(|a| compare(a, &get_tree("[[6]]"), 0, 0) == 0)
        .unwrap();
    let index_b = vector
        .iter()
        .position(|a| compare(a, &get_tree("[[2]]"), 0, 0) == 0)
        .unwrap();
    println!("{}", (index_a + 1) * (index_b + 1));
}
