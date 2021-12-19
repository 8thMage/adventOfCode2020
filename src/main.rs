use core::panic;
use std::collections::*;
mod helpers;
use helpers::*;
fn main() {
    let input = include_str!("input.txt");
    let mut lines = input.lines();
    let mut points   = HashSet::new();
    for line in lines.by_ref() {
        if (line == "") {
            break;
        }
        let mut values = line.split(",").map(|s| s.parse::<i32>().unwrap());
        points.insert((values.next().unwrap(), values.next().unwrap()));
    }
    for line in lines {
        if line.contains("fold along x=") {
            let x_line = line.split_once("fold along x=").unwrap().1.parse::<i32>().unwrap();
            points = points.into_iter().map(|p| {
                if p.0 > x_line {
                    return (2 * x_line - p.0, p.1);
                }
                return (p.0, p.1);
            }).collect();
        } else if line.contains("fold along y=") {
            let y_line = line.split_once("fold along y=").unwrap().1.parse::<i32>().unwrap();
            points = points.into_iter().map(|p| {
                if p.1 > y_line {
                    return (p.0, 2 * y_line - p.1);
                }
                return (p.0, p.1);
            }).collect();
        }
    }
    let mut maxX = 0;
    let mut minX = 100000;
    let mut maxY = 0;
    let mut minY = 100000;
    for p in &points {
        if p.0 < minX {
            minX = p.0;
        }
        if p.0 > maxX {
            maxX = p.0;
        }
        if p.1 < minY {
            minY = p.1;
        }
        if p.1 > maxY {
            maxY = p.1;
        }
    }

    for y in minY..maxY + 1 {
        for x in minX..maxX + 1 {
            if points.get(&(x, y)) != None {
                print!("#");
            }
            else {
                print!(".");
            }
        }
        println!("");
    }
    // println!("{}", points.len());

}
