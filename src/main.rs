use core::num;
// use core::panic;
use std::{collections::*, thread::current, cmp::Reverse};
// mod helpers;
// use helpers::*;
fn main() {
    let input = include_str!("input.txt");
    let mut map: Vec<u8> = input
        .chars()
        .map(|s| u8::from_str_radix(&s.to_string(), 16).unwrap())
        .collect();
    let num_bits = map.len() * 4;
    let data = parse_packet(map.as_slice(), 0, num_bits);
    println!("");
    println!("{}", data.version_number);
    println!("{}", data.version_sum);
    println!("{}", data.len);
    println!("{}", data.value);
}

struct ParsedPacket {
    value: usize,
    version_sum: usize,
    version_number: usize,
    len: usize
}

fn parse_packet(map: &[u8], starting_bit: usize, end_bit: usize) -> ParsedPacket{
    let mut version = get_number(map, starting_bit + 0..starting_bit + 3);
    let mut version_sum = version;
    let packet_type = get_number(map, starting_bit + 3..starting_bit + 6);
    let mut current_bit = starting_bit + 6;
    let mut value = 0;
    if packet_type == 4 {
        loop  {
            value = value * 16 + get_number(map, current_bit + 1.. current_bit + 5);            
            if get_bit(map, current_bit) == 0 {
                current_bit += 5;
                break;
            }
            current_bit += 5;
        }
    } else {
        value = match packet_type {
            0 => 0,
            1 => 1,
            2 => usize::max_value(),
            3 => usize::min_value(),
            5 => 0,
            6 => 0,
            7 => 0,
            _ => panic!(), 
        };
        let length_type = get_bit(map, starting_bit + 6);
        if length_type == 0 {
            let len = get_number(map, starting_bit + 7..starting_bit + 7 + 15);
            current_bit = starting_bit + 7 + 15;
            let mut index = 0;
            while current_bit - starting_bit - 7 - 15 != len {
                let parsed_packet = parse_packet(&map, current_bit, len + starting_bit);
                current_bit = parsed_packet.len + current_bit;
                value = update_value(packet_type, value, parsed_packet.value, index);
                version_sum += parsed_packet.version_sum;
                index += 1;
            }
        } else {
            let len = get_number(map, starting_bit + 7..starting_bit +7 + 11);
            current_bit = starting_bit + 7 + 11;
            for i in 0.. len {
                let parsed_packet = parse_packet(&map, current_bit, end_bit);
                current_bit = parsed_packet.len + current_bit;
                value = update_value(packet_type, value, parsed_packet.value, i);
                version_sum += parsed_packet.version_sum;
            }
        }
    }
    ParsedPacket {value, version_sum, version_number: version, len: current_bit - starting_bit }
}

fn get_bit(map: &[u8], bit: usize) -> u8 {
    (map[bit / 4] >> (3-(bit%4))) % 2
}

fn get_number(map: &[u8], bits: impl std::iter::Iterator<Item = usize>) -> usize {
    bits.fold(0,|acc, bit| acc * 2 + get_bit(map, bit) as usize)
}

fn update_value(packet_type: usize, value: usize, new_value: usize, index: usize) -> usize {
    match packet_type {
        0=> return value + new_value,
        1=> return value * new_value,
        2=> return value.min(new_value),
        3=> return value.max(new_value),
        5=> if index == 0 { return new_value} else {return (value > new_value) as usize}
        6=> if index == 0 { return new_value} else {return (value < new_value) as usize}
        7=> if index == 0 { return new_value} else {return (value == new_value) as usize}
        _ => panic!()
    }
}