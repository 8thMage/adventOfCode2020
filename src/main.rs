use std::{collections::HashMap, thread::current};
fn main() {
    let input = include_str!("input.txt");
//     let input = "939
// 7,13,x,x,59,x,31,19";
// let input = "939
// 17,x,13,19";

let x = input.lines().next().unwrap().parse::<usize>().unwrap();
    let busses = input.lines().skip(1).next().unwrap().split_terminator(",");
    let res = busses.map(|bus| if let Ok(bus_number) = bus.parse::<usize>() {
        (bus_number - x % bus_number, bus_number)
    } else {
        (usize::MAX, 1)
    }
    ).min().unwrap();
    println!("{} {} {}", res.0, res.1, res.0 * res.1);
    let busses = input.lines().skip(1).next().unwrap().split_terminator(",");
    let mut bussesFine = busses.map(|s| s.parse::<usize>().is_ok()).collect::<Vec<bool>>();    
    let busses = input.lines().skip(1).next().unwrap().split_terminator(",");
    let mut bussesSorted = busses.map(|s| s.parse::<usize>()).filter(|b|b.is_ok()).map(|b|b.unwrap()).collect::<Vec<usize>>();    
    let all_prime = bussesSorted.iter().all(|b| is_prime(*b));
    let mut current_gcd = bussesSorted[0] as i128;
    let mut current_number = 0i128;
    let (a, b) = euqclide(11, 7);
    println!("{} {}", a,b);
    let mut count = 0;
    let mut iter = bussesFine.iter().enumerate().peekable();
    for i in 1..bussesSorted.len() as isize {
        iter.next();
        while (!*iter.peek().unwrap().1) {
            iter.next();
        };
        let p = current_number as i128;
        let q = bussesSorted[i as usize] as i128;
        let (a, b) = euqclide(current_gcd, q);
        let a = a.rem_euclid(current_gcd * q as i128);
        let b = b.rem_euclid(current_gcd * q as i128);
        println!("{:x?} {:x?}", current_number,current_gcd);
        let n = (q  - iter.peek().unwrap().0 as i128).rem_euclid(current_gcd * q as i128) as i128;
        let half1 = (a * current_gcd).rem_euclid(current_gcd * q as i128) * n;
        let half2 = (current_number * b).rem_euclid(current_gcd * q as i128) * q;
        current_number =  (half1 + half2).rem_euclid(current_gcd * q as i128);
        current_gcd =current_gcd * q;
    }
    print!("{}", current_number);
}

fn euqclide(a:i128, b:i128) -> (i128, i128) {
    if (a < b) {
        let (x, y) = euqclide(b, a);
        return (y,x);
    }
    let r = a % b;
    let q = a / b;
    if (r == 1) {
        return (r as i128, - (q as i128));
    } 
    else {
        let (x, y) = euqclide(b, r);
        return (y as i128, x - y as i128 * q as i128);
    }
}

fn is_prime(x:usize)->bool {
    for i in 2..x {
        if(i * i > x) {
            return true;
        } 
        if x % i == 0 {
            return false;
        }
    }
    return true;
}
