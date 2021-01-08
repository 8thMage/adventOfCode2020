use core::panic;
fn main() {
  let mut max = 0;
  for i in 100..1000 {
      for j in 100..1000 {
        if isPalindrome(i * j) {
            max = (i * j).max(max);
        }
      }
  }
  println!("{}", max);
}

fn isPalindrome(num:i32) -> bool {
    let mut stack = std::collections::vec_deque::VecDeque::new();
    let mut new_num = num;
    while new_num > 0 {
        stack.push_front(new_num % 10);
        new_num = new_num / 10;
    } 
    while stack.len() > 1 {
        if stack.pop_back() != stack.pop_front() {
            return false;
        }
    }
    true
}
