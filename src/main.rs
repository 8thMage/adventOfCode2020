use std::collections::{HashSet, VecDeque};

fn main() {
    let input = include_str!("input.txt");
    let number_line = input.lines().position(|line| !line.contains("[")).unwrap();
    let number_of_stacks = (input.lines().skip(number_line).next().unwrap().len() - 3) / 4 + 1;
    let mut stacks = vec![VecDeque::new(); number_of_stacks];
    for line in input.lines().take(number_line) {
        let mut str = line;
        let mut start_stack = 0;
        loop {
            let pos = str.find("[");
            if pos.is_none() {
                break;
            }
            let pos = pos.unwrap();
            let stack = if pos == 0 { 0 } else { (pos - 4) / 4 + 1 } + start_stack;
            let number = str.chars().skip(pos + 1).next().unwrap();
            stacks[stack].push_back(number);
            if pos + 4 >= str.len() {
                break;
            }
            str = &str[pos + 4..];
            start_stack = stack + 1
        }
    }
    let mut stacks2 = stacks.clone();

    for line in input.lines().skip(number_line + 2) {
        let mut line = line.split(" ");
        line.next();
        let num = line.next().unwrap();
        let num = usize::from_str_radix(num, 10).unwrap();
        line.next();
        let start = line.next().unwrap();
        let start = usize::from_str_radix(start, 10).unwrap();
        line.next();
        let to = line.next().unwrap();
        let to = usize::from_str_radix(to, 10).unwrap();
        for _ in 0..num {
            let char = stacks[start - 1].pop_front().unwrap();
            stacks[to - 1].push_front(char);
        }
        let mut vec = vec![];
        for _ in 0..num {
            let char = stacks2[start - 1].pop_front().unwrap();
            vec.push(char);
        }

        for _ in 0..num {
            stacks2[to - 1].push_front(vec.pop().unwrap());
        }
    }
    let res = stacks
        .iter()
        .map(|s| s.front().unwrap())
        .collect::<String>();
    println!("{}", res);

    let res = stacks2
        .iter()
        .map(|s| s.front().unwrap())
        .collect::<String>();
    println!("{}", res);
}
