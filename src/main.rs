fn main() {
    let input = include_str!("input.txt");
    //     let input =
    // "BFFFBBFRRR: row 70, column 7, seat ID 567.
    // FFFBBBFRRR: row 14, column 7, seat ID 119.
    // BBFFBBFRLL: row 102, column 4, seat ID 820.
    // ";
    let vector: std::collections::HashSet<u64> = input
        .replace("B", "1")
        .replace("F", "0")
        .replace("R", "1")
        .replace("L", "0")
        .lines()
        .map(|s| u64::from_str_radix(&s, 2).unwrap())
        .collect();

    let first = vector.iter().max();
    println!("{}", first.unwrap());
    let second = vector
        .iter()
        .cloned()
        .filter(|val| vector.contains(&(val + 2)) && !vector.contains(&(val + 1)))
        .next();
    println!("{}", second.unwrap());
}
