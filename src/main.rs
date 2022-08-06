// use core::panic;
use std::{
    cell::RefCell,
    collections::{hash_set, HashMap, HashSet},
};
// mod helpers;
// use helpers::*;

#[derive(PartialEq, Eq, Copy, Clone)]
struct Orientation {
    dirs: [usize; 3],
    signs: [i64; 3],
}

#[derive(PartialEq, Eq, Copy, Clone)]
struct OrientationAndPosition {
    orientation: Orientation,
    position: Vector3,
}

impl Orientation {
    fn compose(orientation1: &Orientation, orientation2: &Orientation) -> Orientation {
        let mut new_dirs = [
            orientation2.dirs[orientation1.dirs[0]],
            orientation2.dirs[orientation1.dirs[1]],
            orientation2.dirs[orientation1.dirs[2]],
        ];
        let mut new_signs = [
            orientation2.signs[orientation1.dirs[0]] * orientation1.signs[0],
            orientation2.signs[orientation1.dirs[1]] * orientation1.signs[1],
            orientation2.signs[orientation1.dirs[2]] * orientation1.signs[2],
        ];
        Orientation {
            dirs: new_dirs,
            signs: new_signs,
        }
    }

    fn apply(&self, vec: Vector3) -> Vector3 {
        let new_vec = Vector3 {
            v: [
                vec.v[self.dirs[0]] * self.signs[0],
                vec.v[self.dirs[1]] * self.signs[1],
                vec.v[self.dirs[2]] * self.signs[2],
            ],
        };
        new_vec
    }
}

#[derive(PartialEq, Eq, Copy, Clone, Hash)]
struct Vector3 {
    v: [i64; 3],
}

impl std::ops::Sub for Vector3 {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self::Output {
        Vector3 {
            v: [
                self.v[0] - rhs.v[0],
                self.v[1] - rhs.v[1],
                self.v[2] - rhs.v[2],
            ],
        }
    }
}

impl std::ops::Add for Vector3 {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        Vector3 {
            v: [
                self.v[0] + rhs.v[0],
                self.v[1] + rhs.v[1],
                self.v[2] + rhs.v[2],
            ],
        }
    }
}
impl OrientationAndPosition {
    fn compose(
        orientation1: &OrientationAndPosition,
        orientation2: &OrientationAndPosition,
    ) -> OrientationAndPosition {
        let orientation =
            Orientation::compose(&orientation1.orientation, &orientation2.orientation);
        let position =
            orientation1.position + orientation1.orientation.apply(orientation2.position);
        OrientationAndPosition {
            orientation,
            position,
        }
    }
}

fn get_possible_orientation() -> Vec<Orientation> {
    let mut acc = vec![];
    for front_plus_sign in 0..6 {
        let front_sign = if front_plus_sign / 3 == 0 { 1 } else { -1 };
        let front = front_plus_sign % 3;
        for rot in 0..4 {
            let (left, up, left_sign, up_sign) = match rot {
                0 => ((front + 1) % 3, (front + 2) % 3, front_sign, 1),
                1 => ((front + 2) % 3, (front + 1) % 3, front_sign, -1),
                2 => ((front + 1) % 3, (front + 2) % 3, -front_sign, -1),
                3 => ((front + 2) % 3, (front + 1) % 3, -front_sign, 1),
                _ => panic!(),
            };
            acc.push(Orientation {
                dirs: [front, left, up],
                signs: [front_sign, left_sign, up_sign],
            })
        }
    }
    acc
}

fn main() {
    let input = include_str!("input.txt");
    let mut scanners = vec![];
    for line in input.lines() {
        if line.contains("---") {
            scanners.push(vec![]);
        } else if line != "" {
            let mut l = line.split(",").map(|v| i64::from_str_radix(v, 10).unwrap());
            let p = Vector3 {
                v: [l.next().unwrap(), l.next().unwrap(), l.next().unwrap()],
            };
            scanners.last_mut().unwrap().push(p);
        }
    }
    let orientations = get_possible_orientation();
    let mut scanner_orientation_and_positions = HashMap::new();
    for (index, scanner) in scanners.iter().enumerate() {
        'big_loop: for (index2, second_scanner) in scanners.iter().enumerate() {
            if index2 == index {
                continue;
            }
            if index > index2 && !scanner_orientation_and_positions.contains_key(&(index2, index)) {
                continue;
            }
            println!("{}", index2);
            for orientation in &orientations {
                let mut deltas = HashMap::new();
                for beacon in scanner {
                    if beacon.v[0] == -618 {
                        let x =0;
                    }
                    for first_beacon_second_scanner in second_scanner {
                        let delta = *beacon - orientation.apply(*first_beacon_second_scanner);
                        if delta.v[0] == -68 {
                            let x =0;
                        }
                        if deltas.contains_key(&(delta)) {
                            *deltas
                                .get_mut(&(delta))
                                .unwrap() += 1;
                            if *deltas
                                .get_mut(&(delta))
                                .unwrap()
                                >= 12
                            {
                                scanner_orientation_and_positions.insert(
                                    (index, index2),
                                    OrientationAndPosition {
                                        orientation: *orientation,
                                        position: delta,
                                    },
                                );                            
                                continue 'big_loop;
                            }
                        } else {
                            deltas.insert(delta, 1);
                        }
                    }
                }
            }
        }
    }
    loop {
        let mut run_again = false;
        for i in 1..scanners.len() {
            for j in 1..scanners.len() {
                if scanner_orientation_and_positions.contains_key(&(0, i))
                    && scanner_orientation_and_positions.contains_key(&(i, j))
                    && !scanner_orientation_and_positions.contains_key(&(0, j))
                {
                    run_again = true;
                    scanner_orientation_and_positions.insert(
                        (0, j),
                        OrientationAndPosition::compose(
                            &scanner_orientation_and_positions[&(0, i)],
                            &scanner_orientation_and_positions[&(i, j)],
                        ),
                    );
                }
            }
        }
        if !run_again {
            break;
        }
    }
    let mut beacons = HashSet::new();
    for beacon in &scanners[0] {
        beacons.insert(*beacon);
    }
    for i in 1..scanners.len() {
        let x = 0;
        for beacon in &scanners[i] {
            let vec = scanner_orientation_and_positions[&(0, i)].position
                + scanner_orientation_and_positions[&(0, i)]
                    .orientation
                    .apply(*beacon);
            beacons.insert(vec);
        }
    }
    let mut max_dist = 0;
    for i in 1..scanners.len() {
        let dist = scanner_orientation_and_positions[&(0, i)]
            .position
            .v
            .iter()
            .fold(0, |acc, a| acc + a.abs());
        max_dist = max_dist.max(dist);

        for j in 1..scanners.len() {
            let dist = scanner_orientation_and_positions[&(0, i)]
                .position
                .v
                .iter()
                .zip(scanner_orientation_and_positions[&(0, j)].position.v.iter())
                .fold(0, |acc, (a, b)| acc + (a - b).abs());
            max_dist = max_dist.max(dist);
        }
    }
    println!();
    println!("{}", beacons.len());
    println!("{}", max_dist);
    let x = 0;
}
