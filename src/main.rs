use std::{
    collections::{HashMap, HashSet, VecDeque},
    hash::Hash,
    path,
    str::FromStr,
    thread::current,
    vec,
};

#[derive(Eq, Hash, PartialEq, Clone)]
struct Key<'a> {
    pos: &'a str,
    elephant_pos: &'a str,
    open_valves: Vec<&'a str>,
}

fn astar<'a>(
    map: &'a HashMap<&str, (i32, Vec<&str>)>,
    cost: &mut HashMap<Key<'a>, i32>,
    key: Key<'a>,
    pressure: i32,
    minute: i32,
) -> i32 {
    let mut wavefront = VecDeque::new();
    wavefront.push_back((key, vec!["AA"], vec!["AA"], pressure, minute));
    let mut max_pressure = 0;
    while !wavefront.is_empty() {
        let (key, path_since_instertion, path_since_instertion_elephant, pressure, minute) =
            wavefront.pop_front().unwrap();
        if cost.get(&key).map_or(false, |p| *p > pressure) {
            continue;
        }
        max_pressure = pressure.max(max_pressure);

        if minute > 26 {
            continue;
        }
        if !key.open_valves.iter().any(|k| k == &key.pos) {
            let rate = map[key.pos].0;
            if rate != 0 {
                let new_minute: i32 = minute + 1;
                let mut new_key = key.clone();
                new_key.open_valves.push(key.pos);
                let new_pressure = pressure + rate * (30 - new_minute);
                if !new_key.open_valves.iter().any(|k| k == &key.elephant_pos) {
                    let rate = map[key.elephant_pos].0;
                    if rate != 0 {
                        new_key.open_valves.push(key.elephant_pos);
                        let new_pressure = new_pressure + rate * (30 - new_minute);
                        let position = wavefront
                            .iter()
                            .position(|&(_, _, _, p, _)| p < new_pressure)
                            .unwrap_or(wavefront.len());
                        if cost.get(&new_key).map_or(false, |p| *p > new_pressure) {
                            continue;
                        }

                        wavefront.insert(
                            position,
                            (
                                new_key.clone(),
                                vec![key.pos],
                                vec![key.elephant_pos],
                                pressure + rate * (30 - new_minute),
                                new_minute,
                            ),
                        );
                    }
                }
                for entry in &map[key.elephant_pos].1 {
                    if path_since_instertion_elephant.contains(entry) {
                        continue;
                    }
                    let mut path_since_instertion_elephant = path_since_instertion_elephant.clone();
                    path_since_instertion_elephant.push(entry);
                    let mut new_key = new_key.clone();
                    new_key.elephant_pos = entry;
                    let position = wavefront
                        .iter()
                        .position(|&(_, _, _, p, minute)| {
                            p < new_pressure || (p == new_pressure && minute > new_minute)
                        })
                        .unwrap_or(wavefront.len());
                    if cost.get(&new_key).map_or(false, |p| *p > pressure) {
                        continue;
                    }
                    wavefront.insert(
                        position,
                        (
                            new_key,
                            vec![key.pos],
                            path_since_instertion_elephant,
                            pressure,
                            minute + 1,
                        ),
                    );
                }
                {
                    let position = wavefront
                        .iter()
                        .position(|&(_, _, _, p, minute)| {
                            p < new_pressure || (p == new_pressure && minute > new_minute)
                        })
                        .unwrap_or(wavefront.len());

                    wavefront.insert(
                        position,
                        (
                            new_key,
                            vec![key.pos],
                            path_since_instertion_elephant.clone(),
                            pressure,
                            minute + 1,
                        ),
                    );
                }
            }
        }

        for entry in &map[key.pos].1 {
            if path_since_instertion.contains(entry) {
                continue;
            }
            let mut path_since_instertion = path_since_instertion.clone();
            path_since_instertion.push(entry);
            let mut new_key = key.clone();
            new_key.pos = entry;
            if !key.open_valves.iter().any(|k| k == &key.elephant_pos) {
                let rate = map[key.elephant_pos].0;
                if rate != 0 {
                    let new_minute = minute + 1;
                    new_key.open_valves.push(key.elephant_pos);
                    let new_pressure = pressure + rate * (30 - new_minute);
                    let position = wavefront
                        .iter()
                        .position(|&(_, _, _, p, minute)| {
                            p < new_pressure || (p == new_pressure && minute > new_minute)
                        })
                        .unwrap_or(wavefront.len());
                    if cost.get(&new_key).map_or(false, |p| *p > new_pressure) {
                        continue;
                    }

                    wavefront.insert(
                        position,
                        (
                            new_key.clone(),
                            path_since_instertion.clone(),
                            vec![key.elephant_pos],
                            pressure + rate * (30 - new_minute),
                            new_minute,
                        ),
                    );
                }
            }
            for entry in &map[key.elephant_pos].1 {
                if path_since_instertion_elephant.contains(entry) {
                    continue;
                }
                let mut path_since_instertion_elephant = path_since_instertion_elephant.clone();
                path_since_instertion_elephant.push(entry);
                let mut new_key = new_key.clone();
                new_key.elephant_pos = entry;
                let position = wavefront
                    .iter()
                    .position(|&(_, _, _, p, minute1)| {
                        p < pressure || (p == pressure && minute1 > minute + 1)
                    })
                    .unwrap_or(wavefront.len());
                if cost.get(&new_key).map_or(false, |p| *p > pressure) {
                    continue;
                }
                wavefront.insert(
                    position,
                    (
                        new_key,
                        path_since_instertion.clone(),
                        path_since_instertion_elephant,
                        pressure,
                        minute + 1,
                    ),
                );
            }
            {
                let position = wavefront
                    .iter()
                    .position(|&(_, _, _, p, minute1)| {
                        p < pressure || (p == pressure && minute1 > minute + 1)
                    })
                    .unwrap_or(wavefront.len());

                wavefront.insert(
                    position,
                    (
                        new_key,
                        path_since_instertion.clone(),
                        path_since_instertion_elephant.clone(),
                        pressure,
                        minute + 1,
                    ),
                );
            }
        }
        cost.insert(key, pressure);
    }
    max_pressure
}

fn astar2<'a>(
    map: &HashMap<&'static str, (i32, Vec<&'a str>)>,
    connectivity_map: &HashMap<(&'a str, &'a str), u32>,
    mut key: Key<'a>,
    pressure: i32,
    minute: i32,
) -> i32 {
    let new_pressure = pressure + (30 - minute) * map[key.pos].0;
    key.open_valves.push(key.pos);
    let mut max_pressure = new_pressure;
    for entry in map
        .iter()
        .filter(|(entry, (pressure, _))| *pressure > 0 && !key.open_valves.contains(entry))
    {
        let delta_time = (connectivity_map[&(key.pos, *entry.0)] as i32) + 1;
        if delta_time < 30 - minute {
            let mut new_key = key.clone();
            new_key.pos = entry.0;
            max_pressure = max_pressure.max(astar2(
                map,
                connectivity_map,
                new_key,
                new_pressure,
                minute + delta_time,
            ));
        }
    }
    return max_pressure;
}

#[derive(Eq, Hash, PartialEq, Clone)]
struct KeyWithElephant<'a> {
    pos: &'a str,
    arrival_time: i32,
    elephant_pos: &'a str,
    elephant_arrival_time: i32,
    open_valves: Vec<&'a str>,
}

fn astar2_with_elephant<'a>(
    map: &HashMap<&'static str, (i32, Vec<&'a str>)>,
    connectivity_map: &HashMap<(&'a str, &'a str), u32>,
    mut key: KeyWithElephant<'a>,
    cost: &mut HashMap<(&'a str, &'a str, Vec<&'a str>), i32>,
    pressure: i32,
    minute: i32,
) -> i32 {
    let mut wavefront: Vec<(KeyWithElephant<'a>, i32, i32, i32)> = vec![(key, 0, i32::MAX, 0)];
    let mut max_pressure = 0;
    let mut max_open_valve = 0;
    while !wavefront.is_empty() {
        let (mut key, pressure, euristic, minute) = wavefront.pop().unwrap();
        if minute == 26 {
            continue;
        }

        let mut new_pressure = pressure;
        let mut possible_dests = vec![(
            key.pos,
            key.arrival_time,
            map[key.pos].0,
            key.arrival_time - minute,
        )];
        let mut possible_elephant_dests = vec![(
            key.elephant_pos,
            key.elephant_arrival_time,
            map[key.elephant_pos].0,
            key.elephant_arrival_time - minute,
        )];
        if minute >= key.arrival_time {
            if !key.open_valves.contains(&key.pos) {
                new_pressure = new_pressure + (26 - minute) * map[key.pos].0;
            }
            key.open_valves.push(key.pos);
            key.open_valves.sort();

            possible_dests = vec![];
            for entry in map
                .iter()
                .filter(|(entry, (pressure, _))| *pressure > 0 && !key.open_valves.contains(entry))
            {
                let delta_time = (connectivity_map[&(key.pos, *entry.0)] as i32) + 1;
                if delta_time < 26 - minute {
                    possible_dests.push((*entry.0, minute + delta_time, entry.1 .0, delta_time));
                }
            }
            possible_dests.push((key.pos, 26, 0, 26))
        }
        if minute >= key.elephant_arrival_time {
            if !key.open_valves.contains(&key.elephant_pos) {
                new_pressure = new_pressure + (26 - minute) * map[key.elephant_pos].0;
            }
            key.open_valves.push(key.elephant_pos);
            key.open_valves.sort();

            possible_elephant_dests = vec![];
            for entry in map
                .iter()
                .filter(|(entry, (pressure, _))| *pressure > 0 && !key.open_valves.contains(entry))
            {
                let delta_time = (connectivity_map[&(key.elephant_pos, *entry.0)] as i32) + 1;
                if delta_time < 26 - minute {
                    possible_elephant_dests.push((
                        *entry.0,
                        minute + delta_time,
                        entry.1 .0,
                        delta_time,
                    ));
                }
            }
            possible_elephant_dests.push((key.elephant_pos, 26, 0, 26 - minute));
        }
        // if new_pressure + euristic < max_pressure {
        //     continue;
        // }
        if key.open_valves.len() > max_open_valve {
            println!("{}", key.open_valves.len());
            max_open_valve = key.open_valves.len();
        }
        if *cost
            .get(&(&key.pos, &key.elephant_pos, key.open_valves.clone()))
            .unwrap_or(&-1)
            > new_pressure
            || *cost
                .get(&(&key.elephant_pos, &key.pos, key.open_valves.clone()))
                .unwrap_or(&-1)
                > new_pressure
        {
            continue;
        }
        cost.insert(
            (key.pos, key.elephant_pos, key.open_valves.clone()),
            new_pressure,
        );
        max_pressure = max_pressure.max(new_pressure);
        possible_dests
            .sort_by_key(|&(_, _, pressure, delta_time)| -pressure * (26 - minute - delta_time));
        possible_elephant_dests
            .sort_by_key(|&(_, _, pressure, delta_time)| -pressure * (26 - minute - delta_time));
        let max_possible_dest = possible_dests[0].2;
        let max_elephant_dest = possible_elephant_dests[0].2;
        // if possible_dests.len() + possible_elephant_dests.len() < 16 - key.open_valves.len() - 2 {
        //     continue;
        // }
        let bound = map
            .iter()
            .filter(|(entry, (pressure, _))| *pressure > 0 && !key.open_valves.contains(entry))
            .map(|(entry, (pressure, _))| pressure * (26 - minute))
            .sum::<i32>()
            + map
                .iter()
                .filter(|(entry, (pressure, _))| *pressure > 0 && !key.open_valves.contains(entry))
                .map(|(entry, (pressure, _))| pressure * (26 - minute) as i32)
                .sum::<i32>();
        if max_pressure > bound + new_pressure {
            continue;
        }

        for dest in &possible_dests {
            for elephant_dest in &possible_elephant_dests {
                let mut new_key = key.clone();
                new_key.elephant_arrival_time = elephant_dest.1;
                new_key.elephant_pos = elephant_dest.0;
                new_key.pos = dest.0;
                new_key.arrival_time = dest.1;
                if new_key.pos == new_key.elephant_pos
                    && (new_key.arrival_time != 26 || new_key.elephant_arrival_time != 26)
                {
                    continue;
                }
                let time = new_key.elephant_arrival_time.min(new_key.arrival_time);

                let new_euristic = 0;
                let position = wavefront
                    .iter()
                    .position(|(_, p, euristic, _)| (*p + euristic) > (new_pressure + new_euristic))
                    .unwrap_or(wavefront.len());
                wavefront.insert(position, (new_key, new_pressure, new_euristic, time));
            }
        }
    }
    return max_pressure;
}

fn create_connectivity_map<'a>(
    map: &HashMap<&'static str, (i32, Vec<&'a str>)>,
    connectivity_map: &mut HashMap<(&'a str, &'a str), u32>,
    pos: &'static str,
) {
    for entry in map.keys() {
        connectivity_map.insert((pos, entry), u32::MAX);
    }
    let mut next_nodes = vec![(pos, 0)];
    while !next_nodes.is_empty() {
        let node = next_nodes.pop().unwrap();
        for entry in &map[node.0].1 {
            if connectivity_map[&(pos, *entry)] > node.1 + 1 {
                connectivity_map.insert((pos, entry), node.1 + 1);
                next_nodes.push((entry, node.1 + 1))
            }
        }
    }
}

fn main() {
    let input = include_str!("input.txt");
    let mut map = HashMap::new();
    for line in input.lines() {
        let mut split = line.split([' ', '=', ';', ',']);
        let name = split.nth(1).unwrap();
        let rate = i32::from_str(split.nth(3).unwrap()).unwrap();
        let connections: Vec<_> = split.skip(5).filter(|s| s.len() != 0).collect();
        map.insert(name, (rate, connections));
    }
    // let mut cost = HashMap::new();
    let key = Key {
        pos: "AA",
        elephant_pos: "AA",
        open_valves: vec![],
    };
    let mut connectivity_map = HashMap::new();
    create_connectivity_map(&map, &mut connectivity_map, "AA");
    for entry in map.iter().filter(|(_, (pressure, _))| *pressure > 0) {
        create_connectivity_map(&map, &mut connectivity_map, entry.0);
    }
    println!("finish conn");
    // let pressure = astar2(&map, &connectivity_map, key, 0, 0);
    let key_with_elephant = KeyWithElephant {
        pos: "AA",
        arrival_time: 0,
        elephant_pos: "AA",
        elephant_arrival_time: 0,
        open_valves: vec![],
    };
    let pressure = astar2_with_elephant(
        &map,
        &connectivity_map,
        key_with_elephant,
        &mut HashMap::new(),
        0,
        0,
    );
    println!("{}", pressure)
}
