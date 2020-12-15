struct Map {
    pub content: Vec<String>,
    pub width: usize,
    pub height: usize,
}

impl Map {
    pub fn from(input: &str) -> Map {
        let width = input.lines().next().expect("Couldn't get first line").len();
        let height = input.lines().count();
        let content = input.lines().map(|s| s.to_string()).collect();

        Map {
            content,
            width,
            height,
        }
    }

    pub fn get(&self, pos: (usize, usize)) -> Option<char> {
        let x = pos.0;
        let y = pos.1;

        if y >= self.height {
            None
        } else {
            let index = x % self.width;
            Some(
                self.content[y]
                    .chars()
                    .nth(index)
                    .unwrap_or_else(|| panic!("Couldn't access index {}", index)),
            )
        }
    }

    pub fn part1(&self, slope: (usize, usize)) -> usize {
        let mut pos: (usize, usize) = (slope.0, slope.1);
        let mut trees = 0;

        while let Some(c) = self.get(pos) {
            if c == '#' {
                trees += 1;
            }

            pos.0 += slope.0;
            pos.1 += slope.1;
        }

        trees
    }
}

pub fn solve_part1(input: &str) -> usize {
    let input = Map::from(input);

    input.part1((3, 1))
}

pub fn solve_part2(input: &str) -> usize {
    let input = Map::from(input);

    [
        input.part1((1, 1)),
        input.part1((3, 1)),
        input.part1((5, 1)),
        input.part1((7, 1)),
        input.part1((1, 2)),
    ]
    .iter()
    .product()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn test() {
        assert_eq!(
            solve_part1(
                "..##.......
#...#...#..
.#....#..#.
..#.#...#.#
.#...##..#.
..#.##.....
.#.#.#....#
.#........#
#.##...#...
#...##....#
.#..#...#.#"
            ),
            7
        );
        assert_eq!(
            solve_part2(
                "..##.......
#...#...#..
.#....#..#.
..#.#...#.#
.#...##..#.
..#.##.....
.#.#.#....#
.#........#
#.##...#...
#...##....#
.#..#...#.#"
            ),
            336
        );
    }
}
