fn main() {
    let input = include_str!("input.txt");
    //     let input = "1-3 a: abcde
    // 1-3 b: cdefg
    // 2-9 c: ccccccccc";
    // let values = input;
    // .map(|str| str.split_terminator('-').collect())
    // println!("{}", input);
    let inputLines = input.lines();
    let mut map: Vec<Vec<char>> = inputLines.map(|str| str.chars().collect()).collect();
    let mut xPos = 0;
    let mut countValid = [0, 0, 0, 0, 0];
    let xSlopes = [1, 3, 5, 7, 1];
    let ySlopes = [1, 1, 1, 1, 2];
    let mut yPos = 0;
    for slope in 0..xSlopes.len() {
        xPos = 0;
        yPos = 0;
        while (yPos < map.len()) {
            if map[yPos][xPos] == '#' {
                countValid[slope] += 1;
            }
            xPos += xSlopes[slope];
            yPos += ySlopes[slope];
            xPos %= map[0].len();
        }
    }
    let mut mul = 1usize;
    for x in &countValid {
        mul = mul * x;
    }
    println!("{}", mul);
    // for line in inputLines{
    // map.
    // }
}
