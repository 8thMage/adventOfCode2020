fn main() {
    let input = include_str!("input.txt");
//     let input = "1-3 a: abcde
// 1-3 b: cdefg
// 2-9 c: ccccccccc";
    // let values = input;
    // .map(|str| str.split_terminator('-').collect())
    // println!("{}", input);
    let inputLines= input.lines();
    let mut countValid = 0;
    let mut countValid2 = 0;
    for line in inputLines{
        let values:Vec<&str> = line.split_whitespace().map(|str| str.trim_matches(':')).collect();
        let policy:Vec<i32> = values[0].split_terminator('-').map(|str| str.parse::<i32>().unwrap()).collect();
        let check = values[1].chars().next().unwrap();;
        let count = values[2].chars().filter(|&c| c==check).count();
        if count >= policy[0] as usize && count <= policy[1] as usize {
            countValid = countValid+1;
        }
        let first = values[2].chars().nth(policy[0] as usize -1).unwrap() == check;
        let second = values[2].chars().nth(policy[1] as usize -1).unwrap() == check;
        if (first && !second) || (second && !first) {
            countValid2 = countValid2 +1;
        }
    }

    println!("{}", countValid);
    println!("{}", countValid2);
}
