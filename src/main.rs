use std::collections::HashMap;
fn main() {
    let input = include_str!("input.txt");
//     let input = 
// "L.LL.LL.LL
// LLLLLLL.LL
// L.L.L..L..
// LLLL.LL.LL
// L.LL.LL.LL
// L.LLLLL.LL
// ..L.L.....
// LLLLLLLLLL
// L.LLLLLL.L
// L.LLLLL.LL";
    let mut map: Vec<Vec<(bool, bool)>> = input
        .lines()
        .map(|s| {
            s.chars()
                .map(|c| match c {
                    'L' => (true, false),
                    '#' => (true, true),
                    '.' => (false, false),
                    _ => (false, false),
                })
                .collect()
        })
        .collect();
    let mut newMap: Vec<Vec<(bool, bool)>> = Vec::new();
    newMap.resize(map.len(), {
        let mut v = Vec::new();
        v.resize(map[0].len(), (false, false));
        v
    });
    let tmp = map.to_vec();
    let mut stable = false;
    while !stable {
        stable = true;
        // println!("");
        for y in 0..map.len() {
            for x in 0..map[0].len() {
                // print!(
                //     "{}",
                //     match map[y][x] {
                //         (false, false) => '.',
                //         (true, false) => 'L',
                //         (true, true) => '#',
                //         (false, true) => panic!(),
                //     }
                // );
                if map[y][x].0 {
                    let mut count = 0;
                    for dirY in -1i32..2 {
                        for dirX in -1..2 {
                            if dirY != 0 || dirX != 0 {
                                let mut newY = y as i32 + dirY;
                                let mut newX = x as i32 + dirX;

                                while (newX >= 0
                                    && newY >= 0
                                    && newY < map.len() as i32
                                    && newX < map[0].len() as i32
                                    && !map[newY as usize][newX as usize].0)
                                {
                                    newX += dirX;
                                    newY += dirY;
                                }
                                if (newX >= 0
                                    && newY >= 0
                                    && newY < map.len() as i32
                                    && newX < map[0].len() as i32)
                                {
                                    count += map[newY as usize][newX as usize].1 as i32;
                                }
                            }
                        }
                    }
                    // let (dirX,dirY) = (0,0);

                    // for newY in y.saturating_sub(1)..(y + 2).min(map.len()) {
                    //     for newX in x.saturating_sub(1)..(x + 2).min(map[0].len()) {
                    //         if newX != x || newY != y {
                    //             count += map[newY][newX].1 as i32;
                    //         }
                    //     }
                    // }
                    if (!map[y][x].1 && count == 0) || (map[y][x].1 && count < 5) {
                        newMap[y][x] = (true, true);
                    } else {
                        newMap[y][x] = (true, false);
                    }
                }
                stable = stable & (newMap[y][x].1 == map[y][x].1);
            }
            // println!("");
        }
        let tmp = map.to_vec();
        map = newMap.to_vec();
    }
    let res = map
        .iter()
        .map(|v| v.iter().filter(|(a, b)| *b).count())
        .sum::<usize>();
    println!("{}", res);
}
