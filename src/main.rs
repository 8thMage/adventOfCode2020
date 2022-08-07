use std::{collections::HashMap, hash::Hash, default};

fn main() {
    let input = include_str!("input.txt");
    let mut reactor: Vec<(i64, Vec<(i64, Vec<(i64, bool)>)>)> = vec![];
    for line in input.lines() {
        let state = line.contains("on");
        let reduced_line = line.split_once(" ").unwrap().1;
        let mut axes = reduced_line.split(",").map(|s| {
            s.split_once("..").map(|s| {
                (
                    i64::from_str_radix(&s.0[2..], 10).unwrap(),
                    i64::from_str_radix(s.1, 10).unwrap(),
                )
            })
        });
        let (min_x, max_x) = axes.next().unwrap().unwrap();
        let (min_y, max_y) = axes.next().unwrap().unwrap();
        let (min_z, max_z) = axes.next().unwrap().unwrap();
        let z_min_position = partition(&mut reactor,min_z);
        let z_max_position = partition(&mut reactor,max_z + 1);
        for z in z_min_position.. z_max_position {
            let y_min_position = partition(&mut reactor[z].1,min_y);
            let y_max_position = partition(&mut reactor[z].1,max_y + 1);
            for y in y_min_position.. y_max_position {
                let x_min_position = partition(&mut reactor[z].1[y].1,min_x);
                let x_max_position = partition(&mut reactor[z].1[y].1,max_x + 1);
                for x in x_min_position.. x_max_position {
                    reactor[z].1[y].1[x].1 = state;
                }
            }                
        }
    }
    let mut acc = 0;
    for z in 0..reactor.len().saturating_sub(1) {
        let mul_z = reactor[z + 1].0 - reactor[z].0;
        for y in 0..reactor[z].1.len().saturating_sub(1) {
            let mul_y = reactor[z].1[y + 1].0 - reactor[z].1[y].0;
            for x in 0..reactor[z].1[y].1.len().saturating_sub(1) {
                let mul_x = reactor[z].1[y].1[x+1].0 - reactor[z].1[y].1[x].0;
                acc += mul_x * mul_y * mul_z * reactor[z].1[y].1[x].1 as i64;
            }
        }
    }
    println!("{}", acc);
}

fn partition<T: Default + Clone> (vec: &mut Vec<(i64, T)>, value: i64) -> usize {
    let position = vec.partition_point(|b| b.0 < value);
    // if vec.get(position).is_some() && vec[position].0 == value {
    //     return position;
    // }
    if position == 0 {
        vec.insert(position, (value, T::default()));
    } else if position == vec.len() {
        vec.insert(position, (value, T::default()));
    } else {
        vec.insert(position, (value, vec[position - 1].1.clone()));
    }
    position
}