use core::panic;
fn main() {
    println!("{}", gcd(10, 5));
    println!("{}", gcd(20, 10));
    println!("{}", gcd(100, 10));
    let mut lcm = 1i64;
    for i in 2..21 {
        lcm = i * lcm / gcd(lcm, i);
    }
    println!("{}", lcm);
}

fn gcd (a:i64, b:i64) -> i64 {
    if a < b {
        return gcd(b, a);
    }
    if b == 1 {
        return b;
    }
    if b == 0 {
        return a;
    }
    return gcd(b, a % b);
}
