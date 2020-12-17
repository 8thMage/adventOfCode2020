use std::collections::{HashMap, HashSet};
fn main() {
    let input = include_str!("input.txt");
    //     let input = "
    // class: 0-1 or 4-19
    // row: 0-5 or 8-19
    // seat: 0-13 or 16-19

    // your ticket:
    // 11,12,13

    // nearby tickets:
    // 3,9,18
    // 15,1,5
    // 5,14,9
    // ";

    let iter = input
        .lines()
        .filter_map(|s| s.split_terminator(": ").nth(1))
        .map(|s| s.split_terminator(" or "));
    let newArr = iter
        .map(|two| {
            two.map(|s| {
                let mut seperate = s.split("-").map(|a| a.parse::<u32>().unwrap());
                (seperate.nth(0).unwrap(), seperate.nth(0).unwrap())
            })
            .collect::<Vec<(u32, u32)>>()
        })
        .collect::<Vec<Vec<(u32, u32)>>>();
    let tickets = input
        .split_terminator("nearby tickets:")
        .nth(1)
        .unwrap()
        .lines()
        .skip(1)
        .map(|s| {
            s.split_terminator(",")
                .map(|v| v.parse::<u32>().unwrap())
                .collect::<Vec<u32>>()
        })
        .collect::<Vec<Vec<u32>>>();
    let mut sum = 0;
    let mut correctTickets = Vec::new();
    for ticketId in 0..tickets.len() {
        let ticket = &tickets[ticketId];
        let mut allCorrect = true;
        for &val in ticket {
            let correct = newArr.iter().fold(false, |acc, pairs| {
                acc || pairs
                    .iter()
                    .fold(false, |acc, &(a, b)| acc || ((a <= val) && (val <= b)))
            });
            if (!correct) {
                sum += val;
                allCorrect = false;
            }
        }
        if allCorrect {
            correctTickets.push(ticket);
        }
    }
    for ticketId in 0..correctTickets.len() {
        let ticket = correctTickets[ticketId];
        let mut allCorrect = true;
        for &val in ticket {
            let correct = newArr.iter().fold(false, |acc, pairs| {
                acc || pairs
                    .iter()
                    .fold(false, |acc, &(a, b)| acc || ((a <= val) && (val <= b)))
            });
            if (!correct) {
                panic!();
            }
        }
    }
    // for ticketId in 0..correctTickets.len() {
    //     let ticket = &tickets[ticketId];
    //     println!("{}",ticket[0]);
    //     println!("");
    // }
    println!("{}", sum);
    let mut possibleMats = Vec::new();
    let length = correctTickets[0].len();
    println!("{}", length);
    possibleMats.resize(length, {
        let mut v = Vec::new();
        v.resize(length, true);
        v
    });
    for ticketId in 0..correctTickets.len() {
        let ticket = &correctTickets[ticketId];
        for (index, &val) in ticket.iter().enumerate() {
            for (indexPair, pairs) in newArr.iter().enumerate() {
                let correct = pairs
                    .iter()
                    .fold(false, |acc, &(a, b)| acc || ((a <= val) && (val <= b)));
                if (!correct) {
                    possibleMats[indexPair][index] = false;
                }
            }
        }
    }
    for line in 0..possibleMats.len() {
        let line = &possibleMats[line];
        for v in line {
            print!("{},", *v as isize);
        }
        println!("");
    }
    println!("");

    loop {
        let mut allOnes = true;
        for lineIndex in 0..possibleMats.len() {
            for line in 0..possibleMats.len() {
                let line = &possibleMats[line];
                for v in line {
                    print!("{},", v);
                }
                println!("");
            }
            println!("");

            let (count, position) = {
                let line = &possibleMats[lineIndex];
                let count = line.iter().filter(|&v| *v).count();
                let position = line.iter().position(|p| *p);
                (count, position)
            };
            if (count == 1) {
                let position = position.unwrap();
                for lineIndex2 in 0..possibleMats.len() {
                    if (lineIndex2 != lineIndex) {
                        possibleMats[lineIndex2][position] = false;
                    }
                }
            }
            if (count == 0) {
                panic!();
            }
            allOnes = allOnes & (count == 1);
        }
        if allOnes {
            break;
        }
        for lineIndex in 0..possibleMats.len() {
            // for line in 0..possibleMats.len() {
            //     let line: &Vec<bool> = &possibleMats.iter().map(|v| v[line]).collect();
            //     for v in line {
            //         print!("{},", v);
            //     }
            //     println!("");
            // }
            // println!("");

            let (count, position) = {
                let line: &Vec<bool> = &possibleMats.iter().map(|v| v[lineIndex]).collect();
                // println!("{} {} {} ",line[0], line[1],line[2]);

                let count = line.iter().filter(|&v| *v).count();
                let position = line.iter().position(|p| *p);
                (count, position)
            };
            if (count == 1) {
                let position = position.unwrap();
                // println!("{}, lineIndex {} ", position, lineIndex);
                for lineIndex2 in 0..possibleMats.len() {
                    if lineIndex2 != lineIndex {
                        possibleMats[position][lineIndex2] = false;
                    }
                }
            }
            if (count == 0) {
                panic!();
            }

            allOnes = allOnes & (count == 1);
        }
        if allOnes {
            break;
        }
    }
    let myTicket = input
        .split_terminator("your ticket:")
        .nth(1)
        .unwrap()
        .lines()
        .nth(1).unwrap()
        .split_terminator(",")
        .map(|v| v.parse::<usize>().unwrap())
        .collect::<Vec<usize>>();
    let mul = possibleMats
        .iter()
        .take(6)
        .map(|v| v.iter().position(|s| *s))
        .fold(1usize, |acc, s| acc * myTicket[s.unwrap()]);
    println!("{}", mul);
}
