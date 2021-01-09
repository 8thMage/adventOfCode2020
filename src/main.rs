use core::panic;
mod helpers;
use helpers::prime_factors;
fn main() {
    let mut current = 2;
    loop {
        if prime_factors(current).len() == 4
            && prime_factors(current + 1).len() == 4
            && prime_factors(current + 2).len() == 4
            && prime_factors(current + 3).len() == 4
        {
            println!("{}", current);
            break;
        }
        current += 1;
    }
}

