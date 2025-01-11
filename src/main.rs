fn main() {
    let input = include_str!("input.txt");
    let map = input
        .lines()
        .map(|s| s.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();
    let mut count = 0;
    for line in &map {
        for x in 0..line.len() - 3 {
            if line[x] == 'X' && line[x + 1] == 'M' && line[x + 2] == 'A' && line[x + 3] == 'S' {
                count += 1
            }
            if line[x] == 'S' && line[x + 1] == 'A' && line[x + 2] == 'M' && line[x + 3] == 'X' {
                count += 1;
            }
        }
    }
    for y in 0..map.len() - 3 {
        for x in 0..map.len() {
            if map[y][x] == 'X'
                && map[y + 1][x] == 'M'
                && map[y + 2][x] == 'A'
                && map[y + 3][x] == 'S'
            {
                count += 1;
            }
            if map[y + 3][x] == 'X'
                && map[y + 2][x] == 'M'
                && map[y + 1][x] == 'A'
                && map[y][x] == 'S'
            {
                count += 1;
            }
        }
    }
    for y in 0..map.len() - 3 {
        for x in 0..map.len() - 3 {
            if map[y][x] == 'X'
                && map[y + 1][x + 1] == 'M'
                && map[y + 2][x + 2] == 'A'
                && map[y + 3][x + 3] == 'S'
            {
                count += 1;
            }
            if map[y + 3][x + 3] == 'X'
                && map[y + 2][x + 2] == 'M'
                && map[y + 1][x + 1] == 'A'
                && map[y][x] == 'S'
            {
                count += 1;
            }
        }
    }
    for y in 0..map.len() - 3 {
        for x in 0..map.len() - 3 {
            if map[y][x + 3] == 'X'
                && map[y + 1][x + 2] == 'M'
                && map[y + 2][x + 1] == 'A'
                && map[y + 3][x] == 'S'
            {
                count += 1;
            }
            if map[y + 3][x] == 'X'
                && map[y + 2][x + 1] == 'M'
                && map[y + 1][x + 2] == 'A'
                && map[y][x + 3] == 'S'
            {
                count += 1;
            }
        }
    }
    let mut count2 = 0;
    for y in 0..map.len() - 2 {
        for x in 0..map.len() - 2 {
            let mut forward = false;
            let mut backward = false;
            if map[y][x + 2] == 'M' && map[y + 1][x + 1] == 'A' && map[y + 2][x] == 'S' {
                backward = true;
            }
            if map[y + 2][x] == 'M' && map[y + 1][x + 1] == 'A' && map[y][x + 2] == 'S' {
                backward = true;
            }
            if map[y + 2][x + 2] == 'M' && map[y + 1][x + 1] == 'A' && map[y][x] == 'S' {
                forward = true;
            }
            if map[y][x] == 'M' && map[y + 1][x + 1] == 'A' && map[y + 2][x + 2] == 'S' {
                forward = true;
            }
            if forward && backward {
                count2 += 1;
            }
        }
    }

    println!("{}", count);
    println!("{}", count2);
}
