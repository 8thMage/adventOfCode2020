use core::panic;
fn main() {
    let mut newNum:i64 = 600851475143;
    'i: for i in 2..newNum - 1 {
        if i% 1000 == 0 {
            // println!("{}", i);
        }
        if newNum % i == 0 {
            for j in 2..i {
                if i % j == 0 {
                    continue 'i;
                }
            }
            while newNum % i ==0 {
                newNum = newNum / i;
            }
            println!("{}", i);
        }
    }
}
