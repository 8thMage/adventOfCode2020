fn main() {
    let input = include_str!("input.txt");
    let mut max_sum = 0;
    let mut sums = vec![];
    for line_groutp in input.split("\n\n") {
        let mut sum = 0;
        for line in line_groutp.lines() {
            sum += u64::from_str_radix(line, 10).unwrap();
        }
        sums.push(sum);
        max_sum = sum.max(max_sum);
    }
    sums.sort();
    
    println!("{}", max_sum);
    println!("{}", sums[sums.len() - 1] + sums[sums.len() - 2] +sums[sums.len() - 3] );
}