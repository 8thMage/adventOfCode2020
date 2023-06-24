use std::{
    collections::{HashMap, HashSet, VecDeque},
    str::FromStr,
};

fn main() {
    let input = include_str!("input.txt");
    let mut cycle = 0;
    let mut delay = 0;
    let mut reg = 1_i32;
    let mut adder = 0;
    let mut sum = 0;
    let mut lines = input.lines();
    let mut screen = vec![vec!['.'; 40]; 6];
    loop {
        if delay == 0 {
            let Some(line) = lines.next() else {
                break;
            };
            
            if line == "noop" {
                delay = 1;
            } else {
                adder = i32::from_str(line.split_once("addx ").unwrap().1).unwrap();
                delay = 2;
            }
        }
        if ((cycle % 40) - reg).abs() <= 1 {
            screen[cycle as usize / 40 ][cycle as usize % 40] = '#';
        }
        cycle += 1;

        if cycle% 40 == 20 {
            sum+= cycle * reg;
        }

        delay -= 1;
        if delay == 0 {
            reg += adder;
            adder = 0;
        }
    }
    println!("{}", sum);
    for line in screen.iter() {
        for char in line.iter() {
            print!("{}", char);
        }
        println!("");
    }
}
