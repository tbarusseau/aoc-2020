use std::collections::HashMap;

use regex::Regex;

#[derive(Debug)]
enum Instruction<'a> {
    Bitmask(&'a str),
    Memset(usize, usize),
}
use Instruction::*;

impl<'a> From<&'a str> for Instruction<'a> {
    fn from(value: &'a str) -> Self {
        let re = Regex::new(r"^.*\[(?P<address>\d+)\] = (?P<value>\d+)$").unwrap();
        if let Some(c) = re.captures(value) {
            Memset(
                c.name("address").unwrap().as_str().parse().unwrap(),
                c.name("value").unwrap().as_str().parse().unwrap(),
            )
        } else {
            Bitmask(value.split(" = ").nth(1).unwrap())
        }
    }
}

fn apply_bitmask(value: usize, bitmask: &str) -> usize {
    let mut result: usize = 0;
    let mut power: usize = 0;

    bitmask.chars().rev().for_each(|c| {
        match c {
            b @ '0' | b @ '1' => {
                result += (b.to_digit(10).unwrap() as usize) << power;
            }
            'X' => {
                result += ((value >> power) & 1) << power;
            }
            b => panic!("Invalid bitmask value: {}", b),
        }

        power += 1;
    });

    result
}

fn decode_memory_address(address: usize, bitmask: &str) -> Vec<usize> {
    let mut result: Vec<usize> = Vec::with_capacity(32);
    let mut power = 0_usize;

    // Apply the bitmask
    let mut value = address;
    bitmask.chars().rev().for_each(|c| {
        if let '1' = c {
            value |= 1 << power;
        }

        power += 1;
    });

    power = 0;
    result.push(value);

    bitmask.chars().rev().for_each(|c| {
        match c {
            'X' => {
                let mut extend = Vec::new();
                result.iter().for_each(|&v| {
                    extend.push(v | 1 << power);
                    extend.push(v & !(1 << power));
                });

                result = extend;
            }
            '0' | '1' => {}
            _ => panic!("Invalid bitmask value"),
        }

        power += 1;
    });

    result
}

pub fn solve_part1(input: &str) -> usize {
    let mut memory: HashMap<usize, usize> = HashMap::new();
    let mut bitmask = String::new();

    input.lines().map(Instruction::from).for_each(|i| match i {
        Bitmask(s) => {
            bitmask = String::from(s);
        }
        Memset(address, value) => {
            memory.insert(address, apply_bitmask(value, &bitmask));
        }
    });

    memory.iter().fold(0, |acc, (_, &v)| acc + v)
}

pub fn solve_part2(input: &str) -> usize {
    let mut memory: HashMap<usize, usize> = HashMap::new();
    let mut bitmask = String::new();

    input.lines().map(Instruction::from).for_each(|i| match i {
        Bitmask(s) => {
            bitmask = String::from(s);
        }
        Memset(address, value) => {
            decode_memory_address(address, &bitmask)
                .iter()
                .for_each(|&address| {
                    memory.insert(address, value);
                });
        }
    });

    memory.iter().fold(0, |acc, (_, &v)| acc + v)
}

#[test]
pub fn test_bitmasks() {
    assert_eq!(
        apply_bitmask(11, "XXXXXXXXXXXXXXXXXXXXXXXXXXXXX1XXXX0X"),
        73
    );
    assert_eq!(
        apply_bitmask(101, "XXXXXXXXXXXXXXXXXXXXXXXXXXXXX1XXXX0X"),
        101
    );
    assert_eq!(apply_bitmask(0, "XXXXXXXXXXXXXXXXXXXXXXXXXXXXX1XXXX0X"), 64);
}
