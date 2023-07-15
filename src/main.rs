use std::{
    cmp::Ordering,
    collections::{HashMap, HashSet, VecDeque},
    hash::Hash,
    iter, path,
    str::FromStr,
    thread::current,
    vec,
};

fn main() {
    let input = include_str!("input.txt");
    let mut sum = 1;
    for line in input.lines().take(3) {
        let mut iter = line
            .split(['.', ':'])
            .map(|s| {
                s.split(" ")
                    .filter_map(|s| u32::from_str(s).ok())
                    .collect::<Vec<_>>()
            })
            .filter(|v| v.len() > 0);
        let index = iter.next().unwrap()[0];
        let mut resources_per_robot = iter.collect::<Vec<_>>();
        for res in &mut resources_per_robot {
            if res.len() == 1 {
                res.push(0);
            }
        }
        resources_per_robot[3].insert(1, 0);
        for res in &mut resources_per_robot {
            if res.len() == 2 {
                res.push(0);
            }
        }
        sum *= 
             create_robots(
                &resources_per_robot,
                [1, 0, 0, 0],
                [0; 4],
                false,
                0,
                &mut HashMap::new(),
            );
    }
    println!("{}", sum);
}

#[derive(PartialEq, Eq, PartialOrd)]
struct Entry {
    robots: [u32; 4],
    resources: [u32; 4],
    minute: u32,
    previously_built: bool,
}

impl Ord for Entry {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        ((self.resources[3] + self.robots[3] * self.minute) * 1000
            + (self.resources[2] + self.robots[2] * self.minute))
            .cmp(
                &((other.resources[3] + other.robots[3] * other.minute) * 1000
                    + (other.resources[2] + other.robots[2] * other.minute)),
            )
    }
}

fn create_robots(
    resources_per_robots: &Vec<Vec<u32>>,
    mut robots: [u32; 4],
    mut resources: [u32; 4],
    previously_built: bool,
    minute: u32,
    state_hash: &mut HashMap<Entry, u32>,
) -> u32 {
    let mut heap = std::collections::BinaryHeap::new();
    // robots = [1, 0, 0, 0];
    // resources = [4, 15, 0, 0];
    // let minute = 10;
    heap.push(Entry {
        robots,
        resources,
        previously_built,
        minute,
    });
    let mut max = 0;
    while !heap.is_empty() {
        let Entry {
            mut robots,
            mut resources,
            previously_built,
            minute,
        } = heap.pop().unwrap();
        if minute == 32 {
            max = max.max(*resources.last().unwrap());
            continue;
        }
        let delta_time = 32 - minute;
        if resources[3] + robots[3] * delta_time + (delta_time) * (delta_time - 1) / 2 <= max {
            continue;
        }

        let possible_robots = resources_per_robots
            .iter()
            .enumerate()
            .filter(|(_, v)| {
                v[0] <= resources[0]
                    && v[1] <= resources[1]
                    && v[2] <= resources[2]
                    && (previously_built
                        || (v[0] > (resources[0] - robots[0]) || v[1] > (resources[1] - robots[1]) || v[2] > (resources[2] - robots[2])))
            })
            .map(|(i, _v)| i)
            .collect::<Vec<_>>();

        for i in 0..resources.len() {
            resources[i] += robots[i];
        }
        for &robot in possible_robots.iter().rev() {
            resources[0] -= resources_per_robots[robot][0];
            resources[1] -= resources_per_robots[robot][1];
            resources[2] -= resources_per_robots[robot][2];
            robots[robot] += 1;
            let delta_time = 32 - minute;
            if resources[3] + robots[3] * delta_time + (delta_time) * (delta_time - 1) / 2 <= max {
                robots[robot] -= 1;
                resources[0] += resources_per_robots[robot][0];
                resources[1] += resources_per_robots[robot][1];
                resources[2] += resources_per_robots[robot][2];
                continue;
            }
            heap.push(Entry {
                robots,
                resources,
                previously_built: true,
                minute: minute + 1,
            });
            robots[robot] -= 1;
            resources[0] += resources_per_robots[robot][0];
            resources[1] += resources_per_robots[robot][1];
            resources[2] += resources_per_robots[robot][2];
        }
        heap.push(Entry {
            robots,
            resources,
            previously_built: false,
            minute: minute + 1,
        });
    }
    println!("{}", max);
    max
}
