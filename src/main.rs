fn main() {
    let input = include_str!("input.txt");
    //         let input = "nop +0
    // acc +1
    // jmp +4
    // acc +3
    // jmp -3
    // acc -99
    // acc +1
    // jmp -4
    // acc +6";
    let mut instructions = input
        .lines()
        .map(|line| (&line[0..3], line[4..].parse::<i64>().unwrap(), 0))
        .collect::<Vec<(&str, i64, i64)>>();

    for instruction in 0..instructions.len() {
        let mut acc = 0i64;
        let mut ip = 0i64;
        match instructions[instruction].0 {
            "nop" => instructions[instruction].0 = "jmp",
            "jmp" => instructions[instruction].0 = "nop",
            _ => continue,
        };
        while ip as usize != instructions.len() && instructions[ip as usize].2 == 0 {
            let mut currentInstruction = &mut instructions[ip as usize];
            (*currentInstruction).2 += 1;
            match currentInstruction.0 {
                "nop" => ip += 1,
                "acc" => {
                    acc += currentInstruction.1 as i64;
                    ip += 1
                }
                "jmp" => ip = ip + currentInstruction.1,
                _ => panic!(),
            }
        }
        if ip as usize == instructions.len() {
            println!("{}", acc);
            break;
        }
        match instructions[instruction].0 {
            "nop" => instructions[instruction].0 = "jmp",
            "jmp" => instructions[instruction].0 = "nop",
            _ => continue,
        }
        for i in 0..instructions.len() {
            instructions[i].2 = 0;
        }
    }
    // println!("{}", acc);
}
