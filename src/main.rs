use std::{
    collections::{HashMap, HashSet, VecDeque},
    str::FromStr,
};

struct Directory {
    kids: HashMap<String, usize>,
    parent: usize,
    files: Vec<u64>,
}

fn main() {
    let input = include_str!("input.txt");
    let mut directory_array: Vec<Directory> = vec![];
    directory_array.push(Directory {
        kids: HashMap::new(),
        parent: 0,
        files: vec![],
    });
    let mut current_parent = 0;
    for line in input.lines() {
        match line.split(" ").next() {
            Some("$") => match line.split(" ").skip(1).next() {
                Some("cd") => {
                    let dir = line.split(" ").skip(2).next();
                    if dir == Some("..") {
                        current_parent = directory_array[current_parent].parent;
                    } else {
                        current_parent = *directory_array[current_parent]
                            .kids
                            .get(dir.unwrap())
                            .unwrap();
                    }
                }
                Some("ls") => {}
                Some(_) | None => {
                    panic!();
                }
            },
            Some("dir") => {
                let dir = line.split(" ").skip(1).next();
                let len = directory_array.len();
                directory_array[current_parent]
                    .kids
                    .insert(String::from_str(dir.unwrap()).unwrap(), len);
                directory_array.push(Directory {
                    kids: HashMap::new(),
                    parent: current_parent,
                    files: vec![],
                })
            }
            Some(number) => directory_array[current_parent]
                .files
                .push(u64::from_str(number).unwrap()),
            None => {
                panic!();
            }
        }
    }
    let mut global_sum = 0;
    let mut min_suff = u64::MAX;
    let sum_of_outmost = calculate_sum(&directory_array, 0);
    println!("{}",30000000 - (70000000 - sum_of_outmost) );
    for i in 0..directory_array.len() {
        let sum = calculate_sum(&directory_array, i);
        if sum <= 100000 {
            global_sum += sum;
        }
        println!("{}, {}", sum, min_suff);
        if sum >= 30000000 - (70000000 - sum_of_outmost)  {
            min_suff = sum.min(min_suff);
        }
    }
    println!("{}", global_sum);
    println!("{}", min_suff);
}

fn calculate_sum(directories: &Vec<Directory>, index: usize) -> u64 {
    let mut sum = 0;
    for directory in directories[index].kids.iter() {
        sum += calculate_sum(directories, *directory.1);
    }
    sum += directories[index].files.iter().sum::<u64>();
    sum
}
