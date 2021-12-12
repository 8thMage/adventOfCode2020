use core::panic;
use std::collections::*;
mod helpers;
use helpers::*;
fn main() {
    let input = include_str!("input.txt");
    let mut lines: Vec<Vec<i64>> = input
        .lines()
        .map(|s| {
            s.split(" -> ")
                .flat_map(|s| s.split(",").map(|s| s.parse::<i64>().unwrap()))
                .collect()
        })
        .collect();
    // lines = lines
    //     .into_iter()
    //     .filter(|v| v[0] == v[2] || v[1] == v[3])
    //     .collect();

    for line in &lines {
        println!("{} {},{} {}", line[0], line[1], line[2], line[3]);
    }
    let l = lines.len();
    let mut map = vec![];
    for i in 0..1000 {
        map.push(vec![]);
        for j in 0..1000 {
            map[i].push(0);
        }
    }
    let mut s0 = std::collections::hash_set::HashSet::new();
    let mut s1 = std::collections::hash_set::HashSet::new();

    for i in 0..l {
        println!("{}", i);
        let mut b = &lines[i];
        if (b[0] != b[2]) {
            let m0 = i64::min(b[0], b[2]);
            let m1 = i64::max(b[0], b[2]);
            for i in m0..m1 + 1 {
                let x = i;
                let y = (i - b[0]) * (b[3] - b[1]) / (b[2] - b[0]) + b[1];
                map[x as usize][y as usize] += 1;
                if (map[x as usize][y as usize] > 1) {
                    s0.insert((x, y));
                }
            }
        } else {
            let m0 = i64::min(b[1], b[3]);
            let m1 = i64::max(b[1], b[3]);
            for i in m0..m1 + 1 {
                let x = b[0];
                let y = i;
                map[x as usize][y as usize] += 1;
                if (map[x as usize][y as usize] > 1) {
                    s0.insert((x, y));
                }
            }
        }
        for j in i + 1..l {
            let mut a = &lines[j];
            if (same_slope(a, b)) {
                if ((a[0] - b[0]) * (b[1] - b[3]) - (a[1] - b[1]) * (b[0] - b[2]) == 0) {
                    if (b[0] != b[2]) {
                        let m0 = i64::max(i64::min(b[0], b[2]), i64::min(a[0], a[2]));
                        let m1 = i64::min(i64::max(b[0], b[2]), i64::max(a[0], a[2]));
                        for i in m0..m1 + 1 {
                            let x = i;
                            let y = (i - b[0]) * (b[3] - b[1]) / (b[2] - b[0]) + b[1];
                            s1.insert((x, y));
                        }
                    } else {
                        let m0 = i64::max(i64::min(b[1], b[3]), i64::min(a[1], a[3]));
                        let m1 = i64::min(i64::max(b[1], b[3]), i64::max(a[1], a[3]));
                        for i in m0..m1 + 1 {
                            s1.insert((b[0], i));
                        }
                    }
                }
                continue;
            }
            let ma_x = a[2] - a[0];
            let mb_x = b[2] - b[0];
            let ma_y = a[3] - a[1];
            let mb_y = b[3] - b[1];
            let t = ((mb_x * (a[1] - b[1]) - mb_y * (a[0] - b[0])) as f64)
                / ((ma_x * mb_y - mb_x * ma_y) as f64);
            let t2 = ((ma_x * (b[1] - a[1]) - ma_y * (b[0] - a[0])) as f64)
                / ((mb_x * ma_y - ma_x * mb_y) as f64);

            if (t >= 0. && t2 >= 0. && t <= 1. && t2 <= 1.) {
                // println!(
                //     "{} {}",
                //     a[0] + ma_x * (mb_x * (a[1] - b[1]) - mb_y * (a[0] - b[0]))
                //         / (ma_x * mb_y - mb_x * ma_y),
                //     a[1] + ma_y * (mb_x * (a[1] - b[1]) - mb_y * (a[0] - b[0]))
                //         / (ma_x * mb_y - mb_x * ma_y)
                // );
                // println!("{} {}", (t * ma_x as f64), (t * ma_y as f64));
                let p_x = a[0] as f64 + t * ma_x as f64;
                let p_y = a[0] as f64 + t * ma_y as f64;
                if ((p_x.round() - p_x).abs() < 0.1 && (p_y.round() - p_y).abs() < 0.1) {
                    s1.insert((
                        a[0] + ma_x * (mb_x * (a[1] - b[1]) - mb_y * (a[0] - b[0]))
                            / (ma_x * mb_y - mb_x * ma_y),
                        a[1] + ma_y * (mb_x * (a[1] - b[1]) - mb_y * (a[0] - b[0]))
                            / (ma_x * mb_y - mb_x * ma_y),
                    ));
                }
            }
            if (s1 != s0) {
                // for s in &s1 {
                //     print!("({}, {}), ", s.0, s.1)
                // }
                // println!("");
                // for s in &s0 {
                //     print!("({}, {}), ", s.0, s.1)
                // }
                // println!("");
                // println!("{} {}", i, j);
            }
        }
    }
    println!("{}", s0.len());
    println!("{}", s1.len());
}

fn same_slope(a: &Vec<i64>, b: &Vec<i64>) -> bool {
    return (a[0] - a[2]) * (b[1] - b[3]) - (b[0] - b[2]) * (a[1] - a[3]) == 0;
}
