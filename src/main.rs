use core::num;
// use core::panic;
use std::{collections::*, thread::current, cmp::Reverse};
// mod helpers;
// use helpers::*;
fn main() {
    let input = include_str!("input.txt");
    let mut area_iter = input.split(",").flat_map(|s| s.split("=").nth(1).unwrap().split("..").map(|s| i64::from_str_radix(s, 10).unwrap()));
    let min_x_area = area_iter.next().unwrap();
    let max_x_area = area_iter.next().unwrap();
    let min_y_area = area_iter.next().unwrap();
    let max_y_area = area_iter.next().unwrap();
    let mut sum = 0;
    for x in 0.. max_x_area + 1 {
        for y in min_y_area.. -min_y_area {

    let mut px = 0;
    let mut py = 0_i64;
    let mut vx = x;
    let mut vy = y;
    let mut hit= true;;
     loop {
        px += vx;
        py += vy;
        vx -= vx.signum();
        vy -= 1;

        if px >= min_x_area && px <= max_x_area && py >= min_y_area && py <= max_y_area {
            println!("{}, {} \t ", x, y);
            sum += 1;
            break;;
        }
        if py <= min_y_area {
            break;;
        }
    }
}
    }
    println!("{}", sum);
}

