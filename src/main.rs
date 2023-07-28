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
    let mut elves = vec![];
    let mut elves_pos = HashSet::new();
    for elf in input
        .lines()
        .enumerate()
        .flat_map(|(y, s)| iter::repeat(y).zip(s.chars().enumerate()))
        .filter(|(y, (x, c))| *c == '#')
    {
        elves.push((
            (elf.0 as i32, elf.1 .0 as i32),
            vec![[-1i32, 0], [1, 0], [0, -1], [0, 1]],
        ));
        elves_pos.insert([elf.0 as i32, elf.1 .0 as i32]);
    }
    let mut count = 0;
    loop {
        let mut intended = vec![];
        let mut any_moved = false;
        'elf: for (index, (elf_position, elf_priority)) in elves.iter().enumerate() {
            if !(-1..=1).any(|dy| {
                (-1..=1).any(|dx| {
                    let new_pos = [elf_position.0 + dy, elf_position.1 + dx];
                    (dx != 0 || dy != 0) && elves_pos.contains(&new_pos)
                })
            }) {
                intended.push((index, [elf_position.0, elf_position.1]));
                continue;
            }
            'dir: for dir in elf_priority {
                let pos_zero = dir.iter().position(|x| *x == 0).unwrap();
                for delta in -1..=1 {
                    let mut new_delta = *dir;
                    new_delta[pos_zero] = delta;
                    let new_pos = [elf_position.0 + new_delta[0], elf_position.1 + new_delta[1]];
                    if elves_pos.contains(&new_pos) {
                        continue 'dir;
                    }
                }
                let new_pos = [elf_position.0 + dir[0], elf_position.1 + dir[1]];
                intended.push((index, new_pos));
                continue 'elf;
            }
            intended.push((index, [elf_position.0, elf_position.1]));
        }
        for (index, pos) in intended.iter() {
            if intended.iter().all(|(i2, p)| i2 == index || *p != *pos) {
                if pos[0] != elves[*index].0 .0 || pos[1] != elves[*index].0 .1 {
                    any_moved = true;

                    elves_pos.remove(&[elves[*index].0 .0, elves[*index].0 .1]);
                    elves[*index].0 .0 = pos[0];
                    elves[*index].0 .1 = pos[1];
                    elves_pos.insert([elves[*index].0 .0, elves[*index].0 .1]);
                }
            }
            let p = elves[*index].1.remove(0);
            elves[*index].1.push(p);
        }
        count += 1;
        if !any_moved {
            break;
        }
    }

    let min_y = elves_pos.iter().map(|s| s[0]).min().unwrap();
    let max_y = elves_pos.iter().map(|s| s[0]).max().unwrap();
    let min_x = elves_pos.iter().map(|s| s[1]).min().unwrap();
    let max_x = elves_pos.iter().map(|s| s[1]).max().unwrap();
    println!(
        "{}",
        ((max_y - min_y + 1) * (max_x - min_x + 1)) as usize - elves_pos.len()
    );
    println!("{}", count);
}
