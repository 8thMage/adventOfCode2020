use std::collections::HashSet;

fn main() {
    let input = include_str!("input.txt");
    let mut count = 0;
    let mut count2 = 0;
    for line in input.lines() {
        let v: Vec<i64> = line
            .split(",")
            .flat_map(|s| s.split("-").map(|s| i64::from_str_radix(s, 10).unwrap()))
            .collect();
        if (v[0] >= v[2] && v[1] <= v[3]) || (v[2] >= v[0] && v[3] <= v[1]) {
            count += 1;
        }
        if !(v[0] > v[3] || v[1] < v[2]) {
            count2 += 1;
        }
    }
    println!("{}", count);
    println!("{}", count2);
}
