#![feature(destructuring_assignment)]

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

#[async_std::main]
async fn main() {
    let mut opt = Opt::from_args();
    let today = Local::now().day();

    aoc_import!(2020, 14);

    if opt.all {
        opt.days = Some((1..=today).collect());
    } else if opt.days.is_none() {
        opt.days = Some(vec![today]);
    }

    aoc_match!(opt, 2020, 14);
}
