fn main() {
    let input = include_str!("input.txt");
//     let input = "abc

// a
// b
// c

// ab
// ac

// a
// a
// a
// a

// b";
    let groups = input.split_terminator("\r\n\r\n");
    // let groups = input.split_terminator("\n\n");
    let first = groups.fold(0u32, |acc, group| {
        let sum = group.split_whitespace().fold(0u32, |acc, s| acc | s.chars().fold(0u32, |acc, c| acc | 1 << (c as usize - 'a' as usize)));
        acc + sum.count_ones()
    });
    let groups = input.split_terminator("\r\n\r\n");
    let second = groups.fold(0u32, |acc, group| {
        let sum = group.split_whitespace().fold(u32::max_value(), |acc, s| acc & s.chars().fold(0u32, |acc, c| acc | 1 << (c as usize - 'a' as usize)));
        acc + sum.count_ones()
    });
    println!("{}", first);
    println!("{}", second);
}
