use std::collections::{HashMap, HashSet};

pub fn solve_part1(input: &str) -> usize {
    let mut count = 0_usize;
    let mut m = HashSet::<char>::new();
    input.lines().for_each(|l| {
        if l.is_empty() {
            count += m.len();
            m.clear();
        } else {
            for c in l.chars() {
                m.insert(c);
            }
        }
    });

    count + m.len()
}

fn count_everyone_voted(m: &HashMap<char, usize>, lines: usize) -> usize {
    let mut count = 0;
    m.iter().for_each(|(_, v)| {
        if *v == lines {
            count += 1;
        }
    });

    count
}

pub fn solve_part2(input: &str) -> usize {
    let mut count = 0_usize;
    let mut lines = 0_usize;
    let mut m = HashMap::<char, usize>::new();

    input.lines().for_each(|l| {
        lines += 1;
        if l.is_empty() {
            count += count_everyone_voted(&m, lines - 1);
            lines = 0;
            m.clear();
        } else {
            for c in l.chars() {
                if m.contains_key(&c) {
                    m.entry(c).and_modify(|v| *v += 1);
                } else {
                    m.insert(c, 1);
                }
            }
        }
    });

    count + count_everyone_voted(&m, lines)
}
