pub struct Map {
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

        loop {
            match self.get(pos) {
                Some(c) => {
                    if c == '#' {
                        trees += 1;
                    }

                    pos.0 += slope.0;
                    pos.1 += slope.1;
                }
                None => break,
            }
        }

        trees
    }
}

#[aoc_generator(day3)]
pub fn input_generator(input: &str) -> Map {
    Map::from(input)
}

#[aoc(day3, part1)]
pub fn solve_part1(input: &Map) -> usize {
    input.part1((3, 1))
}

#[aoc(day3, part2)]
pub fn solve_part2(input: &Map) -> usize {
    [
        input.part1((1, 1)),
        input.part1((3, 1)),
        input.part1((5, 1)),
        input.part1((7, 1)),
        input.part1((1, 2)),
    ]
    .iter()
    .fold(1, |acc, x| acc * x)
}
