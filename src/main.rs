fn main() {
    let input = include_str!("input.txt");
    //     let input =
    // "BFFFBBFRRR: row 70, column 7, seat ID 567.
    // FFFBBBFRRR: row 14, column 7, seat ID 119.
    // BBFFBBFRLL: row 102, column 4, seat ID 820.
    // ";
    let input = input.replace("B", "1");
    let input = input.replace("F", "0");
    let input = input.replace("R", "1");
    let input = input.replace("L", "0");

    let mut vector: Vec<bool> = Vec::new();
    vector.resize(128 * 8, false);
    for line in input.lines() {
        vector[toInt(line.chars())] = true;
    }
    let first = vector
        .iter()
        .enumerate()
        .filter(|&val| *val.1)
        .max_by_key(|&val| val.0);
    println!("{}", first.unwrap().0);
    let iter = vector
        .iter()
        .skip(1)
        .take(vector.len() - 2)
        .enumerate()
        .filter(|&(ind, val)| !val && vector[ind] && vector[ind + 2])
        .next();
    println!("{}", iter.unwrap().0 + 1);
}

fn toInt<I>(iter: I) -> usize
where
    I: Iterator<Item = char>,
{
    iter.fold(0, |acc, val| acc * 2 + (val as usize - '0' as usize))
}
