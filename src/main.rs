use core::num;
use std::{
    cmp::Reverse,
    collections::{BinaryHeap, HashMap, HashSet, VecDeque},
    fmt::Write,
    hash::Hash,
    iter::{Cycle, FromIterator},
    str::FromStr,
};

#[derive(PartialEq)]
enum Type {
    Broadcast,
    FlipFlop,
    Conjuction,
}

fn main() {
    let input = include_str!("input.txt");
    let map = input
        .lines()
        .map(|l| l.chars().collect())
        .collect::<Vec<Vec<_>>>();
    let s_pos = map
        .iter()
        .enumerate()
        .find_map(|(y, v)| {
            v.iter()
                .enumerate()
                .find(|(x, c)| **c == 'S')
                .map(|(x, c)| (y, x))
        })
        .unwrap();
    let mut plots_from = HashMap::new();
    let mut dests = HashSet::new();

    combination(&map, &mut plots_from, &mut dests, s_pos, 64);
    println!("{}", dests.len());
    let mut plots_from = HashMap::new();
    let mut dests: HashSet<_> = HashSet::new();

    for steps in (50..400).step_by(1) {
        combination_2(
            &map,
            &mut plots_from,
            &mut dests,
            (s_pos.0 as i64, s_pos.1 as i64),
            steps,
        );

        let sum = plots_from
            .get(&((s_pos.0 as i64, s_pos.1 as i64), steps))
            .unwrap()
            .iter()
            .map(|(_, v)| v.len())
            .sum::<usize>();

        println!("{} {}", steps, sum);
    }

    // for entry in result.iter() {
    //     println!("{:?}", entry);
    // }
    // let steps = 100;
    // let mut plots_from = HashMap::new();

    // combination_3(
    //     &map,
    //     &mut plots_from,
    //     (s_pos.0 as i64, s_pos.1 as i64),
    //     steps,
    //     &mut 0,
    // );
    // // println!("{}", dests.len());
    // let mut sum = 0;
    // let result = plots_from
    //     .get(&(s_pos.0 as i64, s_pos.1 as i64, steps))
    //     .unwrap();
    // sum += plots_from
    //     .get(&(s_pos.0 as i64, s_pos.1 as i64, steps))
    //     .unwrap()
    //     .iter()
    //     .map(|(_, v)| {
    //         v.x_min_max
    //             .iter()
    //             .map(|(_, (a, b))| (b - a + 1 + map[0].len() as i64 % 2 ) / (1+map[0].len() as i64 % 2))
    //             .sum::<i64>()
    //     })
    //     .sum::<i64>();

    // let mut plots_from = HashMap::new();
    // let mut dests2 = HashSet::new();

    // combination_3(
    //     &map,
    //     &mut plots_from,
    //     &mut dests2,
    //     (s_pos.0 as i64, s_pos.1 as i64),
    //     10,
    // );
    // println!("{}", sum);
}

fn combination(
    map: &Vec<Vec<char>>,
    plots_from: &mut HashMap<((usize, usize), u64), u64>,
    dests: &mut HashSet<(usize, usize)>,
    pos: (usize, usize),
    steps: u64,
) {
    if steps == 0 {
        dests.insert(pos);
        return;
    }
    if let Some(plots) = plots_from.get(&(pos, steps)) {
        return;
    }
    for delta in [(0, 1), (0, -1i32), (1, 0), (-1i32, 0)] {
        let next_y = pos.0.wrapping_add(delta.0 as usize);
        let next_x = pos.1.wrapping_add(delta.1 as usize);
        if next_x >= map[0].len() || next_y >= map.len() {
            continue;
        }
        if map[next_y][next_x] == '#' {
            continue;
        }
        combination(map, plots_from, dests, (next_y, next_x), steps - 1);
    }
    plots_from.insert((pos, steps), 0);
}

fn combination_2(
    map: &Vec<Vec<char>>,
    plots_from: &mut HashMap<((i64, i64), u64), HashMap<(i64, i64), HashSet<(i64, i64)>>>,
    dests: &mut HashSet<(i64, i64)>,
    pos: (i64, i64),
    steps: u64,
) {
    let checked_x = pos.1.rem_euclid(map[0].len() as i64);
    let checked_y = pos.0.rem_euclid(map.len() as i64);
    if steps == 0 {
        dests.insert(pos);
        let mut plots = HashMap::new();
        plots.insert((0, 0), HashSet::from_iter([(0, 0)]));
        plots_from.insert(((checked_y, checked_x), steps), plots);

        return;
    }

    if let Some(plots) = plots_from.get(&((checked_y, checked_x), steps)) {
        // for (y, x) in plots {
        //     let next_x = x + pos.1;
        //     let next_y = y + pos.0;
        //     dests.insert((next_y, next_x));
        // }
        return;
    }
    let mut plots = HashMap::new();
    let (step, first_deltas) = (1..steps)
        .rev()
        .filter_map(|i| {
            plots_from
                .get(&((checked_y, checked_x), i))
                .map(|s| (i, s.clone()))
        })
        .next()
        .unwrap_or_else(|| {
            (
                1,
                HashMap::from_iter([
                    ((0, 1), HashSet::from_iter([(0, 0)])),
                    ((0, -1i64), HashSet::from_iter([(0, 0)])),
                    ((1, 0), HashSet::from_iter([(0, 0)])),
                    ((-1i64, 0), HashSet::from_iter([(0, 0)])),
                ]),
            )
        });
    for (delta, v0) in first_deltas {
        let next_y = pos.0 + delta.0;
        let next_x = pos.1 + delta.1;
        let checked_x = next_x.rem_euclid(map[0].len() as i64);
        let checked_y = next_y.rem_euclid(map.len() as i64);
        if map[checked_y as usize][checked_x as usize] == '#' {
            continue;
        }

        combination_2(map, plots_from, dests, (next_y, next_x), steps - step);
        plots_from
            .get(&((checked_y, checked_x), steps - step))
            .unwrap()
            .iter()
            .for_each(|((y, x), v)| {
                let a = y + next_y - pos.0;

                let b = x + next_x - pos.1;
                let m1 = plots
                    .entry((
                        a.rem_euclid(map.len() as i64),
                        b.rem_euclid(map[0].len() as i64),
                    ))
                    .or_insert(HashSet::new());
                for (my0, mx0) in v0.iter() {
                    for (my, mx) in v.iter() {
                        m1.insert((
                            my0 + my + a.div_euclid(map.len() as i64),
                            mx0 + mx + b.div_euclid(map[0].len() as i64),
                        ));
                    }
                }
            })
    }
    plots_from.insert(((checked_y, checked_x), steps), plots);
}

#[derive(Default, Clone)]
struct Entry {
    x_min_max: HashMap<i64, (i64, i64)>,
}

fn combination_3(
    map: &Vec<Vec<char>>,
    plots_from_to: &mut HashMap<(i64, i64, u64), HashMap<(i64, i64), Entry>>,
    pos: (i64, i64),
    steps: u64,
    calcs: &mut u64,
) {
    // if pos.0 == 0 {
    //     println!("sdf");
    // }
    if plots_from_to.contains_key(&(pos.0, pos.1, steps)) {
        return;
    }
    if steps == 0 {
        plots_from_to.insert(
            (pos.0, pos.1, steps),
            HashMap::from([(
                (pos.0, pos.1),
                Entry {
                    x_min_max: HashMap::from([(0, (0, 0))]),
                },
            )]),
        );
        return;
    }
    *calcs += 1;
    if *calcs % 100 == 0 {
        println!("heartbeat {}", *calcs);
    }
    assert!(pos.0 >= 0 && pos.1 >= 0 && pos.0 < map.len() as i64 && pos.1 < map[0].len() as i64);
    if steps % 2 == 1 {
        let mut p: HashMap<(i64, i64), Entry> = HashMap::new();
        for (dx, dy) in [(0, 1), (0, -1), (1, 0), (-1, 0)] {
            let next_y = pos.0 + dy;
            let next_x = pos.1 + dx;
            let new_checked_x = next_x.rem_euclid(map[0].len() as i64);
            let new_checked_y = next_y.rem_euclid(map.len() as i64);
            if map[new_checked_y as usize][new_checked_x as usize] == '#' {
                continue;
            }
            let mx0 = next_x.div_euclid(map[0].len() as i64);
            let my0 = next_y.div_euclid(map.len() as i64);
            combination_3(
                map,
                plots_from_to,
                (new_checked_y, new_checked_x),
                steps - 1,
                calcs,
            );
            let v = plots_from_to
                .get(&(new_checked_y, new_checked_x, steps - 1))
                .unwrap();
            {
                for ((py, px), z) in v.iter() {
                    // if min_y < 0 {
                    //     println!("asfd");
                    // }
                    let res0 = p.entry((*py, *px)).or_insert_with(|| Entry {
                        x_min_max: HashMap::new(),
                    });
                    for (dy, (min_mx, max_mx)) in z.x_min_max.iter() {
                        let y = dy + my0;
                        let min_x = min_mx + mx0;
                        let max_x = max_mx + mx0;
                        let entry = res0.x_min_max.entry(y).or_insert((i64::MAX, i64::MIN));
                        entry.0 = entry.0.min(min_x);
                        entry.1 = entry.1.max(max_x);
                    }
                }
            }
        }
        plots_from_to.insert((pos.0, pos.1, steps), p);
        return;
    }
    combination_3(map, plots_from_to, pos, steps / 2, calcs);
    let v0 = plots_from_to
        .get(&(pos.0 as i64, pos.1 as i64, steps / 2))
        .unwrap();
    let mut v0 = v0.clone();
    let mut res: HashMap<(i64, i64), Entry> = HashMap::new();
    for (&(ny, nx), p0) in v0.iter() {
        combination_3(map, plots_from_to, (ny as i64, nx as i64), steps / 2, calcs);
        let v = plots_from_to
            .get(&(ny as i64, nx as i64, steps / 2))
            .unwrap();
        for (&(py, px), p) in v.iter() {
            let z = v0.get(&(ny, nx)).unwrap();
            let res0 = res.entry((py, px)).or_insert_with(|| Entry {
                x_min_max: HashMap::new(),
            });
            if py == 0 && px == 10 && steps == 20 {
                println!(":");
            }
            // if z.x_min_max.len() > 1 || p.x_min_max.len() > 1 {
            //     println!(":");
            // }
            for (dy, (min_mx, max_mx)) in z.x_min_max.iter() {
                for (dyp, (min_mxp, max_mxp)) in p.x_min_max.iter() {
                    let y = dy + dyp;
                    let min_x = min_mx + min_mxp;
                    let max_x = max_mx + max_mxp;
                    let entry = res0.x_min_max.entry(y).or_insert((i64::MAX, i64::MIN));
                    entry.0 = entry.0.min(min_x);
                    entry.1 = entry.1.max(max_x);
                }
            }
        }
    }
    plots_from_to.insert((pos.0, pos.1, steps), res);
}
