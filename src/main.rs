use std::collections::HashMap;
fn main() {
    let input = include_str!("input.txt");
//     let input = 
// "F10
// N3
// F7
// R90
// F11";
let mut heading = (10, 1);
let (mut x,mut y) = (0,0); 
for instruction in input.lines() {
    let command = instruction.chars().next().unwrap();
    let mut number = instruction.chars().skip(1).collect::<String>().parse::<i64>().unwrap();
    match command {
        'N' => heading.1 += number,
        'S' => heading.1 -= number,
        'E' => heading.0 += number,
        'W' => heading.0 -= number,
        'F' => { 
            x += heading.0 * number;
            y += heading.1 * number;
        },
        'R' => while number > 0 {
            let (newx,newy) = (heading.1, -heading.0);
            heading = (newx, newy);
            number -= 90;
        } 
        'L' => while number > 0 {
            let (newx,newy) = (-heading.1, heading.0);
            heading = (newx, newy);
            number -= 90;
        } 
        _ => panic!(),

    }
}
println!("{}", x.abs() + y.abs());
}
