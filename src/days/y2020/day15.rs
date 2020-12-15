fn run_vec(input: &str, stop_index: usize) -> usize {
    let mut v = vec![0; stop_index];
    let mut index = 1_usize;
    let mut last: usize;

    let input = input
        .split(',')
        .flat_map(str::parse::<usize>)
        .collect::<Vec<usize>>();
    input.iter().take(input.len() - 1).for_each(|&n| {
        v[n] = index;
        index += 1;
    });
    last = *input.last().unwrap();

    while index < stop_index {
        last = match v[last] {
            0 => {
                v[last] = index;
                0
            }
            i => {
                v[last] = index;
                index - i
            }
        };

        index += 1;
    }

    last
}

pub fn solve_part1(input: &str) -> usize {
    run_vec(input, 2020)
}

pub fn solve_part2(input: &str) -> usize {
    run_vec(input, 30000000)
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
