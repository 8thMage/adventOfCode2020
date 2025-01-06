fn main() {
    let input = include_str!("input.txt");
    let sub = input.split("mul(");
    let sub2 = input
        .split("do()")
        .map(|s| s.split_once("don't").unwrap_or((s, "")).0)
        .flat_map(|s| s.split("mul("));
    let mut sum = 0;
    let mut is_activated = true;
    let mut is_first = true;
    for subs in sub {
        if is_activated && !is_first {
            let chars = subs.chars();
            let (d, n) = chars
                .clone()
                .take_while(|c| c.is_numeric())
                .fold((0, 0u64), |acc, c| {
                    (acc.0 + 1, acc.1 * 10 + c.to_digit(10).unwrap() as u64)
                });
            if chars.clone().skip(d).next() != Some(',') {
                continue;
            }
            let (d2, n2) = chars
                .clone()
                .skip(d + 1)
                .take_while(|c| c.is_numeric())
                .fold((0, 0u64), |acc, c| {
                    (acc.0 + 1, acc.1 * 10 + c.to_digit(10).unwrap() as u64)
                });
            if chars.clone().skip(d + d2 + 1).next() != Some(')') {
                continue;
            }
            sum += n * n2;
        }
        is_first = false;
        let pos_dont = subs.rfind("don't()");
        let pos_do = subs.rfind("do()");
        if pos_dont.is_some() && pos_do.is_none() {
            is_activated = false;
        } else if pos_do.is_some() && pos_dont.is_none() {
            is_activated = true;
        } else if pos_do.is_some() && pos_dont.is_some() {
            if pos_do.unwrap() > pos_dont.unwrap() {
                is_activated = true;
            } else {
                is_activated = false;
            }
        }
    }
    println!("{}", sum);
}
