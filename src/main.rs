use core::panic;
use std::collections::*;
mod helpers;
use helpers::*;
fn main() {
    let time = std::time::Instant::now();
    'i: for i in 1..100000 {
        let firstLen = digits(i).iter().cloned().collect::<HashSet<usize>>();
        let twoLen = digits(2 * i).iter().cloned().collect::<HashSet00000<usize>>();
        let threeLen = digits(3 * i)
            .iter()
            .cloned()
            .collect::<HashSet<usize>>()
            .len();
        let fourLen = digits(4 * i)
            .iter()
            .cloned()
            .collect::<HashSet<usize>>()
            .len();
        let fiveLen = digits(5 * i)
            .iter()
            .cloned()
            .collect::<HashSet<usize>>()
            .len();
        let sixLen = digits(6 * i)
            .iter()
            .cloned()
            .collect::<HashSet<usize>>()
            .len();
        
        for &d in &firstLen {
            if !twoLen.contains(&d) {
                continue 'i;
            }
        }
        if firstLen.len() != twoLen.len() {
            continue 'i;
        }
        println!("{}", i);
        break;
    }
    println!(
        "{}",
        std::time::Instant::now().duration_since(time).as_millis()
    );
}
