use define_main::define_main;
#[define_main(7)]
fn a() {}

use std::collections::HashMap;

use itertools::Itertools;

type Color = String;
type Rules = HashMap<Color, Vec<(usize, Color)>>;

fn parse_rules(input: &str) -> Rules {
    input
        .lines()
        .filter_map(|line| {
            let color = line.split_ascii_whitespace().take(2).join(" ");

            let contains: Vec<_> = line
                .split(" bags contain ")
                .nth(1)
                .expect("Couldn't find \" bags contain \"")
                .split(", ")
                .filter_map(|desc| {
                    if desc == "no other bags." {
                        return None;
                    }

                    let count = desc
                        .split(' ')
                        .next()
                        .expect("No count")
                        .parse::<usize>()
                        .expect("Couldn't parse count");
                    let color = desc.split(' ').skip(1).take(2).join(" ");
                    Some((count, color))
                })
                .collect();

            Some((color.to_string(), contains))
        })
        .collect()
}

fn can_contain_shiny_gold(color: &str, rules: &Rules) -> bool {
    if color == "shiny gold" {
        return true;
    }

    for inside in rules[color].iter() {
        if can_contain_shiny_gold(&inside.1, rules) {
            return true;
        }
    }

    return false;
}

pub fn solve_part1(input: &str) -> usize {
    let rules = parse_rules(&input);
    rules
        .keys()
        .filter(|color| *color != "shiny gold" && can_contain_shiny_gold(*color, &rules))
        .count()
}

fn count_total_bags(color: &str, rules: &Rules) -> usize {
    rules[color]
        .iter()
        .map(|(count, color)| count * count_total_bags(color, rules))
        .sum::<usize>()
        + 1
}

pub fn solve_part2(input: &str) -> usize {
    let rules = parse_rules(&input);
    count_total_bags("shiny gold", &rules) - 1
}
