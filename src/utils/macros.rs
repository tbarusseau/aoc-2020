/// Import all solutions from day `1` to `n`
macro_rules! aoc_import {
    ($day:literal) => {
        seq_macro::seq! {Day in 1..=$day { paste::paste!{
            use day#Day::{solve_part1 as [<day#Day _solve_part1>], solve_part2 as [<day#Day _solve_part2>]};
        }}}
    };
}

/// Generate a `match` statement that that checks `opt.days` from `1` to `n`
macro_rules! aoc_match {
    ($opt:expr,$day:literal,$year:literal) => {
        seq_macro::seq! {Day in 1..=$day { paste::paste!{

        use futures::stream::{FuturesUnordered, StreamExt};
        let mut futures = FuturesUnordered::new();

        $opt.days
            .unwrap_or_else(|| panic!("'opt.days' is None"))
            .iter()
            .for_each(|d| match d {
                #(Day => {
                    let path = format!("./input/{}/day{}.txt", $year, Day);
                    let input = match std::fs::read_to_string(&path) {
                        Ok(s) => s,
                        Err(_) => {
                            // Download the input file from adventofcode.com
                            use std::io::Write;
                            use std::fs::OpenOptions;

                            use curl::easy::Easy;

                            print!("Downloading input for day {}... ", Day);
                            let mut easy = Easy::new();
                            easy.url(&format!("https://adventofcode.com/{}/day/{}/input", $year, Day)).unwrap();
                            easy.write_function(|data| {
                                let mut file = OpenOptions::new()
                                    .write(true)
                                    .create(true)
                                    .append(true)
                                    .open(&format!("./input/{}/day{}.txt", $year, Day))
                                    .expect("Couldn't open output file");
                                file.write_all(data).expect("Couldn't write to file");
                                Ok(data.len())
                            }).unwrap();
                            println!("Done!");
                            easy.cookie("session=53616c7465645f5f24669579e985604dd998b25e1573ff317d30d1dedb5f72351600515acf3e32647867bfd35607acb1").expect("Couldn't set request cookie");
                            easy.perform().unwrap();

                            std::fs::read_to_string(&path).expect("Couldn't read downloaded input")
                        }
                    };

                    use async_std::task;

                    let s = input.clone();
                    futures.push(task::spawn(async move {
                        println!("Day {}, part 1: {:?}", Day, [<day#Day _solve_part1>](&s));
                    }));

                    let s = input.clone();
                    futures.push(task::spawn(async move {
                        println!("Day {}, part 2: {:?}", Day, [<day#Day _solve_part2>](&s));
                    }));
                },)*
                i => panic!("Unavailable day: {}", i),
            });
        }}}

        while let Some(()) = futures.next().await { }
    };
}
