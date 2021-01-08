use core::panic;
use std::collections::{btree_map::Iter, HashMap, HashSet, VecDeque};
fn main() {
    let start_time = std::time::Instant::now();
    let input = include_str!("input.txt");
    let mut subject = 7isize;
    let mut value = 1isize;
    let cardTarget = 13135480;
    let mut card_loop_size = 0;
    while cardTarget != value {
        value = value * subject;
        value = value % 20201227;
        card_loop_size +=1;
    }
    println!("{}", card_loop_size);
    value = 1;
    let doorTarget = 8821721;
    let mut door_loop_size = 0;
    while doorTarget != value {
        value = value * subject;
        value = value % 20201227;
        door_loop_size +=1;
    }
    println!("{}", door_loop_size);
    value = 1;
    subject = doorTarget;
    for _ in 0..card_loop_size {
        value = value * subject;
        value = value % 20201227;
    }
    println!("{}", value);
    value = 1;
    subject = cardTarget;

    for _ in 0..door_loop_size {
        value = value * subject;
        value = value % 20201227;
    }
    println!("{}", value);

}
