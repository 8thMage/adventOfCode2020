use core::num;
use std::{
    cell::RefCell,
    cmp::Reverse,
    collections::{BinaryHeap, HashMap, HashSet, VecDeque},
    fmt::Write,
    hash::Hash,
    iter::{Cycle, FromIterator},
    mem::swap,
    rc::Rc,
    str::FromStr,
};

#[derive(PartialEq, Eq)]
struct Entry {
    x: i64,
    y: i64,
    vx: i64,
    vy: i64,
    current_len: i64,
    visited: Rc<RefCell<HashSet<(i64, i64)>>>,
}

impl PartialOrd for Entry {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.current_len.partial_cmp(&other.current_len)
    }
}

impl Ord for Entry {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.current_len.cmp(&other.current_len)
    }
}

fn main() {
    let input = include_str!("input.txt");
    let mut map = input
        .lines()
        .map(|l| Vec::from_iter(l.chars()))
        .collect::<Vec<Vec<_>>>();
    let first = map[0].iter().position(|c| *c == '.').unwrap();
    let mut longest_map = HashMap::new();
    let mut front = BinaryHeap::new();
    front.push(Entry {
        x: first as i64,
        y: 0,
        vx: 0,
        vy: 1,
        current_len: 0,
        visited: Rc::new(RefCell::new(HashSet::from([]))),
    });
    let mut memo = HashMap::new();
    let mut try_push = |map: &Vec<Vec<char>>,
                        front: &mut BinaryHeap<Entry>,
                        x: i64,
                        y: i64,
                        vx: i64,
                        vy: i64,
                        current_len: i64,
                        visited: &Rc<RefCell<HashSet<(i64, i64)>>>| {
        let next_x = (x as usize).wrapping_add(vx as usize);
        let next_y = (y as usize).wrapping_add(vy as usize);
        if next_x < map[0].len()
            && next_y < map.len()
            && !visited.borrow().contains(&(y as i64, x as i64))
        {
            let can_push = match map[next_y][next_x] {
                '.' => true,
                '#' => false,
                '>' => true,
                '<' => true,
                '^' => true,
                'v' => true,
                _ => unreachable!(),
            };
            if can_push {
                let (next_x, next_y, next_vx, next_vy, next_len, next_visited) = {
                    let mut next_visited =
                        Rc::new(RefCell::new(visited.borrow_mut().to_owned().clone()));
                    next_visited.borrow_mut().insert((y as i64, x as i64));
                    let (next_x, next_y, next_vx, next_vy, next_len) = run_to_end(
                        map,
                        next_x as i64,
                        next_y as i64,
                        vx,
                        vy,
                        current_len + 1,
                        &mut memo,
                    );

                    (next_x, next_y, next_vx, next_vy, next_len, next_visited)
                };

                front.push(Entry {
                    x: next_x,
                    y: next_y,
                    vx: next_vx,
                    vy: next_vy,
                    current_len: next_len,
                    visited: next_visited,
                })
            }
        }
    };
    let mut current_max = 0;
    while let Some(next) = front.pop() {
        if next.y as usize == map.len() - 1 {
            if next.current_len > current_max {           
                println!("{}", next.current_len);
            }
            current_max = current_max.max(next.current_len);
        }
        // if longest_map
        //     .get(&(next.y, next.x, next.vx, next.vy))
        //     .is_some_and(|c| *c > next.current_len)
        // {
        //     continue;
        // }
        longest_map.insert((next.y, next.x, next.vx, next.vy), next.current_len);
        try_push(
            &map,
            &mut front,
            next.x,
            next.y,
            next.vx,
            next.vy,
            next.current_len,
            &next.visited,
        );
        try_push(
            &map,
            &mut front,
            next.x,
            next.y,
            next.vy,
            next.vx,
            next.current_len,
            &next.visited,
        );
        try_push(
            &map,
            &mut front,
            next.x,
            next.y,
            -next.vy,
            -next.vx,
            next.current_len,
            &next.visited,
        );
    }
    println!("{}", current_max);
}

fn run_to_end(
    map: &Vec<Vec<char>>,
    x: i64,
    y: i64,
    old_vx: i64,
    old_vy: i64,
    current_len: i64,
    memo: &mut HashMap<(i64, i64, i64, i64), (i64, i64, i64, i64, i64)>,
) -> (i64, i64, i64, i64, i64) {
    let mut vx = old_vx;
    let mut vy = old_vy;
    let (mut next_x, mut next_y, mut next_len) = if y == 1 && x == 1 {
        (x, y, current_len)
    } else {
        (x + vx, y + vy, current_len + 1)
    };
    if let Some((nx, ny, vx, vy, len)) = memo.get(&(x, y, vx, vy)) {
        return (*nx, *ny, *vx, *vy, len + current_len);
    }
    let is_junction = |map: &Vec<Vec<char>>, x: i64, y: i64| {
        if map[y as usize][x as usize] != '.' {
            return false;
        }
        let mut count = 0;
        for delta in [(0, 1), (0, -1), (1, 0), (-1, 0)] {
            let ny = (y + delta.0) as usize;
            let nx = (x + delta.1) as usize;
            if nx >= map[0].len() || ny >= map.len() {
                continue;
            }
            if map[ny][nx] != '.' && map[ny][nx] != '#' {
                count += 1;
            }
        }
        return count >= 2;
    };

    loop {
        if next_y as usize == map.len() - 1 || next_y == 0 || is_junction(map, next_x, next_y) {
            memo.insert(
                (x, y, old_vx, old_vy),
                (next_x, next_y, vx, vy, next_len - current_len),
            );
            return (next_x, next_y, vx, vy, next_len);
        }
        if map[(next_y as usize).wrapping_add(vy as usize)]
            [(next_x as usize).wrapping_add(vx as usize)]
            != '#'
        {
            next_x = next_x + vx;
            next_y = next_y + vy;
            next_len += 1;
            continue;
        }
        if map[(next_y as usize).wrapping_add(vx as usize)]
            [(next_x as usize).wrapping_add(vy as usize)]
            != '#'
        {
            swap(&mut vx, &mut vy);
            next_x = next_x + vx;
            next_y = next_y + vy;
            next_len += 1;
            continue;
        }
        if map[(next_y as usize).wrapping_add(-vx as usize)]
            [(next_x as usize).wrapping_add(-vy as usize)]
            != '#'
        {
            vx = -vx;
            vy = -vy;
            swap(&mut vx, &mut vy);
            next_x = next_x + vx;
            next_y = next_y + vy;
            next_len += 1;
            continue;
        }
    }
}
