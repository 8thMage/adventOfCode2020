use std::collections::HashMap;
fn main() {
    let input = include_str!("input.txt");
//     let input = 
// "shiny gold bags contain 2 dark red bags.
// dark red bags contain 2 dark orange bags.
// dark orange bags contain 2 dark yellow bags.
// dark yellow bags contain 2 dark green bags.
// dark green bags contain 2 dark blue bags.
// dark blue bags contain 2 dark violet bags.
// dark violet bags contain no other bags.";
    let dependendt = input
        .lines()
        .map(|rule: &str| {
            let first = rule.split_terminator("contain").next().unwrap();
            let color = first.split_terminator(" bags").next().unwrap();

            let second = rule.split_terminator("contain").skip(1).next().unwrap();
            let deps = second
                .split_terminator(",")
                .map(|specific_bag: &str| {
                    if specific_bag.contains("no") {
                        None
                    } else {
                        let number = i64::from_str_radix(
                            specific_bag.split_whitespace().next().unwrap(),
                            10,
                        );
                        let color = specific_bag
                            .trim_left_matches(|c| c <= '9' && c >= '0' || c == ' ')
                            .split_terminator(" bag")
                            .next();
                        Some((number.unwrap(), color.unwrap()))
                    }
                })
                .collect::<Vec<Option<(i64, &str)>>>();
            (color, deps)
        }).collect::<std::collections::HashMap<&str, Vec<Option<(i64, &str)>>>>();
    let mut checks: std::collections::hash_set::HashSet<&str> =
        std::collections::hash_set::HashSet::new();
    checks.insert("shiny gold");
    findContains(&dependendt, &mut checks);
    let mut countPerBag = HashMap::new();
    let sum = findNumber(&dependendt, &mut countPerBag,  "shiny gold");

    println!("{}", checks.len() - 1);
    println!("sum {} ", sum);
}

fn findContains<'a>(dependents: &'a HashMap<&str, Vec<Option<(i64, &str)>>>,
    checks :&mut std::collections::HashSet<&'a str>) {
    for deps in dependents {
        if checks.contains(deps.0) {
            continue;
        }
        for dep in deps.1 {
            if let Some(x) = dep {
                if checks.contains(x.1) {
                    checks.insert(deps.0);
                    findContains(dependents, checks);
                    break;
                }
            }
        }
    } 
}

fn findNumber<'a>(
    dependents: &'a HashMap<&str, Vec<Option<(i64, &str)>>>,
    countPerBag: &mut HashMap<&'a str, i64>,
    specificColor:&'a str,
) -> i64 {
    if countPerBag.contains_key(specificColor) {
        return countPerBag[specificColor];
    }
    let mut sum = 0;
    for dep in &dependents[specificColor] {
        if dep.is_none() {
            let cloned = specificColor.clone();
            countPerBag.insert(specificColor, 0);
            return 0;
        } else {
            sum += dep.unwrap().0 * (1 + findNumber(dependents, countPerBag, dep.unwrap().1));
        }
    }
    countPerBag.insert(specificColor, sum);
    sum
}
