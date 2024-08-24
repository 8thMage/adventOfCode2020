use core::{f64, num};
use std::{
    cell::RefCell,
    cmp::Reverse,
    collections::{BinaryHeap, HashMap, HashSet, VecDeque},
    fmt::Write,
    hash::Hash,
    i128, i64,
    iter::{repeat, Cycle, FromIterator},
    mem::swap,
    rc::Rc,
    str::FromStr,
    thread::current,
};

#[derive(Clone, Copy, Debug)]
struct Stone {
    x: i64,
    y: i64,
    z: i64,
    vx: i64,
    vy: i64,
    vz: i64,
}

#[derive(Clone, Copy, Debug)]
struct StoneWindow {
    start: [i128; 3],
    size: u64,
}

fn main() {
    let input = include_str!("input.txt");
    let mut hail_stones = input
        .lines()
        .map(|l| {
            let mut iter = l
                .split(' ')
                .filter_map(|s| i64::from_str(s.trim_end_matches([','])).ok());
            Stone {
                x: iter.next().unwrap(),
                y: iter.next().unwrap(),
                z: iter.next().unwrap(),
                vx: iter.next().unwrap(),
                vy: iter.next().unwrap(),
                vz: iter.next().unwrap(),
            }
        })
        .collect::<Vec<_>>();
    let mut count = 0;
    for i in 0..hail_stones.len() {
        for j in i + 1..hail_stones.len() {
            let a = hail_stones[i];
            let b = hail_stones[j];
            let t = ((a.vx * (b.y - a.y) - a.vy * (b.x - a.x)) as f64)
                / ((a.vy * b.vx - a.vx * b.vy) as f64);
            let s = (b.x as f64 - a.x as f64 + b.vx as f64 * t) / a.vx as f64;
            // println!("0,  {} {}", t, s);
            if t < 0. || s < 0. {
                continue;
            }
            let intersection_x = b.x as f64 + t * b.vx as f64;
            if intersection_x < 200000000000000. || intersection_x > 400000000000000. {
                continue;
            }
            let intersection_y = b.y as f64 + t * b.vy as f64;
            if intersection_y < 200000000000000. || intersection_y > 400000000000000. {
                continue;
            }
            count += 1;
        }
    }
    println!("{}", count);
    let min_x = hail_stones.iter().map(|stone| stone.x).min().unwrap();
    let min_y = hail_stones.iter().map(|stone| stone.y).min().unwrap();
    let min_z = hail_stones.iter().map(|stone| stone.z).min().unwrap();
    let stones = hail_stones
        .iter()
        .map(|stone| {
            vec![
                (stone.x - min_x) as i128,
                (stone.y - min_y) as i128,
                (stone.z - min_z) as i128,
                stone.vx as i128,
                stone.vy as i128,
                stone.vz as i128,
            ]
        })
        .collect::<Vec<_>>();
    let start_bits = 1 << 16 as u64;
    let mut current_stage = vec![];
    let mut next_stage = vec![];
    // for dcoords in 0..1 << 3 {
    //     let vx = (dcoords & 1) / 1;
    //     let vy = (dcoords & 2) / 2;
    //     let vz = (dcoords & 4) / 4;

    //     current_stage.push(StoneWindow {
    //         start: [
    //             -vx * start_bits as i128,
    //             -vy * start_bits as i128,
    //             -vz * start_bits as i128,
    //         ],
    //         size: start_bits,
    //     });
    // }
    println!(
        "{}",
        does_have_over_zero(
            &StoneWindow {
                start: [-20480 * 128, 65472 * 128, 2304 * 128],
                size: 64 * 128
            },
            &stones
        )
    );

    current_stage.push(StoneWindow {
        start: [-20480 * 4, 65472 * 4, 2304 * 4],
        size: 64*16,
    });
    current_stage.push(StoneWindow {
        start: [20480 * 4, -65472 * 4, -2304 * 4],
        size: 64*16,
    });
    loop {
        for (i, entry) in current_stage.drain(..).enumerate() {
            if i % 1000 == 0 {
                println!("heartbeat {}", i);
            }
            let new_size = if entry.size == 2 {
                1
            } else {
                entry.size as i64 / 2
            };
            if entry.size == 0 {
                // if gcd(
                //     entry.start[0].abs(),
                //     gcd(entry.start[1].abs(), entry.start[2].abs()),
                // ) != 1
                // {
                //     continue;
                // }
                println!("{:?}", entry);
                // println!("{:?}", entry.start.x + entry.start.y + entry.start.z);
                // return;
                continue;
            }
            for x_div in 0..2 {
                let vx = entry.start[0] + x_div * new_size as i128;
                if x_div >= entry.size as i128 {
                    continue;
                }

                for y_div in 0..2 {
                    let vy = entry.start[1] + y_div * new_size as i128;
                    if y_div >= entry.size as i128 {
                        continue;
                    }
                    for z_div in 0..2 {
                        if z_div >= entry.size as i128 {
                            continue;
                        }
                        let vz = entry.start[2] + z_div * new_size as i128;
                        let next_stone_window = StoneWindow {
                            start: [vx, vy, vz],
                            size: new_size as u64,
                        };
                        if does_have_over_zero(&next_stone_window, &stones) {
                            next_stage.push(next_stone_window);
                        }
                    }
                }
            }
        }

        swap(&mut current_stage, &mut next_stage);
        if current_stage[0].size == 4 {
            for i in &current_stage {
                println!("{:?}, {}, {}", i.start[0], i.start[1], i.start[2]);
            }
        }
        println!("{:?} {}", current_stage[0].size, current_stage.len());
    }

    // for i in 0..hail_stones.len() {
    //     for j in i + 1..hail_stones.len() {
    //         let delta_x = stones[j][0] - stones[i][0];
    //         let delta_y = stones[j][1] - stones[i][1];
    //         let delta_z = stones[j][2] - stones[i][2];

    //         let t = println!(
    //             "1 {} {} {}",
    //             triple_product(&[-3, 1, 2], &stones[i][3..6], &[delta_x, delta_y, delta_z]) as f64,
    //             triple_product(&[-3, 1, 2], &stones[i][3..6], &[delta_x, delta_y, delta_z]) as f64
    //                 / triple_product(&[0, 0, 1], &stones[j][3..6], &stones[i][3..6]) as f64,
    //             triple_product(
    //                 &[-3, 1, 2],
    //                 &stones[j][3..6],
    //                 &[-delta_x, -delta_y, -delta_z]
    //             ) as f64
    //                 / triple_product(&[0, 0, 1], &stones[i][3..6], &stones[j][3..6]) as f64
    //         );
    //     }
    // }
}

fn does_have_over_zero(stone_window: &StoneWindow, hail_stones: &[Vec<i128>]) -> bool {
    if stone_window.start == [0, 0, 0] && stone_window.size == 0 {
        return false;
    }
    let mut min_x = f64::MIN;
    let mut max_x = f64::MAX;
    let mut min_y = f64::MIN;
    let mut max_y = f64::MAX;
    for i in 0..hail_stones.len() {
        for j in i + 1..hail_stones.len() {
            let mut min_0_0 = i128::MAX;
            let mut max_0_0 = i128::MIN;
            let mut min_0_1 = i128::MAX;
            let mut max_0_1 = i128::MIN;

            let mut min_1_0 = i128::MAX;
            let mut max_1_0 = i128::MIN;
            let mut min_1_1 = i128::MAX;
            let mut max_1_1 = i128::MIN;

            let delta_x = hail_stones[j][0] - hail_stones[i][0];
            let delta_y = hail_stones[j][1] - hail_stones[i][1];
            let delta_z = hail_stones[j][2] - hail_stones[i][2];
            for dcoords in 0..(1 << 3).min(1 + stone_window.size as i128 * 8) {
                let mut v = [
                    stone_window.start[0] + (dcoords & 1) * stone_window.size as i128,
                    stone_window.start[1] + ((dcoords & 2) / 2) * stone_window.size as i128,
                    stone_window.start[2] + ((dcoords & 4) / 4) * stone_window.size as i128,
                ];
                let triple_0 =
                    triple_product(&v, &hail_stones[i][3..6], &[delta_x, delta_y, delta_z]);
                max_0_0 = max_0_0.max(triple_0);
                min_0_0 = min_0_0.min(triple_0);
                let triple_1 = triple_product(&v, &hail_stones[j][3..6], &hail_stones[i][3..6]);
                max_0_1 = max_0_1.max(triple_1);
                min_0_1 = min_0_1.min(triple_1);

                let triple_2 =
                    triple_product(&v, &hail_stones[j][3..6], &[-delta_x, -delta_y, -delta_z]);
                max_1_0 = max_1_0.max(triple_2);
                min_1_0 = min_1_0.min(triple_2);
                max_1_1 = max_1_1.max(-triple_1);
                min_1_1 = min_1_1.min(-triple_1);
            }
            let is_negative = (min_0_0 > 0 && max_0_1 < 0)
                || (min_0_1 > 0 && max_0_0 < 0)
                || (min_1_0 > 0 && max_1_1 < 0)
                || (min_1_1 > 0 && max_1_0 < 0)
                || ((min_0_0 == 0 && max_0_0 == 0) && (min_0_1 > 0 || max_0_1 < 0))
                || ((min_1_0 == 0 && max_1_0 == 0) && (min_1_1 > 0 || max_1_1 < 0));
            if is_negative {
                return false;
            }
            if min_0_1 <= 0 && max_0_1 >= 0 {
                continue;
            }
            let a = (max_0_0 as f64 / min_0_1 as f64).max(0.0);
            let b = (min_0_0 as f64 / max_0_1 as f64).max(0.0);
            let max_t = a.max(b);
            let min_t = a.min(b);
            let max_pos_0 = [
                hail_stones[j][0] as f64 + hail_stones[j][3] as f64 * max_t,
                hail_stones[j][1] as f64 + hail_stones[j][4] as f64 * max_t,
                hail_stones[j][2] as f64 + hail_stones[j][5] as f64 * max_t,
            ];
            let min_pos_0 = [
                hail_stones[j][0] as f64 + hail_stones[j][3] as f64 * min_t,
                hail_stones[j][1] as f64 + hail_stones[j][4] as f64 * min_t,
                hail_stones[j][2] as f64 + hail_stones[j][5] as f64 * min_t,
            ];

            let a = (max_1_0 as f64 / min_1_1 as f64).max(0.0);
            let b = (min_1_0 as f64 / max_1_1 as f64).max(0.0);
            let max_t = a.max(b);
            let min_t = a.min(b);
            let max_pos_1 = [
                hail_stones[i][0] as f64 + hail_stones[i][3] as f64 * max_t,
                hail_stones[i][1] as f64 + hail_stones[i][4] as f64 * max_t,
                hail_stones[i][2] as f64 + hail_stones[i][5] as f64 * max_t,
            ];
            let min_pos_1 = [
                hail_stones[i][0] as f64 + hail_stones[i][3] as f64 * min_t,
                hail_stones[i][1] as f64 + hail_stones[i][4] as f64 * min_t,
                hail_stones[i][2] as f64 + hail_stones[i][5] as f64 * min_t,
            ];
            let mut current_max_0_x = f64::MIN;
            let mut current_min_0_x = f64::MAX;
            let mut current_max_0_y = f64::MIN;
            let mut current_min_0_y = f64::MAX;

            let mut current_max_1_x = f64::MIN;
            let mut current_min_1_x = f64::MAX;
            let mut current_max_1_y = f64::MIN;
            let mut current_min_1_y = f64::MAX;
            for dcoords in 0..(1 << 3).min(1 + stone_window.size as i128 * 8) {
                let v = [
                    stone_window.start[0] as f64
                        + ((dcoords & 1) * stone_window.size as i128) as f64,
                    stone_window.start[1] as f64
                        + (((dcoords & 2) / 2) * stone_window.size as i128) as f64,
                    stone_window.start[2] as f64
                        + (((dcoords & 4) / 4) * stone_window.size as i128) as f64,
                ];
                let cross_x = cross_product_x(&v, &max_pos_0);
                current_max_0_x = current_max_0_x.max(cross_x);
                current_min_0_x = current_min_0_x.min(cross_x);
                let cross_x = cross_product_x(&v, &min_pos_0);
                current_max_0_x = current_max_0_x.max(cross_x);
                current_min_0_x = current_min_0_x.min(cross_x);

                let cross_y = cross_product_y(&v, &max_pos_0);
                current_max_0_y = current_max_0_y.max(cross_y);
                current_min_0_y = current_min_0_y.min(cross_y);
                let cross_y = cross_product_y(&v, &min_pos_0);
                current_max_0_y = current_max_0_y.max(cross_y);
                current_min_0_y = current_min_0_y.min(cross_y);

                let cross_x = cross_product_x(&v, &max_pos_1);
                current_max_1_x = current_max_1_x.max(cross_x);
                current_min_1_x = current_min_1_x.min(cross_x);
                let cross_x = cross_product_x(&v, &min_pos_1);
                current_max_1_x = current_max_1_x.max(cross_x);
                current_min_1_x = current_min_1_x.min(cross_x);

                let cross_y = cross_product_y(&v, &max_pos_1);
                current_max_1_y = current_max_1_y.max(cross_y);
                current_min_1_y = current_min_1_y.min(cross_y);
                let cross_y = cross_product_y(&v, &min_pos_1);
                current_max_1_y = current_max_1_y.max(cross_y);
                current_min_1_y = current_min_1_y.min(cross_y);
            }
            if current_max_0_x < min_x
                || current_min_0_x > max_x
                || current_max_1_x < min_x
                || current_min_1_x > max_x
            {
                return false;
            }
            max_x = max_x.min(current_max_0_x).min(current_max_1_x);
            min_x = min_x.max(current_min_0_x).max(current_min_1_x);

            if current_max_0_y < min_y
                || current_min_0_y > max_y
                || current_max_1_y < min_y
                || current_min_1_y > max_y
            {
                return false;
            }
            max_y = max_y.min(current_max_0_y).min(current_max_1_y);
            min_y = min_y.max(current_min_0_y).max(current_min_1_y);
        }
    }
    return true;
}

fn triple_product(a: &[i128], b: &[i128], c: &[i128]) -> i128 {
    let mut sum = 0;
    for i in 0..3 {
        sum += a[i] * b[(i + 1) % 3] * c[(i + 2) % 3];
    }
    for i in 0..3 {
        sum -= a[i] * b[(i + 2) % 3] * c[(i + 1) % 3];
    }
    sum as i128
}

fn cross_product_x(a: &[f64], b: &[f64]) -> f64 {
    a[1] * b[2] - a[2] * b[1]
}

fn cross_product_y(a: &[f64], b: &[f64]) -> f64 {
    a[2] * b[0] - a[0] * b[2]
}

fn gcd(a: i128, b: i128) -> i128 {
    if b == 0 {
        return a;
    }
    if a < b {
        return gcd(b, a);
    }
    return gcd(b, a.rem_euclid(b));
}
