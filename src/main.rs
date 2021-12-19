use core::panic;
use std::collections::*;
mod helpers;
use helpers::*;
fn main() {
    let input = include_str!("input.txt");
    let mut map: HashMap<&str, Vec<&str>> = HashMap::new();
    for line in input.lines() {
        let (a, b) = line.split_once('-').unwrap();
        if (map.get(a) == None) {
            map.insert(a, vec![]);
        }
        if (map.get(b) == None) {
            map.insert(b, vec![]);
        }
        map.get_mut(a).unwrap().push(b);
        map.get_mut(b).unwrap().push(a);
    }
    let s = "s";
    let mut hash = HashSet::new();
    hash.insert("start");
    let mut string = String::from("");
    println!("{}", number_of_paths("start", &map, &mut string, &mut hash));
}

fn number_of_paths<'a>(
    current_station: &str,
    connectivity: &HashMap<&'a str, Vec<&'a str>>,
    visited_twice_in: &mut String,
    hash_set: &mut HashSet<&'a str>,
) -> i32 {
    if (current_station == "end") {
        return 1;
    }
    let mut sum = 0;
    for station in &connectivity[current_station] {
        let isUpperCase = station.chars().next().unwrap() <= 'Z';
        if (!isUpperCase) {
            if hash_set.contains(station) {
                if (visited_twice_in == "" && station != &"start") {
                    *visited_twice_in = String::from(*station);
                } else {
                    continue;
                }
            }
            hash_set.insert(*station);
        }
        sum += number_of_paths(station, connectivity, visited_twice_in, hash_set);
        if (!isUpperCase) {
            if visited_twice_in == *station {
                *visited_twice_in = String::from(""); 
                continue;
            }
            hash_set.remove(*station);
        }
    }
    return sum;
}
