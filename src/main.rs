use std::{
    cmp::Reverse,
    collections::{BinaryHeap, HashMap},
    iter::Cycle,
};

fn main() {
    let input = include_str!("input.txt");
    let map = input
        .lines()
        .map(|c| c.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();
    let empty_y = map
        .iter()
        .enumerate()
        .map(|(y, v)| (y, v.iter().all(|c| *c == '.')))
        .filter_map(|(y, b)| b.then_some(y))
        .collect::<Vec<_>>();
    let empty_x = (0..map[0].len())
        .map(|x| (x, map.iter().all(|v| v[x] == '.')))
        .filter_map(|(x, b)| b.then_some(x))
        .collect::<Vec<usize>>();
    let position_galaxies = map
        .iter()
        .enumerate()
        .flat_map(|(y, v)| std::iter::once(y).cycle().zip(v.iter().enumerate()))
        .filter_map(|(y, (x, c))| (*c == '#').then_some((y, x)))
        .collect::<Vec<_>>();
    let mut sum = 0;
    let mut sum2 = 0;
    for &(y0, x0) in &position_galaxies {
        for &(y1, x1) in &position_galaxies {
            let count_empty_x_between = empty_x
                .iter()
                .filter(|x| x.abs_diff(x0) + x.abs_diff(x1) == x0.abs_diff(x1))
                .count();
            let count_empty_y_between = empty_y
                .iter()
                .filter(|y| y.abs_diff(y0) + y.abs_diff(y1) == y0.abs_diff(y1))
                .count();
            sum +=
                x0.abs_diff(x1) + count_empty_x_between + y0.abs_diff(y1) + count_empty_y_between;
            sum2 += x0.abs_diff(x1) + count_empty_x_between*(1000000-1) + y0.abs_diff(y1) + count_empty_y_between*(1000000-1);
        }
    }
    println!("{}", sum / 2);
    println!("{}", sum2 / 2);
}
