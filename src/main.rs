fn main() {
    let input = include_str!("input.txt");
    let mut sum = 0;
    'next: for line in input.lines() {
        'i: for i in 0..14 {
            let mut is_increasing = true;
            let mut is_decreasing = true;
            let split = line.split_whitespace().map(|x| x.parse::<u64>().unwrap());
            let split3 = split.clone();
            let split = split.take(i).chain(split3.skip(i + 1));
            let split2 = split.clone().skip(1);
            for (a, b) in split.zip(split2) {
                if a > b {
                    is_increasing = false;
                    if !is_decreasing {
                        continue 'i;
                    }
                }
                if a < b {
                    is_decreasing = false;
                    if !is_increasing {
                        continue 'i;
                    }
                }
                if a.abs_diff(b) < 1 || a.abs_diff(b) > 3 {
                    continue 'i;
                }
            }
            if is_increasing || is_decreasing {
                sum += 1;
                continue 'next;
            }
        }
    }
    println!("{}", sum);
}
