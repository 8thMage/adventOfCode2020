use core::panic;
use std::collections::*;
mod helpers;
use helpers::*;
fn main() {
    let input = include_str!("input.txt");
    let mut lines = input.lines();
    let nums = lines
        .next()
        .unwrap()
        .split(",")
        .map(|s| s.parse::<i32>().unwrap());
    lines.next();
    let mut boards = vec![];
    let mut curr_board = vec![];
    for line in lines {
        if line == "" {
            boards.push(curr_board);
            curr_board = vec![];
            continue;
        }
        let board_line: Vec<i32> = line
            .split_ascii_whitespace()
            .map(|s| s.parse::<i32>().unwrap())
            .collect();
        curr_board.push(board_line);
    }
    boards.push(curr_board);
    for popped_num in nums {
        for board in boards.iter_mut() {
            for line in board.iter_mut() {
                for num in line.iter_mut() {
                    if *num == popped_num {
                        *num = -1;
                    }
                }
            }
        }
        let mut board_index = 0;
        loop {
            if (board_index == boards.len()) {
                break;
            }
            if (is_board_win(&boards[board_index])) {
                let board = &boards[board_index];
                let mut sum = 0;
                for line in board {
                    for num in line {
                        if (*num != -1) {
                            sum += *num;
                        }
                    }
                }
                boards.remove(board_index);
                println!("score {}", sum * popped_num);
            } else {
                board_index +=1;
            }
        }
    }
}

fn is_board_win(board: &Vec<Vec<i32>>) -> bool {
    for i in 0..5 {
        for j in 0..5 {
            if (board[i][j] != -1) {
                break;
            }
            if (j == 4) {
                return true;
            }
        }
    }
    for i in 0..5 {
        for j in 0..5 {
            if (board[j][i] != -1) {
                break;
            }
            if (j == 4) {
                return true;
            }
        }
    }
    return false;
}
