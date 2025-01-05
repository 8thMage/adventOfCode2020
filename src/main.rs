fn main() {
    let input = include_str!("input.txt");
    let mut first = vec![];
    let mut second = vec![];
    for line in input.lines() {
        let mut split = line.split_whitespace();
        first.push(i64::from_str_radix(split.next().unwrap(), 10).unwrap());
        second.push(i64::from_str_radix(split.next().unwrap(), 10).unwrap());
    }
    first.sort();
    second.sort();
    let res: u64 = first
        .iter()
        .zip(second.iter())
        .map(|(a, b)| a.abs_diff(*b))
        .sum();
    let res2: usize = first
        .iter()
        .map(|&a| (a as usize) * second.iter().filter(|&&b| b == a).count())
        .sum();
    println!("{}", res);
    println!("{}", res2);
}
