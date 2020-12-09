use chrono::{Datelike, Local};
use structopt::StructOpt;

use aoc_2020::solutions::*;

#[derive(Debug, StructOpt)]
#[structopt(name = "AoC 2020")]
struct Opt {
    #[structopt(short, long)]
    all: bool,
    #[structopt(short, long)]
    day: Option<Vec<u32>>,
}

fn main() {
    let mut opt = Opt::from_args();

    if !opt.all && opt.day == None {
        // Run today!
        opt.day = Some(vec![Local::now().day()]);
    }

    if opt.all {
        // Run all days!
    } else if let Some(days) = opt.day {
        // Run specified days!
        for day in days {
            println!("Running day{}:", day);
        }
    }
}
