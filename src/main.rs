use core::panic;
fn main() {
    let mut fib = Vec::new();
    fib.push(1);
    fib.push(1);
    while fib.last().unwrap() <= &4000000 {
        fib.push(fib.last().unwrap() + fib[fib.len() - 2]);
    }
    let sum:i32 = fib.iter().filter(|&x| x % 2 == 0).sum();
    println!("{}", sum);
}
