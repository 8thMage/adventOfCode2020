use core::panic;
mod helpers;
use helpers::*;
fn main() {
    let time = std::time::Instant::now();
    let p = erathostenes_sieve(1000000);
    for &prime in &p {
        if prime == 120383 {
            let _x = 0;
        }
        let mut digits = Vec::new();
        let mut newP = prime;
        while newP != 0 {
            digits.push(newP % 10);
            newP /= 10;
        }
        let digits = digits;
        let mut max_count = 0;
        for i in 1usize..(1 << digits.len()) { //>
            let mut newDigits = digits.clone();
            let mut count = 0;
            let setBit = i.trailing_zeros() as usize;
            let sameBit = newDigits.iter().enumerate().fold(true, |acc, (index, &c)| acc && (((i >> index) & 1 == 0) || c == newDigits[setBit]));
            if (!sameBit) {
                continue;
            }
            for new_digit in (i >> digits.len() - 1) & 1 as usize..10 {
                for digit in 0..digits.len() {
                    if (i >> digit) & 1 == 1 {
                        newDigits[digit] = new_digit;
                    }
                }
                let newNumber = newDigits.iter().rev().fold(0, |acc, d| acc * 10 + d);
                if p.binary_search(&newNumber).is_ok() {
                    count += 1;
                }
            }
            max_count = max_count.max(count);
        }
        if max_count == 8 {
            println!("{}", prime);
            break;
        }
    }
    println!("{}", std::time::Instant::now().duration_since(time).as_millis());
}
