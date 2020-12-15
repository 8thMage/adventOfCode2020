use std::{
    collections::{HashMap, HashSet},
    mem::zeroed,
    thread::current,
};
fn main() {
    let input = include_str!("input.txt");
//     let input = "mask = 000000000000000000000000000000X1001X
// mem[42] = 100
// mask = 00000000000000000000000000000000X0XX
// mem[26] = 1";
    let mut zero_mask = usize::MAX;
    let mut one_mask = 0;
    println!("{} {}", one_mask, zero_mask);
    let mut hash = HashMap::new();
    let mut floatingX = HashSet::new();
    floatingX.insert(0);

    for line in input.lines() {
        let subLine = &line[0..3];
        match subLine {
            "mas" => {
                one_mask = line
                    .chars()
                    .skip(7)
                    .fold(0usize, |acc, c| (acc << 1) | ((c == '1') as usize));
                zero_mask = line
                    .chars()
                    .skip(7)
                    .fold(0usize, |acc, c| (acc << 1) | ((c != '0') as usize));
                floatingX = line
                    .chars()
                    .skip(7)
                    .enumerate()
                    .filter_map(|(ind, c)| if (c == 'X') { Some(ind) } else { None })
                    .collect();
                // currentFloating.insert(0);
                // for c in line.chars().skip(7) {
                //     let mut newFloating = HashSet::new();
                //     for flt in currentFloating {
                //         newFloating.insert(flt << 1);
                //         if (c == 'X') {
                //             newFloating.insert((flt<<1)|1);
                //         }
                //     }
                //     currentFloating = newFloating;
                // }
                // floatingX = currentFloating;
            }
            "mem" => {
                // let pos:usize = line.strip_prefix("mem[").unwrap().split("]").next().unwrap().parse().unwrap();
                // let value:usize = line.split("= ").skip(1).next().unwrap().parse().unwrap();
                // let newValue = (value | one_mask) &(zero_mask);
                // hash.insert( pos, newValue );
                let pos: usize = line
                    .strip_prefix("mem[")
                    .unwrap()
                    .split("]")
                    .next()
                    .unwrap()
                    .parse()
                    .unwrap();
                let value: usize = line.split("= ").skip(1).next().unwrap().parse().unwrap();
                let new_pos = pos | one_mask;
                let mut currentFloating = HashSet::new();
                currentFloating.insert(0);
                for bit in 0..36 {
                    let mut newFloating = HashSet::new();
                    for flt in &currentFloating {
                        if floatingX.contains(&bit) {
                            newFloating.insert(flt << 1);
                            newFloating.insert((flt << 1) | 1);
                        } else {
                            newFloating.insert(flt << 1 | ((new_pos>>(35 - bit))&1));
                        }
                    }
                    currentFloating = newFloating;
                }
                for &flt in &currentFloating {
                    hash.insert(flt, value);
                    // println!("{:b} {}", flt, flt);
                }
            }
            _ => {
                panic!();
            }
        }
    }
    println!("{}", hash.iter().fold(0, |acc, (_, val2)| acc + val2));
}
