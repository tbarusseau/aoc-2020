use regex::Regex;

#[derive(Debug)]
struct Pair {
    min: usize,
    max: usize,
}

impl Pair {
    fn is_inside(&self, value: usize) -> bool {
        self.min <= value && self.max >= value
    }
}

#[derive(Debug)]
struct Rule<'a> {
    name: &'a str,
    first: Pair,
    second: Pair,
}

impl<'a> Rule<'a> {
    fn value_is_valid(&self, value: usize) -> bool {
        self.first.is_inside(value) || self.second.is_inside(value)
    }
}

fn generate_input(input: &str) -> (Vec<Rule>, Vec<usize>, Vec<Vec<usize>>) {
    let mut rules: Vec<Rule> = vec![];
    let mut lines = input.lines();
    let mut my_ticket: Vec<usize> = vec![];
    let mut nearby_tickets: Vec<Vec<usize>> = vec![];

    let pairs_re =
        Regex::new(r"^(?P<name>.+): (?P<min1>\d+)-(?P<max1>\d+) or (?P<min2>\d+)-(?P<max2>\d+)")
            .unwrap();
    let mut step = 0;

    while let Some(l) = lines.next() {
        if l.is_empty() {
            step += 1;
            continue;
        }

        match step {
            0 => {
                // Match rules
                let caps = pairs_re.captures(l).expect("Line doesn't match pairs_re");
                rules.push(Rule {
                    name: caps.name("name").expect("No name found").as_str(),
                    first: Pair {
                        min: caps
                            .name("min1")
                            .expect("No min1 found")
                            .as_str()
                            .parse()
                            .expect("Invalid min1"),
                        max: caps
                            .name("max1")
                            .expect("No max1 found")
                            .as_str()
                            .parse()
                            .expect("Invalid max1"),
                    },
                    second: Pair {
                        min: caps
                            .name("min2")
                            .expect("No min2 found")
                            .as_str()
                            .parse()
                            .expect("Invalid min2"),
                        max: caps
                            .name("max2")
                            .expect("No max2 found")
                            .as_str()
                            .parse()
                            .expect("Invalid max2"),
                    },
                })
            }
            1 => {
                // Match my ticket
                if l == "your ticket:" {
                    continue;
                }

                my_ticket = l.split(',').flat_map(str::parse).collect();
            }
            2 => {
                // Match nearby tickets
                if l == "nearby tickets:" {
                    continue;
                }

                nearby_tickets.push(l.split(',').flat_map(str::parse).collect());
            }
            i => panic!("Invalid step: {}", i),
        }
    }

    (rules, my_ticket, nearby_tickets)
}

fn get_ticket_invalid_value(rules: &Vec<Rule>, ticket: &Vec<usize>) -> Option<usize> {
    for &value in ticket.iter() {
        if !rules.iter().any(|r| r.value_is_valid(value)) {
            return Some(value);
        }
    }

    None
}

fn get_value_invalid_rules<'a>(rules: &'a Vec<Rule>, value: usize) -> Vec<&'a str> {
    let mut r = vec![];

    for rule in rules.iter() {
        if !rule.value_is_valid(value) {
            r.push(rule.name);
        }
    }

    r
}

pub fn solve_part1(input: &str) -> usize {
    let (rules, _, nearby_tickets) = generate_input(input);
    nearby_tickets
        .iter()
        .flat_map(|t| get_ticket_invalid_value(&rules, t))
        .sum()
}

pub fn solve_part2(input: &str) -> usize {
    let (rules, my_ticket, nearby_tickets) = generate_input(input);

    // Algorithm:
    // 1. Only keep the valid tickets (using previous part), in `valid_tickets`.
    // 2. Create an array containing arrays of the rules' names, example:
    //  [
    //      ["seat", "row", "class"],
    //      ["seat", "row", "class"],
    //      ["seat", "row", "class"],
    //  ]
    //    With as much arrays as there are rules. This array is `possible_rules`.
    // 3. For each valid ticket, iterate over its values, from `0` to `len - 1`:
    //      a. The value's index is `i`
    //      b. Get all the rules broken by this value, i.e. all the rules in which this value cannot fit
    //      c. For each rule broken by this value, update `possible_rules[i]` by removing said rule
    // 4. After the previous step, you should have at least one index of `possible_rules`
    //    which only has a single possible rule.
    // 5. While there is any rule with more than one element, do the following:
    //      a. Store the one-rule elements in another temporary collection
    //      b. For each element in this collection, remove it from the more-than-one-rule elements
    //      c. Replace the array from 5. with a new one, which contains the one-rule elements, and the updated rest

    // Discard invalid tickets
    let valid_tickets: Vec<Vec<usize>> = nearby_tickets
        .into_iter()
        .filter(|t| get_ticket_invalid_value(&rules, &t).is_none())
        .collect();

    // Assemble `possible_rules`
    let rules_names: Vec<&str> = rules.iter().map(|r| r.name).collect();
    let mut possible_rules: Vec<Vec<&str>> = vec![rules_names.clone(); rules_names.iter().count()];

    // For each nearby ticket, eliminate whichever rules are impossible
    valid_tickets.iter().for_each(|t| {
        t.iter().enumerate().for_each(|(i, v)| {
            get_value_invalid_rules(&rules, *v).iter().for_each(|&r| {
                possible_rules[i].retain(|&v| v != r);
            });
        });
    });

    // Safety checks to ensure that the program doesn't go into an infinite loop :)
    if possible_rules.iter().any(|r| r.len() == 0) {
        panic!("One item in possible_rules has no candidate");
    }
    if possible_rules.iter().all(|r| r.len() != 1) {
        panic!("No item in possible_rules has a single candidate");
    }

    while possible_rules.iter().any(|l| l.len() > 1) {
        let confirmed_rules: Vec<&str> = possible_rules
            .iter()
            .filter(|l| l.len() == 1)
            .map(|l| l[0])
            .collect();

        possible_rules
            .iter_mut()
            .filter(|l| l.len() != 1)
            .for_each(|l| {
                l.retain(|e| !confirmed_rules.contains(e));
            });
    }

    let indexes: Vec<usize> = possible_rules
        .iter()
        .map(|v| v[0])
        .enumerate()
        .filter(|(_, v)| v.starts_with("departure"))
        .map(|(i, _)| i)
        .collect();

    my_ticket
        .iter()
        .enumerate()
        .filter(|(i, _)| indexes.contains(i))
        .map(|(_, v)| v)
        .product()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn solve1() {
        assert_eq!(
            solve_part1(
                "class: 1-3 or 5-7
row: 6-11 or 33-44
seat: 13-40 or 45-50

your ticket:
7,1,14

nearby tickets:
7,3,47
40,4,50
55,2,20
38,6,12"
            ),
            71
        );
    }

    // No full example was provided :'(
    #[test]
    pub fn solve2() {
        assert_eq!(
            solve_part2(
                "class: 0-1 or 4-19
row: 0-5 or 8-19
seat: 0-13 or 16-19

your ticket:
11,12,13

nearby tickets:
3,9,18
15,1,5
5,14,9"
            ),
            1,
        );
    }
}
