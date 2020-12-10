#![allow(unused)]
#![allow(dead_code)]
#![allow(unused_imports)]

mod days;
use days::*;

macro_rules! aoc_import {
    ($n:literal) => {
        seq_macro::seq! {N in 1..=$n { paste::paste!{
            use day#N::{solve_part1 as [<day#N _solve_part1>], solve_part2 as [<day#N _solve_part2>]};
        }}}
    };
}

use chrono::{Datelike, Local};
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(name = "AoC 2020")]
struct Opt {
    #[structopt(short, long)]
    all: bool,
    #[structopt(short, long)]
    days: Option<Vec<u32>>,
}

fn main() {
    let mut opt = Opt::from_args();
    let today = Local::now().day();

    aoc_import!(10);

    if opt.all {
        opt.days = Some((1..=today).collect());
    } else if opt.days.is_none() {
        opt.days = Some(vec![today]);
    }

    // macro_rules! aoc_match {
    //     ($n:literal) => {
    //         opt.days.unwrap().iter().for_each(|d| match d {
    //             seq_macro::seq! {N in 1..=$n { paste::paste!{
    //                 #N => {
    //                     [<day#N _solve_part1()>;]
    //                     [<day#N _solve_part2()>;]
    //                 },
    //             }}}
    //             i @ _ => panic!("Unavailable day: {}", i),
    //         });
    //     };
    // }

    // aoc_match!(10)

    opt.days.unwrap().iter().for_each(|d| match d {
        1 => {
            let input = std::fs::read_to_string(&format!("./input/2020/day{}.txt", 1))
                .expect("Input file not found");
            println!("Day 1, part 1: {:?}", day1_solve_part1(&input));
            println!("Day 1, part 2: {:?}", day1_solve_part2(&input));
        }
        10 => {
            let input = std::fs::read_to_string(&format!("./input/2020/day{}.txt", 10))
                .expect("Input file not found");
            println!("Day 10, part 1: {:?}", day10_solve_part1(&input));
            println!("Day 10, part 2: {:?}", day10_solve_part2(&input));
        }
        i @ _ => panic!("Unavailable day: {}", i),
    });
}
