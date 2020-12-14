use std::{collections::HashMap, convert::TryFrom};

use regex::Regex;

#[derive(Debug)]
enum Instruction<'a> {
    Bitmask(&'a str),
    Memset(usize, usize),
}
use Instruction::*;

enum InstructionParsingError {
    Error,
}

impl<'a> TryFrom<&'a str> for Instruction<'a> {
    type Error = InstructionParsingError;

    fn try_from(value: &'a str) -> Result<Self, Self::Error> {
        let re = Regex::new(r"^.*\[(?P<address>\d+)\] = (?P<value>\d+)$").unwrap();
        if let Some(c) = re.captures(value) {
            Ok(Memset(
                c.name("address").unwrap().as_str().parse().unwrap(),
                c.name("value").unwrap().as_str().parse().unwrap(),
            ))
        } else {
            if let Some(s) = value.split(" = ").nth(1) {
                Ok(Bitmask(s))
            } else {
                Err(InstructionParsingError::Error)
            }
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
            b @ _ => panic!("Invalid bitmask value: {}", b),
        }

        power += 1;
    });

    result
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

pub fn solve_part1(input: &str) -> usize {
    let mut memory: HashMap<usize, usize> = HashMap::new();
    let mut bitmask = String::new();

    input
        .lines()
        .flat_map(|l| Instruction::try_from(l))
        .for_each(|i| match i {
            Bitmask(s) => {
                bitmask = unsafe { String::from_utf8_unchecked(s.bytes().collect()) };
            }
            Memset(address, value) => {
                memory.insert(address, apply_bitmask(value, &bitmask));
            }
        });

    memory.iter().fold(0, |acc, (_, &v)| acc + v)
}

// address: 000000000000000000000000000000101010  (decimal 42)
// mask:    000000000000000000000000000000X1001X
// result:  000000000000000000000000000000X1101X

//                                             X -> Add in vec: 0 << 0, 1 << 0
//                                                      Vec: 0, 1
//                                            1_ -> To all numbers in vec: += 1 << 1
//                                                      Vec: 0 + 2 = 2, 1 + 2 = 3
//                                           0__ -> Do nothing
//                                          0___ -> Do nothing
//                                         1____ -> To all numbers in vec: += 1 << 4
//                                                      Vec: 2 + 16 = 18, 3 + 16 = 19
//                                        X_____ -> For each number n in vec, add in vec: n + 1 << 5, n
//                                                      Vec: 18, 19, 50, 51

fn decode_memory_address(address: usize, bitmask: &str) -> Vec<usize> {
    let mut result: Vec<usize> = Vec::with_capacity(32);
    let mut current_value: usize = 0;
    let mut power = 0_usize;

    bitmask.chars().rev().for_each(|c| {
        match c {
            b @ '0' | b @ '1' => {
                result.iter_mut().for_each(|v| {
                    *v += (b.to_digit(10).unwrap() as usize) << power;
                });

                current_value += (b.to_digit(10).unwrap() as usize) << power;
            }
            'X' => {
                result.push(current_value + 1 << power);
                result.push(current_value);
            }
            _ => panic!("Invalid bitmask value"),
        }

        power += 1;
    });

    result
}

pub fn solve_part2(input: &str) -> usize {
    let mut memory: HashMap<usize, usize> = HashMap::new();
    let mut bitmask = String::new();

    input
        .lines()
        .flat_map(|l| Instruction::try_from(l))
        .for_each(|i| match i {
            Bitmask(s) => {
                bitmask = unsafe { String::from_utf8_unchecked(s.bytes().collect()) };
            }
            Memset(address, value) => {
                memory.insert(address, apply_bitmask(value, &bitmask));
            }
        });

    memory.iter().fold(0, |acc, (_, &v)| acc + v)
}
