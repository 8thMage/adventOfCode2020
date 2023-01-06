fn main() {
    let input = include_str!("input.txt");
    let mut sum = 0;
    for line in input.lines() {
        let strs = line.split_once(" ").unwrap_or(("0","0"));
        let op = strs.0.chars().next().unwrap() as usize - 'A' as usize;
        let you = strs.1.chars().next().unwrap() as usize - 'X' as usize;
        let you2 = (2+ op + you)% 3;
        sum += you2+1;
        // println!("  {}",sum);

        sum += match (6+you2 - op) % 3 {
            0 => 3 ,
            1 => 6,
            2 => 0,
            _=>0,
        };
        // println!("{}",sum);
    }
    println!("{}",sum);
}