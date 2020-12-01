use itertools::Itertools;

#[aoc_generator(day1)]
pub fn input_generator(input: &str) -> Vec<i32> {
    input.lines().flat_map(|s| s.parse::<i32>()).collect()
}

#[aoc(day1, part1)]
pub fn solve_part1(input: &[i32]) -> Option<i32> {
    for v in input.iter().permutations(2) {
        if v[0] + v[1] == 2020 {
            return Some(v[0] * v[1]);
        }
    }

    None
}

#[aoc(day1, part2)]
pub fn solve_part2(input: &[i32]) -> Option<i32> {
    for v in input.iter().permutations(3) {
        if v[0] + v[1] + v[2] == 2020 {
            return Some(v[0] * v[1] * v[2]);
        }
    }

    None
}
