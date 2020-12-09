use itertools::Itertools;
use std::collections::VecDeque;

#[aoc(day9, part1)]
pub fn solve_part1(input: &str) -> usize {
    let input = input
        .lines()
        .map(|l| l.parse::<usize>().expect("Invalid input number"));

    let mut v: VecDeque<usize> = input.clone().take(25).collect();
    let mut it = input.skip(25);

    loop {
        let n = it.next().expect("No input left");
        if !v.iter().permutations(2).any(|vec| vec[0] + vec[1] == n) {
            return n;
        }

        v.pop_front();
        v.push_back(n);
    }
}

#[aoc(day9, part2)]
pub fn solve_part2(input: &str) -> usize {
    let needle: usize = 507_622_668;

    let haystack = input
        .lines()
        .map(|l| l.parse::<usize>().expect("Invalid input number"));
    let mut index = 0;
    let mut sum = 0;

    loop {
        let set: Vec<usize> = haystack
            .clone()
            .skip(index)
            .take_while(|e| match e + sum <= needle {
                true => {
                    sum += e;
                    true
                }
                false => false,
            })
            .collect();

        if sum == needle {
            break set.iter().min().expect("No min") + set.iter().max().expect("No max");
        }

        if set.len() == 1 {
            println!("{:?}", set);
            break 0;
        }

        index += 1;
        sum = 0;
    }
}
