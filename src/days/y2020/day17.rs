use itertools::iproduct;

const SIZE: isize = 24;
const ACTIVE: char = '#';
const INACTIVE: char = '.';

fn run_cycle(grid: Vec<Vec<Vec<char>>>) -> Vec<Vec<Vec<char>>> {
    let mut clone = grid.clone();

    for i in 0..SIZE {
        for j in 0..SIZE {
            for k in 0..SIZE {
                let mut active = 0;
                for t in iproduct!(-1..=1, -1..=1, -1..=1) {
                    let (x, y, z): (isize, isize, isize) = t;

                    // Ignore current case (0, 0, 0)
                    if x == 0 && y == 0 && z == 0 {
                        continue;
                    }

                    let x = i + x;
                    let y = j + y;
                    let z = k + z;

                    // println!("Checking in {}, {}, {}", x, y, z);

                    if x < 0 || x >= SIZE || y < 0 || y >= SIZE || z < 0 || z >= SIZE {
                        continue;
                    }

                    if grid[x as usize][y as usize][z as usize] == ACTIVE {
                        active += 1;
                    }
                }

                let i = i as usize;
                let j = j as usize;
                let k = k as usize;

                if grid[i][j][k] == ACTIVE && (active != 2 && active != 3) {
                    clone[i][j][k] = INACTIVE;
                }

                if grid[i][j][k] == INACTIVE && active == 3 {
                    clone[i][j][k] = ACTIVE;
                }
            }
        }
    }

    clone
}

fn parse_input(input: &str) -> Vec<Vec<Vec<char>>> {
    // Maximum bounds have been found by trial and error, mostly
    let mut grid = vec![vec![vec![INACTIVE; SIZE as usize]; SIZE as usize]; SIZE as usize];

    input.lines().enumerate().for_each(|(i, v)| {
        v.chars().enumerate().for_each(|(j, c)| {
            grid[SIZE as usize / 2][i + 9][j + 9] = c;
        });
    });

    grid
}

fn count_active(grid: &Vec<Vec<Vec<char>>>) -> usize {
    grid.iter().fold(0, |acc, e| {
        acc + e.iter().fold(0, |acc, e| {
            acc + e
                .iter()
                .fold(0, |acc, &e| if e == ACTIVE { acc + 1 } else { acc })
        })
    })
}

fn run_cycle4(grid: Vec<Vec<Vec<Vec<char>>>>) -> Vec<Vec<Vec<Vec<char>>>> {
    let mut clone = grid.clone();

    for i in 0..SIZE {
        for j in 0..SIZE {
            for k in 0..SIZE {
                for l in 0..SIZE {
                    let mut active = 0;
                    for t in iproduct!(-1..=1, -1..=1, -1..=1, -1..=1) {
                        let (x, y, z, w): (isize, isize, isize, isize) = t;

                        // Ignore current case (0, 0, 0)
                        if x == 0 && y == 0 && z == 0 && w == 0 {
                            continue;
                        }

                        let x = i + x;
                        let y = j + y;
                        let z = k + z;
                        let w = l + w;

                        if x < 0
                            || x >= SIZE
                            || y < 0
                            || y >= SIZE
                            || z < 0
                            || z >= SIZE
                            || w < 0
                            || w >= SIZE
                        {
                            continue;
                        }

                        if grid[x as usize][y as usize][z as usize][w as usize] == ACTIVE {
                            active += 1;
                        }
                    }

                    let i = i as usize;
                    let j = j as usize;
                    let k = k as usize;
                    let l = l as usize;

                    // FIXME
                    if grid[i][j][k][l] == ACTIVE && (active != 2 && active != 3) {
                        clone[i][j][k][l] = INACTIVE;
                    }

                    if grid[i][j][k][l] == INACTIVE && active == 3 {
                        clone[i][j][k][l] = ACTIVE;
                    }
                }
            }
        }
    }

    clone
}

fn parse_input4(input: &str) -> Vec<Vec<Vec<Vec<char>>>> {
    // Maximum bounds have been found by trial and error, mostly
    let mut grid = vec![
        vec![vec![vec![INACTIVE; SIZE as usize]; SIZE as usize]; SIZE as usize];
        SIZE as usize
    ];

    input.lines().enumerate().for_each(|(i, v)| {
        v.chars().enumerate().for_each(|(j, c)| {
            grid[SIZE as usize / 2][SIZE as usize / 2][i + 9][j + 9] = c;
        });
    });

    grid
}

fn count_active4(grid: &Vec<Vec<Vec<Vec<char>>>>) -> usize {
    grid.iter().fold(0, |acc, e| {
        acc + e.iter().fold(0, |acc, e| {
            acc + e.iter().fold(0, |acc, e| {
                acc + e
                    .iter()
                    .fold(0, |acc, &e| if e == ACTIVE { acc + 1 } else { acc })
            })
        })
    })
}

pub fn solve_part1(input: &str) -> usize {
    let mut grid = parse_input(input);

    for _ in 0..6 {
        grid = run_cycle(grid);
    }

    count_active(&grid)
}

pub fn solve_part2(input: &str) -> usize {
    let mut grid = parse_input4(input);

    for _ in 0..6 {
        grid = run_cycle4(grid);
    }

    count_active4(&grid)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn test1() {
        assert_eq!(
            solve_part1(
                ".#.
..#
###"
            ),
            112,
        );
    }

    #[test]
    pub fn test2() {
        assert_eq!(
            solve_part2(
                ".#.
..#
###"
            ),
            848,
        )
    }

    #[test]
    pub fn test_count_active() {
        assert_eq!(count_active(&vec![vec![vec![INACTIVE; 3]; 3]; 3]), 0);
        assert_eq!(count_active(&vec![vec![vec![ACTIVE; 3]; 3]; 3]), 27);
        assert_eq!(count_active(&vec![vec![vec![ACTIVE; 4]; 4]; 4]), 64);
    }
}
