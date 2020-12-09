use itertools::Itertools;

fn input_generator(input: &str) -> &[i32] {
    input.lines().flat_map(|s| s.parse::<i32>()).collect()
}

pub fn solve_part1(input: &str) -> Option<i32> {
    let input = input_generator(input);

    for v in input.iter().permutations(2) {
        if v[0] + v[1] == 2020 {
            return Some(v[0] * v[1]);
        }
    }

    None
}

pub fn solve_part2(input: &str) -> Option<i32> {
    let input = input_generator(input);

    for v in input.iter().permutations(3) {
        if v[0] + v[1] + v[2] == 2020 {
            return Some(v[0] * v[1] * v[2]);
        }
    }

    None
}
