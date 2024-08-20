use core::{f64, num};
use std::{
    cell::RefCell,
    cmp::Reverse,
    collections::{BinaryHeap, HashMap, HashSet, VecDeque},
    fmt::Write,
    hash::Hash,
    i64,
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
    start: Stone,
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
    let start_bits = 1 << 16 as u64;
    let mut current_stage = vec![];
    let mut next_stage = vec![];
    for dcoords in 0..1 << 6 {
        let x = dcoords & 1;
        let y = (dcoords & 2) / 2;
        let z = (dcoords & 4) / 4;
        let vx = (dcoords & 8) / 8;
        let vy = (dcoords & 16) / 16;
        let vz = (dcoords & 32) / 32;

        current_stage.push(StoneWindow {
            start: Stone {
                x: -x * start_bits as i64,
                y: -y * start_bits as i64,
                z: -z * start_bits as i64,
                vx: -vx * start_bits as i64,
                vy: -vy * start_bits as i64,
                vz: -vz * start_bits as i64,
            },
            size: start_bits,
        });
    }
    loop {
        for entry in &current_stage {
            if (24 >= entry.start.x && 24 < entry.start.x + entry.size as i64)
                && (13 >= entry.start.y && 13 < entry.start.y + entry.size as i64)
                && (10 >= entry.start.z && 10 < entry.start.z + entry.size as i64)
                && (-3 >= entry.start.vx && -3 < entry.start.vx + entry.size as i64)
                && (1 >= entry.start.vy && 1 < entry.start.vy + entry.size as i64)
                && (2 >= entry.start.vz && 2 < entry.start.vz + entry.size as i64)
            {
                println!("{}", entry.size);
                break;
            }
        }
        if current_stage.is_empty() {
            break;
        }
        let mut memo_forbidden_x = HashSet::new();
        let mut memo_forbidden_y = HashSet::new();
        let mut memo_forbidden_z = HashSet::new();
        let mut memo_forbidden_xy = HashSet::new();
        let mut memo_forbidden_yz = HashSet::new();
        let mut memo_forbidden_xz = HashSet::new();
        for (i, entry) in current_stage.drain(..).enumerate() {
            if i % 1000 == 0 {
                println!("heartbeat {}", i);
            }
            if (24 >= entry.start.x && 24 < entry.start.x + entry.size as i64)
                && (13 >= entry.start.y && 13 < entry.start.y + entry.size as i64)
                && (10 >= entry.start.z && 10 < entry.start.z + entry.size as i64)
                && (-3 >= entry.start.vx && -3 < entry.start.vx + entry.size as i64)
                && (1 >= entry.start.vy && 1 < entry.start.vy + entry.size as i64)
                && (2 >= entry.start.vz && 2 < entry.start.vz + entry.size as i64)
            {
                println!("{}", entry.size);
            }
            if (0 >= entry.start.x && 0 < entry.start.x + entry.size as i64)
                && (1 >= entry.start.y && 1 < entry.start.y + entry.size as i64)
                && (3 >= entry.start.z && 3 < entry.start.z + entry.size as i64)
                && (0 >= entry.start.vx && 0 < entry.start.vx + entry.size as i64)
                && (0 >= entry.start.vy && 0 < entry.start.vy + entry.size as i64)
                && (0 >= entry.start.vz && 0 < entry.start.vz + entry.size as i64)
            {
                println!("{}", entry.size);
            }
            let new_size = if entry.size == 2 {
                1
            } else {
                entry.size as i64 / 4
            };
            if entry.size == 0 {
                println!("{:?}", entry);
                println!("{:?}", entry.start.x + entry.start.y + entry.start.z);
                return;
            }

            'x_div: for x_div in 0..16 {
                let dx = x_div & 3;
                let dvx = x_div >> 2;
                if dx >= entry.size as i64 || dvx >= entry.size as i64 {
                    continue;
                }
                let x = entry.start.x + dx * new_size;
                let vx = entry.start.vx + dvx * new_size;

                if memo_forbidden_x.contains(&(x, vx, new_size as u64)) {
                    continue;
                }
                'y_div: for y_div in 0..16 {
                    let dy = y_div & 3;
                    let dvy = y_div >> 2;
                    if dy >= entry.size as i64 || dvy >= entry.size as i64 {
                        continue;
                    }
                    let y = entry.start.y + dy * new_size;
                    let vy = entry.start.vy + dvy * new_size;

                    if memo_forbidden_y.contains(&(y, vy, new_size as u64)) {
                        continue;
                    }
                    if memo_forbidden_xy.contains(&(x, vx, y, vy, new_size as u64)) {
                        continue;
                    }
                    'z_div: for z_div in 0..16 {
                        let dz = z_div & 3;
                        let dvz = z_div >> 2;
                        if dz >= entry.size as i64 || dvz >= entry.size as i64 {
                            continue;
                        }
                        let next_stone_window = StoneWindow {
                            start: Stone {
                                x,
                                y,
                                z: entry.start.z + dz * new_size,
                                vx,
                                vy,
                                vz: entry.start.vz + dvz * new_size,
                            },
                            size: new_size as u64,
                        };

                        if memo_forbidden_z.contains(&(
                            next_stone_window.start.z,
                            next_stone_window.start.vz,
                            next_stone_window.size,
                        )) {
                            continue;
                        }

                        if memo_forbidden_xz.contains(&(
                            next_stone_window.start.x,
                            next_stone_window.start.vx,
                            next_stone_window.start.z,
                            next_stone_window.start.vz,
                            next_stone_window.size,
                        )) {
                            continue;
                        }
                        if memo_forbidden_yz.contains(&(
                            next_stone_window.start.y,
                            next_stone_window.start.vy,
                            next_stone_window.start.z,
                            next_stone_window.start.vz,
                            next_stone_window.size,
                        )) {
                            continue;
                        }
                        for i in 0..hail_stones.len() {
                            if !is_cross_product_passes_zero(next_stone_window, hail_stones[i]) {
                                continue 'z_div;
                            }
                            let min_t_x = min_t_coord(
                                next_stone_window.start.x,
                                next_stone_window.start.vx,
                                new_size,
                                hail_stones[i].x,
                                hail_stones[i].vx,
                            );
                            let max_t_x = max_t_coord(
                                next_stone_window.start.x,
                                next_stone_window.start.vx,
                                new_size,
                                hail_stones[i].x,
                                hail_stones[i].vx,
                            );
                            let min_t_y = min_t_coord(
                                next_stone_window.start.y,
                                next_stone_window.start.vy,
                                new_size,
                                hail_stones[i].y,
                                hail_stones[i].vy,
                            );
                            let max_t_y = max_t_coord(
                                next_stone_window.start.y,
                                next_stone_window.start.vy,
                                new_size,
                                hail_stones[i].y,
                                hail_stones[i].vy,
                            );
                            let min_t_z = min_t_coord(
                                next_stone_window.start.z,
                                next_stone_window.start.vz,
                                new_size,
                                hail_stones[i].z,
                                hail_stones[i].vz,
                            );
                            let max_t_z = max_t_coord(
                                next_stone_window.start.z,
                                next_stone_window.start.vz,
                                new_size,
                                hail_stones[i].z,
                                hail_stones[i].vz,
                            );
                            if max_t_x < 0. {
                                memo_forbidden_x.insert((
                                    next_stone_window.start.x,
                                    next_stone_window.start.vx,
                                    next_stone_window.size,
                                ));
                                continue 'x_div;
                            }

                            if max_t_y < 0. {
                                memo_forbidden_y.insert((
                                    next_stone_window.start.y,
                                    next_stone_window.start.vy,
                                    next_stone_window.size,
                                ));
                                continue 'y_div;
                            }
                            if max_t_z < 0. {
                                memo_forbidden_z.insert((
                                    next_stone_window.start.z,
                                    next_stone_window.start.vz,
                                    next_stone_window.size,
                                ));
                                continue 'z_div;
                            }
                            // if min_t_x > hail_stones.len() as f64 {
                            //     continue 'x_div;
                            // }
                            // if min_t_y > hail_stones.len() as f64 {
                            //     continue 'y_div;
                            // }
                            // if min_t_z > hail_stones.len() as f64 {
                            //     continue 'z_div;
                            // }
                            if max_t_x < min_t_y || max_t_y < min_t_x {
                                memo_forbidden_xy.insert((
                                    next_stone_window.start.x,
                                    next_stone_window.start.vx,
                                    next_stone_window.start.y,
                                    next_stone_window.start.vy,
                                    next_stone_window.size,
                                ));
                                continue 'y_div;
                            }
                            if max_t_x < min_t_z || max_t_z < min_t_x {
                                memo_forbidden_xz.insert((
                                    next_stone_window.start.x,
                                    next_stone_window.start.vx,
                                    next_stone_window.start.z,
                                    next_stone_window.start.vz,
                                    next_stone_window.size,
                                ));
                                continue 'z_div;
                            }
                            if max_t_y < min_t_z || max_t_z < min_t_y {
                                memo_forbidden_yz.insert((
                                    next_stone_window.start.y,
                                    next_stone_window.start.vy,
                                    next_stone_window.start.z,
                                    next_stone_window.start.vz,
                                    next_stone_window.size,
                                ));
                                continue 'z_div;
                            }
                        }
                        next_stage.push(next_stone_window)
                    }
                }
            }
        }

        swap(&mut current_stage, &mut next_stage);
        println!(
            "{:?} {}",
            current_stage[0].size.checked_ilog2(),
            current_stage.len()
        );
    }
}

fn min_t_coord(
    window_coord_p: i64,
    window_coord_a: i64,
    window_size: i64,
    stone_coord_p: i64,
    stone_coord_v: i64,
) -> f64 {
    if stone_coord_v >= window_coord_a && stone_coord_v <= window_coord_a + window_size as i64 {
        return -f64::INFINITY;
    }
    let mut min = f64::INFINITY;
    (0..=1)
        .flat_map(|x| repeat(x).zip(0..=1))
        .for_each(|(dx, dvx)| {
            let p = window_coord_p + dx * window_size as i64;
            let a = window_coord_a + dvx * window_size as i64;
            let t = (stone_coord_p - p) as f64 / (a - stone_coord_v) as f64;
            if t < min {
                min = t;
            }
        });
    return min;
}

fn max_t_coord(
    window_coord_p: i64,
    window_coord_a: i64,
    window_size: i64,
    stone_coord_p: i64,
    stone_coord_v: i64,
) -> f64 {
    if stone_coord_v >= window_coord_a && stone_coord_v <= window_coord_a + window_size as i64 {
        return f64::INFINITY;
    }
    let mut max = -f64::INFINITY;
    (0..=1)
        .flat_map(|x| repeat(x).zip(0..=1))
        .for_each(|(dx, dvx)| {
            let p = window_coord_p + dx * window_size as i64;
            let a = window_coord_a + dvx * window_size as i64;
            let t = (stone_coord_p - p) as f64 / (a - stone_coord_v) as f64;
            if t > max {
                max = t;
            }
        });

    return max;
}

fn is_cross_product_passes_zero(window: StoneWindow, stone: Stone) -> bool {
    let mut min_cross_x = i128::MAX;
    let mut max_cross_x = -i128::MAX;
    let mut min_cross_y = i128::MAX;
    let mut max_cross_y = -i128::MAX;
    let mut min_cross_z = i128::MAX;
    let mut max_cross_z = -i128::MAX;
    for dcoords in 0..1 << 6 {
        let x = window.start.x as i128 + window.size as i128 * (dcoords & 1);
        let y = window.start.y as i128 + window.size as i128 * ((dcoords & 2) / 2);
        let z = window.start.z as i128 + window.size as i128 * ((dcoords & 4) / 4);
        let vx = window.start.vx as i128 + window.size as i128 * ((dcoords & 8) / 8);
        let vy = window.start.vy as i128 + window.size as i128 * ((dcoords & 16) / 16);
        let vz = window.start.vz as i128 + window.size as i128 * ((dcoords & 32) / 32);
        let delta_x = x - stone.x as i128;
        let delta_y = y - stone.y as i128;
        let delta_z = z - stone.z as i128;
        let delta_vx = vx - stone.vx as i128;
        let delta_vy = vy - stone.vy as i128;
        let delta_vz = vz - stone.vz as i128;
        let cross_z = delta_x * delta_vy - delta_y * delta_vx;
        let cross_y = delta_x * delta_vz - delta_z * delta_vx;
        let cross_x = delta_z * delta_vy - delta_y * delta_vz;
        max_cross_x = max_cross_x.max(cross_x);
        max_cross_y = max_cross_y.max(cross_y);
        max_cross_z = max_cross_z.max(cross_z);
        min_cross_x = min_cross_x.min(cross_x);
        min_cross_y = min_cross_y.min(cross_y);
        min_cross_z = min_cross_z.min(cross_z);
    }
    max_cross_x >= 0
        && min_cross_x <= 0
        && max_cross_y >= 0
        && min_cross_y <= 0
        && max_cross_z >= 0
        && min_cross_z <= 0
}

fn find_min_of_cross_product_with_dot_product(
    window: StoneWindow,
    stone: Stone,
    vector_x: i128,
    vector_y: i128,
    vector_z: i128,
) -> (i128, i128, i128) {
    let mut min_value = i128::MAX;
    let mut coord = None;
    for dcoords in 0..1 << 6 {
        let x = window.start.x as i128 + window.size as i128 * (dcoords & 1);
        let y = window.start.y as i128 + window.size as i128 * ((dcoords & 2) / 2);
        let z = window.start.z as i128 + window.size as i128 * ((dcoords & 4) / 4);
        let vx = window.start.vx as i128 + window.size as i128 * ((dcoords & 8) / 8);
        let vy = window.start.vy as i128 + window.size as i128 * ((dcoords & 16) / 16);
        let vz = window.start.vz as i128 + window.size as i128 * ((dcoords & 32) / 32);
        let delta_x = x - stone.x as i128;
        let delta_y = y - stone.y as i128;
        let delta_z = z - stone.z as i128;
        let delta_vx = vx - stone.vx as i128;
        let delta_vy = vy - stone.vy as i128;
        let delta_vz = vz - stone.vz as i128;
        let cross_z = delta_x * delta_vy - delta_y * delta_vx;
        let cross_y = delta_x * delta_vz - delta_z * delta_vx;
        let cross_x = delta_z * delta_vy - delta_y * delta_vz;
        let dot = cross_x * vector_x + cross_y * vector_y + cross_z * vector_z;
        if dot < min_value {
            min_value = dot;
            coord = Some((cross_x, cross_y, cross_z));
        }
    }
    return coord.unwrap();
}

fn is_cross_product_contains_zero(window: StoneWindow, stone: Stone) -> bool {
    let va = find_min_of_cross_product_with_dot_product(window, stone, 1, 0, 0);
    let vb = find_min_of_cross_product_with_dot_product(window, stone, va.0, va.1, va.2);
    if vb.0 * va.0 + vb.1 * vb.1 + vb.2 * vb.2 > 0 {
        return false;
    }
    let vab = (vb.0 - va.0, vb.1 - va.1, vb.2 - va.2);
    let nb = (0, -vab.2, vab.1);
    let vc = find_min_of_cross_product_with_dot_product(window, stone, nb.0, nb.1, nb.2);
    if vc.0 * nb.0 + vc.1 * nb.1 + vc.2 * nb.2 > 0 {
        return false;
    }
    let vca = (vc.0 - va.0, vc.1 - va.1, vc.2 - va.2);
    let nd = (
        vab.1 * vca.2 - vab.2 * vca.1,
        vab.2 * vca.0 - vab.0 * vca.2,
        vab.0 * vca.1 - vab.1 * vca.2,
    );
    let vd = find_min_of_cross_product_with_dot_product(window, stone, nd.0, nd.1, nd.2);
    if vd.0 * nd.0 + vd.1 * nd.1 + vd.2 * nd.2 > 0 {
        return false;
    }
    true
}
