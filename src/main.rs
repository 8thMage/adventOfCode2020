use core::panic;
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

fn prime_factors(a: i64) -> std::collections::HashSet<i64> {
    let mut new_a = a;
    let mut res = std::collections::HashSet::new();
    let mut i = 2;
    'i: while i * i < new_a {
        if new_a % i == 0 {
            let mut j = 2;
            while j * j < i {
                if i % j == 0 {
                    i += 1;
                    continue 'i;
                }
                j += 1;
            }
            while new_a % i == 0 {
                new_a /= i;
            }
            res.insert(i);
        }
        i += 1;
    }
    res.insert(new_a);
    return res;
}
