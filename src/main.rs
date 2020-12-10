#![allow(unused)]
#![allow(dead_code)]
#![allow(unused_imports)]

mod days;

#[macro_use]
mod utils;

use days::*;

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

    aoc_match!(opt, 10, 2020);
}
