pub fn prime_factors(a: i64) -> std::collections::HashSet<i64> {
    let mut new_a = a;
    let mut res = std::collections::HashSet::new();
    let mut i = 2;
    'i: while i * i < new_a {
        if new_a % i == 0 {
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

pub fn is_prime(a: i64) -> bool {
    let mut j = 2;
    while j * j < a {
        if a % j == 0 {
            return false;
        }
        j += 1;
    }
    true
}

pub fn erathostenes_sieve(max_value: usize) -> Vec<usize> {
    let mut primes = Vec::new();
    let mut sieve = Vec::new();
    sieve.resize(max_value, false);
    for i in 2..max_value {
        if sieve[i] == false {
            primes.push(i);
        }
        sieve.iter_mut().skip(i).step_by(i).for_each(|mut c| *c = true );
    }
    return primes;
}
