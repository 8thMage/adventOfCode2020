use std::{
    collections::{HashMap, HashSet},
    thread::current,
};

fn main() {
    let time = std::time::Instant::now();
    let seive_size = 1 << 28;
    let mut seive = vec![false; seive_size];
    seive[0] = true;
    seive[1] = true;
    let mut count = 0;
    for i in 2..seive_size {
        if seive[i] == false {
            count += 1;
            // println!("{:b}", i);
            if count == 5_000_000 {
                let duration = time.elapsed().as_millis();
                println!("5_000_000th prime {}", i);
                println!("time {}", duration);
            }
            let mut j = 2;
            loop {
                let index = carry_less_mul(i as u64, j);
                if index as usize >= seive.len() * 2 {
                    break;
                }
                j += 1;
                if index as usize >= seive.len() {
                    continue;
                }
                seive[index as usize] = true;
            }
        }
    }
    println!("count {}", count);
    let x = 0;
}

fn carry_less_mul(mut a: u64, mut b: u64) -> u64 {
    if a < b {
        return carry_less_mul(b, a);
    }
    let mut sum = 0;
    while b != 0 {
        if b % 2 != 0 {
            sum ^= a;
        }
        a <<= 1;
        b >>= 1;
    }
    sum
}
