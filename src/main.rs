use std::str::FromStr;

fn main() {
    let input = include_str!("input.txt");
    let (rules, lines) = input.split_once("\n\n").unwrap();
    let mut count = 0;
    let mut count2 = 0;
    'line: for (ind, s) in lines.lines().enumerate() {
        println!("{}/{}", ind, lines.lines().count());
        let v = s.split(",").collect::<Vec<_>>();
        let mut new_v = v.clone();
        let mut should_count = true;
        for (i, a) in v.iter().enumerate() {
            for b in v[i + 1..].iter() {
                if rules.contains((String::from_str(b).unwrap() + "|" + a).as_str()) {
                    should_count = false;
                }
            }
        }
        'stop: loop {
            for i in 0..new_v.len() {
                for j in i + 1..v.len() {
                    let a = new_v[i];
                    let b = new_v[j];
                    if rules.contains((String::from_str(b).unwrap() + "|" + a).as_str()) {
                        new_v.swap(i, j);
                        continue 'stop;
                    }
                }
            }
            break;
        }
        if should_count {
            count += v[v.len() / 2].parse::<u64>().unwrap();
        } else {
            count2 += new_v[v.len() / 2].parse::<u64>().unwrap();
        }
    }
    println!("{}", count);
    println!("{}", count2);
}
