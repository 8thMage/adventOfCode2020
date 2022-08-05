// use core::panic;
use std::{cell::RefCell};
// mod helpers;
// use helpers::*;

enum SnailNumberValue {
    SnailValue(usize),
    SnailPointer(Box<RefCell<SnailNumber>>),
}

use SnailNumberValue::*;

struct SnailNumber {
    left_value: SnailNumberValue,
    right_value: SnailNumberValue,
}

fn create_snail_number_from_string(s: &str) -> (usize, SnailNumberValue) {
    if &s[0..1] == "[" {
        let (len, left) = create_snail_number_from_string(&s[1..]);
        let (right_len, right) = create_snail_number_from_string(&s[2 + len..]);
        let new_snail = Box::new(RefCell::new(SnailNumber {
            left_value: left,
            right_value: right,
        }));
        (right_len + len + 3, SnailPointer(new_snail))
    } else {
        (
            1,
            SnailNumberValue::SnailValue(usize::from_str_radix(&s[0..1], 10).unwrap()),
        )
    }
}

fn print_snail(snail: &SnailNumberValue) {
    match snail {
        SnailNumberValue::SnailValue(value) => {
            print!("{}", value)
        }
        SnailNumberValue::SnailPointer(snail) => {
            print!("[");
            print_snail(&snail.borrow().left_value);
            print!(",");
            print_snail(&snail.borrow().right_value);
            print!("]");
        }
    }
}

#[derive(PartialEq)]
enum Action {
    Split(usize),
    Explode {
        left_value: usize,
        right_value: usize,
    },
    ExplodeLeft {
        left_value: usize,
    },
    ExplodeRight {
        right_value: usize,
    },
    DoneAction,
    None,
}

fn find_next_to_right(snail: &mut SnailNumberValue, update_value: usize) -> bool {
    match snail {
        SnailValue(value) => {
            *value += update_value;
            return true;
        }
        SnailPointer(snail) => {
            if find_next_to_right(&mut snail.borrow_mut().left_value, update_value) {
                return true;
            }
            return false;
        }
    }
}

fn find_next_to_left(snail: &mut SnailNumberValue, update_value: usize) -> bool {
    match snail {
        SnailValue(value) => {
            *value += update_value;
            return true;
        }
        SnailPointer(snail) => {
            if find_next_to_left(&mut snail.borrow_mut().right_value, update_value) {
                return true;
            }
            return false;
        }
    }
}

fn do_split(snail: &mut SnailNumberValue, index: usize) -> Action {
    match snail {
        SnailValue(value) => {
            if *value >= 10 {
                Action::Split(*value)
            } else {
                Action::None
            }
        }
        SnailPointer(snail) => {
            let left_action = do_split(&mut snail.borrow_mut().left_value, index + 1);
            match left_action {
                Action::Split(value) => {
                    snail.borrow_mut().left_value = SnailPointer(Box::new(RefCell::new(SnailNumber {
                        left_value: SnailValue(value / 2),
                        right_value: SnailValue(value - value / 2),
                    })));
                    return Action::DoneAction;
                }
                Action::None => {}
                Action::DoneAction => {
                    return Action::DoneAction;
                }
                _ => {},
            }

            let right_action = do_split(&mut snail.borrow_mut().right_value, index + 1);
            match right_action {
                Action::Split(value) => {
                    snail.borrow_mut().right_value = SnailPointer(Box::new(RefCell::new(SnailNumber {
                        left_value: SnailValue(value / 2),
                        right_value: SnailValue(value - value / 2),
                    })));
                    return Action::DoneAction;
                }
                Action::None => {}
                Action::DoneAction => {
                    return Action::DoneAction;
                }
                _ => {}
            }
            return Action::None;
        }
    }
}

fn do_explode(snail: &mut SnailNumberValue, index: usize) -> Action {
    match snail {
        SnailValue(value) => {
            Action::None
        }
        SnailPointer(snail) => {
            if index >= 4 {
                let left_value = match snail.borrow().left_value {
                    SnailValue(value) => value,
                    _ => panic!(),
                };
                let right_value = match snail.borrow().right_value {
                    SnailValue(value) => value,
                    _ => panic!(),
                };
                return Action::Explode { left_value, right_value };
            }
            let left_action = do_explode(&mut snail.borrow_mut().left_value, index + 1);
            match left_action {
                Action::Explode {
                    left_value,
                    right_value,
                } => {
                    snail.borrow_mut().left_value = SnailValue(0);
                    find_next_to_right(&mut snail.borrow_mut().right_value, right_value);
                    return Action::ExplodeLeft { left_value: left_value}
                }
                Action::ExplodeLeft { left_value } => {
                    return Action::ExplodeLeft { left_value };
                }
                Action::ExplodeRight { right_value } => {
                    find_next_to_right(&mut snail.borrow_mut().right_value, right_value);
                    return Action::DoneAction;
                }
                Action::None | Action::Split(_) => {}
                Action::DoneAction => {
                    return Action::DoneAction;
                }
            }

            let right_action = do_explode(&mut snail.borrow_mut().right_value, index + 1);
            match right_action {
                Action::Split(value) => {
                }
                Action::Explode {
                    left_value,
                    right_value,
                } => {
                    snail.borrow_mut().right_value = SnailValue(0);
                    find_next_to_left(&mut snail.borrow_mut().left_value, left_value);
                    return Action::ExplodeRight { right_value} 
                }
                Action::ExplodeLeft { left_value } => {
                    find_next_to_left(&mut snail.borrow_mut().left_value, left_value);
                    return Action::DoneAction;
                }
                Action::ExplodeRight { right_value } => {
                    return Action::ExplodeRight { right_value };
                }
                Action::None => {}
                Action::DoneAction => {
                    return Action::DoneAction;
                }
            }
            return Action::None;
        }
    }
}


fn do_action(snail: &mut SnailNumberValue, index: usize) -> Action {
    if do_explode(snail, 0) != Action::None {
        return Action::DoneAction;
    }
    if do_split(snail, 0) != Action::None {
        return Action::DoneAction;
    }
    return Action::None;
}

fn reduce_snail(snail: &mut SnailNumberValue) {
    // print_snail(&snail);
    // println!();
    while Action::None != do_action(snail, 0) {
        // print_snail(&snail);
        // println!();
    }
}

fn magnitude(snail: &SnailNumberValue) -> usize {
    match snail {
        SnailValue(value) => *value,
        SnailPointer(snail) => {
            magnitude(&snail.borrow().left_value) * 3 + magnitude(&snail.borrow().right_value) * 2
        }
    }
}
fn main() {
    let input = include_str!("input.txt");
    let mut first_number = input.lines().next().map(|s| create_snail_number_from_string(s)).unwrap();
    reduce_snail(&mut first_number.1);
    print_snail(&first_number.1);
    println!();
    let acc = input.lines().skip(1).fold(first_number.1, |acc, line| {
        println!();
        let second_fish = create_snail_number_from_string(line);
        let mut new_snail = SnailPointer(Box::new(RefCell::new(SnailNumber {
            left_value: acc,
            right_value: second_fish.1,
        })));
        reduce_snail(&mut new_snail);
        print_snail(&new_snail);
        println!();
        new_snail
    });
    print_snail(&acc);
    let mut max = 0;
    for line in input.lines() {
        for second_line in input.lines() {
            let mut first_number = create_snail_number_from_string(line);
            reduce_snail(&mut first_number.1);
        
            let second_fish = create_snail_number_from_string(second_line);
            let mut new_snail = SnailPointer(Box::new(RefCell::new(SnailNumber {
                left_value: first_number.1,
                right_value: second_fish.1,
            })));
            reduce_snail(&mut new_snail);
            max = max.max(magnitude(&new_snail));
        }
    }

    println!();
    println!("magnitude {}", magnitude(&acc));
    println!("max {}", max);
}
