use std::collections::HashSet;

fn main() {
    let input = include_str!("input.txt");
    let mut sum = 0;
    // for line in input.lines() {
    //     let split_line = line.split_at(line.len() / 2);
    //     println!("{}", split_line.0);
    //     let mut hash_1 = HashSet::new();
    //     let mut hash_2 = HashSet::new();
    //     for char in split_line.0.chars() {
    //         hash_1.insert(char);
    //     }
    //     for char in split_line.1.chars() {
    //         hash_2.insert(char);
    //     }
    //     let share = hash_1.intersection(&hash_2).next().unwrap();
    //     println!("{}", share);
    //     if share.is_uppercase() {
    //         sum += 26;
    //     }
    //     sum += share.to_lowercase().next().unwrap() as usize - 'a' as usize + 1;
    // }
    let mut lineA = None;
    let mut lineB = None;

    for line in input.lines() {
        if lineA.is_none() {
            lineA = Some(line);
            continue;
        }
        if lineB.is_none() {
            lineB = Some(line);
            continue;
        }
        // let split_line = line.split_at(line.len() / 2);
        let mut hash_1 = HashSet::new();
        let mut hash_2 = HashSet::new();
        let mut hash_3 = HashSet::new();
        for char in lineA.unwrap().chars() {
            hash_1.insert(char);
        }
        for char in lineB.unwrap().chars() {
            hash_2.insert(char);
        }
        for char in line.chars() {
            hash_3.insert(char);
        }
        let hash_12 = hash_1
            .intersection(&hash_2)
            .map(|c| *c)
            .collect::<HashSet<char>>();
        let share = hash_12.intersection(&hash_3).next().unwrap();
        println!("{}", share);
        if share.is_uppercase() {
            sum += 26;
        }
        sum += share.to_lowercase().next().unwrap() as usize - 'a' as usize + 1;
        lineA = None;
        lineB = None;
    }
    println!("{}", sum);
}
