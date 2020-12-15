use std::collections::HashMap;

fn run(input: &str, stop_index: usize) -> usize {
    let mut map: HashMap<usize, usize> = HashMap::new();
    let mut index: usize = 1;
    let mut last: usize;

    let input = input
        .split(',')
        .flat_map(str::parse::<usize>)
        .collect::<Vec<usize>>();
    input.iter().take(input.len() - 1).for_each(|n| {
        map.insert(*n, index);
        index += 1;
    });
    last = *input.last().unwrap();

    while index < stop_index {
        if map.contains_key(&last) {
            let new_last = index - map.get(&last).unwrap();
            map.insert(last, index);
            last = new_last;
        } else {
            map.insert(last, index);
            last = 0;
        }

        index += 1;
    }

    last
}

pub fn solve_part1(input: &str) -> usize {
    run(input, 2020)
}

pub fn solve_part2(input: &str) -> usize {
    run(input, 30000000)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn test_1() {
        assert_eq!(solve_part1("0,3,6"), 436);
        assert_eq!(solve_part1("1,3,2"), 1);
        assert_eq!(solve_part1("2,1,3"), 10);
    }
}
