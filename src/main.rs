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
    ops::ControlFlow,
    path,
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
    let mut connections = vec![];
    let mut unique_nodes = HashMap::new();
    for line in input.lines() {
        let (start, list) = line.split_once(": ").unwrap();
        if !unique_nodes.contains_key(start) {
            unique_nodes.insert(String::from_str(start).unwrap(), unique_nodes.len());
        }
        for entry in list.split(" ") {
            connections.push((start, entry));
            if !unique_nodes.contains_key(entry) {
                unique_nodes.insert(String::from_str(entry).unwrap(), unique_nodes.len());
            }
        }
    }
    let mut adj_matrix = vec![vec![]; unique_nodes.len()];

    for &connection in &connections {
        adj_matrix[*unique_nodes.get(connection.0).unwrap()]
            .push(*unique_nodes.get(connection.1).unwrap());
        adj_matrix[*unique_nodes.get(connection.1).unwrap()]
            .push(*unique_nodes.get(connection.0).unwrap());
    }

    for uniq_0 in 0..unique_nodes.len() {
        for uniq_1 in uniq_0 + 1..unique_nodes.len() {
            let mut forbidden = HashSet::new();
            let first_path = find_path(&HashSet::new(), &adj_matrix, uniq_0, uniq_1).unwrap();
            for path in first_path.windows(2) {
                forbidden.insert((path[0], path[1]));
                forbidden.insert((path[1], path[0]));
            }
            let Some(second_path) = find_path(&forbidden, &adj_matrix, uniq_0, uniq_1) else {
                continue;
            };
            for path in second_path.windows(2) {
                forbidden.insert((path[0], path[1]));
                forbidden.insert((path[1], path[0]));
            }
            let Some(third_path) = find_path(&forbidden, &adj_matrix, uniq_0, uniq_1) else {
                continue;
            };
            for path in third_path.windows(2) {
                forbidden.insert((path[0], path[1]));
                forbidden.insert((path[1], path[0]));
            }
            let fourth_path = find_path(&forbidden, &adj_matrix, uniq_0, uniq_1);
            if fourth_path.is_some() {
                continue;
            }
            let mut edges = vec![];
            for path in [first_path, second_path, third_path] {
                for end in path.windows(2).rev() {
                    let Some(_fourth_path) = find_path(&forbidden, &adj_matrix, uniq_0, end[0])
                    else {
                        continue;
                    };
                    edges.push((end[0], end[1]));
                    break;
                }
            }
            finish_connection(&connections, &edges, &unique_nodes);
            return
        }
    }
}

fn find_path(
    forbidden_path: &HashSet<(usize, usize)>,
    adj_matrix: &Vec<Vec<usize>>,
    start: usize,
    end: usize,
) -> Option<Vec<usize>> {
    let mut already_visited = HashSet::new();
    let mut queue: VecDeque<(usize, Vec<usize>)> = VecDeque::new();
    queue.push_back((start, vec![start]));
    while let Some(entry) = queue.pop_front() {
        for &next in &adj_matrix[entry.0] {
            if already_visited.contains(&next) || forbidden_path.contains(&(entry.0, next)) {
                continue;
            }
            if next == end {
                let mut queue = entry.1;
                queue.push(next);
                return Some(queue);
            }
            already_visited.insert(next);
            let mut next_queue = entry.1.clone();
            next_queue.push(next);
            queue.push_back((next, next_queue));
        }
    }
    return None;
}

fn finish_connection(
    connections: &Vec<(&str, &str)>,
    forbidden_edges: &Vec<(usize, usize)>,
    unique_nodes: &HashMap<String, usize>,
) {
    // for connection0 in 0..connections.len() {
    //     for connection1 in connection0 + 1..connections.len() {
    //         println!("{}", connection1);
    //         for connection2 in connection1 + 1..connections.len() {
    let mut groups = vec![1; unique_nodes.len()];
    let mut is_connected = vec![false; unique_nodes.len()];
    let mut number_of_connections = vec![vec![0; unique_nodes.len()]; unique_nodes.len()];
    for connection in 0..connections.len() {
        // if connection != connection0 && connection != connection1 && connection != connection2 {
        let pair = connections[connection];
        let mut index0 = *unique_nodes.get(pair.0).unwrap();
        let mut index1 = *unique_nodes.get(pair.1).unwrap();
        if forbidden_edges.contains(&(index0, index1))
            || forbidden_edges.contains(&(index1, index0))
        {
            continue;
        }
        while is_connected[index0] {
            index0 = groups[index0];
        }
        while is_connected[index1] {
            index1 = groups[index1];
        }
        if index0 == index1 {
            continue;;
        }
        if groups[index0] < groups[index1] {
            swap(&mut index0, &mut index1);
        }
        groups[index0] += groups[index1];
        is_connected[index1] = true;
        groups[index1] = index0;
    }
    // }
    if is_connected.iter().filter(|s| !**s).count() == 2 {
        let result: usize = is_connected
            .iter()
            .enumerate()
            .filter(|(i, s)| !**s)
            .map(|(i, s)| groups[i])
            .product();
        println!("{}", result);
        return;
    }
}
