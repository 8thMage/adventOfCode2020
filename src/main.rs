use std::{
    cmp::Ordering,
    collections::{hash_map, HashMap, HashSet, VecDeque},
    hash::Hash,
    iter,
    ops::Neg,
    path,
    str::FromStr,
    thread::current,
    vec,
};

fn main() {
    let input = include_str!("input.txt");
    let mut zero_state = vec![];
    let mut width = 0;
    let mut height = 0;
    for (y, line) in input
        .lines()
        .skip(1)
        .take_while(|s| !s.contains("##"))
        .enumerate()
    {
        for (x, c) in line.chars().enumerate() {
            match c {
                '>' | '<' | 'v' | '^' => {
                    zero_state.push((y, x - 1, c));
                }
                _ => {}
            }
        }
        width = line.len() - 2;
        height += 1;
    }

    let mut min = find_min(&zero_state, 0, 0, width, height, &mut usize::MAX, 1);
    min = find_min_rev(
        &zero_state,
        height - 1,
        width - 1,
        width,
        height,
        &mut usize::MAX,
        min + 2,
    );
    min = find_min(&zero_state, 0, 0, width, height, &mut usize::MAX, min + 2);
    println!("{}", min + 1);
}

#[derive(PartialEq, Eq)]
struct State {
    y: usize,
    x: usize,
    width: usize,
    height: usize,
    time: usize,
}

impl PartialOrd for State {
    fn ge(&self, other: &Self) -> bool {
        (self.y + self.x) as isize >= (other.y + other.x) as isize
    }
    fn lt(&self, other: &Self) -> bool {
        ((self.y + self.x) as isize) < (other.y + other.x) as isize
    }
    fn gt(&self, other: &Self) -> bool {
        (self.y + self.x) as isize > (other.y + other.x) as isize
    }
    fn le(&self, other: &Self) -> bool {
        (self.y + self.x) as isize <= (other.y + other.x) as isize
    }
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        ((self.y + self.x) as isize).partial_cmp(&((other.y + other.x) as isize))
    }
}
impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(other).unwrap()
    }
}

#[derive(PartialEq, Eq)]
struct RevState {
    y: usize,
    x: usize,
    width: usize,
    height: usize,
    time: usize,
}

impl PartialOrd for RevState {
    fn ge(&self, other: &Self) -> bool {
        ((self.y + self.x) as isize).neg() >= ((other.y + other.x) as isize).neg()
    }
    fn lt(&self, other: &Self) -> bool {
        (((self.y + self.x) as isize).neg()) < ((other.y + other.x) as isize).neg()
    }
    fn gt(&self, other: &Self) -> bool {
        ((self.y + self.x) as isize).neg() > ((other.y + other.x) as isize).neg()
    }
    fn le(&self, other: &Self) -> bool {
        ((self.y + self.x) as isize).neg() <= ((other.y + other.x) as isize).neg()
    }
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        (((self.y + self.x) as isize).neg()).partial_cmp(&(((other.y + other.x) as isize).neg()))
    }
}
impl Ord for RevState {
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(other).unwrap()
    }
}

fn find_min(
    zero_state: &Vec<(usize, usize, char)>,
    y: usize,
    x: usize,
    width: usize,
    height: usize,
    max_min: &mut usize,
    time: usize,
) -> usize {
    let mut min_heap = std::collections::BinaryHeap::new();
    min_heap.push(State {
        x,
        y,
        time: time,
        height,
        width,
    });
    let mut checked = std::collections::HashSet::new();

    while let Some(State { x, y, time, .. }) = min_heap.pop() {
        if time + height - y + width - x >= *max_min {
            continue;
        }
        if y == height - 1 && x == width - 1 {
            *max_min = time;
            println!("{}", time);
            continue;
        }
        if !check_exists(zero_state, y, x + 1, width, height, time + 1)
            && !checked.contains(&(y, x + 1, time + 1))
        {
            checked.insert((y, x + 1, time + 1));
            min_heap.push(State {
                x: x + 1,
                y,
                time: time + 1,
                height,
                width,
            });
        }
        if !check_exists(zero_state, y + 1, x, width, height, time + 1)
            && !checked.contains(&(y + 1, x, time + 1))
        {
            checked.insert((y + 1, x, time + 1));

            min_heap.push(State {
                x,
                y: y + 1,
                time: time + 1,
                height,
                width,
            });
        }
        if !check_exists(zero_state, y, x.wrapping_sub(1), width, height, time + 1)
            && !checked.contains(&(y, x.wrapping_sub(1), time + 1))
        {
            checked.insert((y, x.wrapping_sub(1), time + 1));

            min_heap.push(State {
                x: x.wrapping_sub(1),
                y,
                time: time + 1,
                height,
                width,
            });
        }

        if !check_exists(zero_state, y.wrapping_sub(1), x, width, height, time + 1)
            && !checked.contains(&(y.wrapping_sub(1), x, time + 1))
        {
            checked.insert((y.wrapping_sub(1), x, time + 1));

            min_heap.push(State {
                x,
                y: y.wrapping_sub(1),
                time: time + 1,
                height,
                width,
            });
        }
        if !check_exists(zero_state, y, x, width, height, time + 1)
            && !checked.contains(&(y, x, time + 1))
        {
            checked.insert((y, x, time + 1));
            min_heap.push(State {
                x,
                y,
                time: time + 1,
                height,
                width,
            });
        }
    }
    *max_min
}

fn find_min_rev(
    zero_state: &Vec<(usize, usize, char)>,
    y: usize,
    x: usize,
    width: usize,
    height: usize,
    max_min: &mut usize,
    time: usize,
) -> usize {
    let mut min_heap = std::collections::BinaryHeap::new();
    min_heap.push(RevState {
        x,
        y,
        time: time,
        height,
        width,
    });
    let mut checked = std::collections::HashSet::new();

    while let Some(RevState { x, y, time, .. }) = min_heap.pop() {
        if time + y + x >= *max_min {
            continue;
        }
        if y == 0 && x == 0 {
            *max_min = time;
            println!("{}", time);
            continue;
        }
        if !check_exists(zero_state, y, x + 1, width, height, time + 1)
            && !checked.contains(&(y, x + 1, time + 1))
        {
            checked.insert((y, x + 1, time + 1));
            min_heap.push(RevState {
                x: x + 1,
                y,
                time: time + 1,
                height,
                width,
            });
        }
        if !check_exists(zero_state, y + 1, x, width, height, time + 1)
            && !checked.contains(&(y + 1, x, time + 1))
        {
            checked.insert((y + 1, x, time + 1));

            min_heap.push(RevState {
                x,
                y: y + 1,
                time: time + 1,
                height,
                width,
            });
        }
        if !check_exists(zero_state, y, x.wrapping_sub(1), width, height, time + 1)
            && !checked.contains(&(y, x.wrapping_sub(1), time + 1))
        {
            checked.insert((y, x.wrapping_sub(1), time + 1));

            min_heap.push(RevState {
                x: x.wrapping_sub(1),
                y,
                time: time + 1,
                height,
                width,
            });
        }

        if !check_exists(zero_state, y.wrapping_sub(1), x, width, height, time + 1)
            && !checked.contains(&(y.wrapping_sub(1), x, time + 1))
        {
            checked.insert((y.wrapping_sub(1), x, time + 1));

            min_heap.push(RevState {
                x,
                y: y.wrapping_sub(1),
                time: time + 1,
                height,
                width,
            });
        }
        if !check_exists(zero_state, y, x, width, height, time + 1)
            && !checked.contains(&(y, x, time + 1))
        {
            checked.insert((y, x, time + 1));
            min_heap.push(RevState {
                x,
                y,
                time: time + 1,
                height,
                width,
            });
        }
    }
    if *max_min == usize::MAX {
        find_min_rev(zero_state, y, x, width, height, max_min, time + 1);
    }
    *max_min
}

fn check_exists(
    zero_state: &Vec<(usize, usize, char)>,
    y: usize,
    x: usize,
    width: usize,
    height: usize,
    time: usize,
) -> bool {
    x == usize::MAX
        || y == usize::MAX
        || x == width
        || y == height
        || zero_state.iter().any(|&(y_s, x_s, c)| match c {
            '>' => y_s == y && x == (x_s + time) % width,
            '<' => y_s == y && x_s == (x + time) % width,
            '^' => x_s == x && y_s == (y + time) % height,
            'v' => x_s == x && y == (y_s + time) % height,
            _ => unreachable!(),
        })
}
