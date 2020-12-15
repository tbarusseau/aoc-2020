use itertools::Itertools;

fn input_generator(input: &str) -> Vec<i32> {
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn test_generator() {
        assert_eq!(
            input_generator(
                "1
2
3"
            ),
            [1, 2, 3]
        );
    }

    #[test]
    pub fn test1() {
        assert_eq!(
            solve_part1(
                "1721
979
366
299
675
1456"
            ),
            Some(514579)
        );
    }

    #[test]
    pub fn test2() {
        assert_eq!(
            solve_part2(
                "1721
979
366
299
675
1456"
            ),
            Some(241861950)
        )
    }
}
