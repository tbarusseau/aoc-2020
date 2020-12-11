pub fn solve_part1(input: &str) -> usize {
    let s = solve(input, false);

    s.lines()
        .map(|l| l.chars().filter(|&c| c == '#').count())
        .sum()
}

pub fn solve_part2(input: &str) -> usize {
    let s = solve(input, true);

    s.lines()
        .map(|l| l.chars().filter(|&c| c == '#').count())
        .sum()
}

fn get_next_in_direction(
    input: &str,
    size: (usize, usize),
    pos: (isize, isize),
    direction: (isize, isize),
    remote_check: bool,
) -> Option<char> {
    let (posx, posy) = pos;
    let (directionx, directiony) = direction;

    let next = (posx.checked_add(directionx), posy.checked_add(directiony));

    match next {
        (Some(x), Some(y)) => {
            if x < 0 || y < 0 {
                return None;
            }

            let x = x as usize;
            let y = y as usize;

            if x >= size.0 || y >= size.1 {
                return None;
            }

            match input.lines().nth(y).unwrap().chars().nth(x).unwrap() {
                '.' => {
                    if remote_check {
                        return get_next_in_direction(
                            input,
                            size,
                            (x as isize, y as isize),
                            direction,
                            remote_check,
                        );
                    } else {
                        return Some('.');
                    }
                }
                seat @ 'L' | seat @ '#' => return Some(seat),
                seat @ _ => panic!("Invalid seat: {}", seat),
            }
        }
        _ => return None,
    }

    return None;
}

fn get_neighbours(
    input: &str,
    pos: (usize, usize),
    remote_check: bool,
    size: (usize, usize),
) -> usize {
    let mut neighbours = 0;
    let (pos_x, pos_y) = pos;

    let directions = [
        (-1, -1),
        (-1, 0),
        (-1, 1),
        (0, -1),
        (0, 1),
        (1, -1),
        (1, 0),
        (1, 1),
    ];

    directions.iter().for_each(|(x, y)| {
        if let Some(c) = get_next_in_direction(
            input,
            size,
            (pos_x as isize, pos_y as isize),
            (*x, *y),
            remote_check,
        ) {
            if c == '#' {
                neighbours += 1;
            }
        }
    });

    neighbours
}

fn get_updated_seat(
    input: &str,
    x: usize,
    y: usize,
    remote_check: bool,
    size: (usize, usize),
) -> char {
    match input.lines().nth(y) {
        Some(l) => {
            let seat = l
                .chars()
                .nth(x)
                .unwrap_or_else(|| panic!("Invalid column index: {}", x));
            if seat != 'L' && seat != '#' && seat != '.' {
                panic!("Invalid seat: {}", seat);
            }

            let neighbours = get_neighbours(input, (x, y), remote_check, size);

            match seat {
                '#' => {
                    let check = match remote_check {
                        true => 5,
                        false => 4,
                    };

                    if neighbours >= check {
                        'L'
                    } else {
                        '#'
                    }
                }
                'L' => {
                    if neighbours == 0 {
                        '#'
                    } else {
                        'L'
                    }
                }
                '.' => '.',
                _ => panic!("Invalid seat: {}", seat),
            }
        }
        None => panic!("Invalid row index: {}", y),
    }
}

fn step(input: &str, remote_check: bool) -> String {
    let height = input.lines().count();
    let width = input
        .lines()
        .next()
        .expect("No lines to step on")
        .chars()
        .count();

    let mut s = String::new();

    for y in 0..height {
        for x in 0..width {
            s.push(get_updated_seat(input, x, y, remote_check, (width, height)));
        }

        s.push('\n');
    }

    s.trim_end().to_owned()
}

fn solve(input: &str, remote_check: bool) -> String {
    let mut change = false;

    let mut s = step(input, remote_check);
    loop {
        let last = s.clone();
        s = step(&last, remote_check);

        if s == last {
            break;
        }
    }

    s
}

#[test]
pub fn test_step() {
    assert_eq!(
        step(
            "L.LL.LL.LL
LLLLLLL.LL
L.L.L..L..
LLLL.LL.LL
L.LL.LL.LL
L.LLLLL.LL
..L.L.....
LLLLLLLLLL
L.LLLLLL.L
L.LLLLL.LL",
            false
        ),
        "#.##.##.##
#######.##
#.#.#..#..
####.##.##
#.##.##.##
#.#####.##
..#.#.....
##########
#.######.#
#.#####.##"
            .to_string()
    );

    assert_eq!(
        step(
            "#.##.##.##
#######.##
#.#.#..#..
####.##.##
#.##.##.##
#.#####.##
..#.#.....
##########
#.######.#
#.#####.##",
            false
        ),
        "#.LL.L#.##
#LLLLLL.L#
L.L.L..L..
#LLL.LL.L#
#.LL.LL.LL
#.LLLL#.##
..L.L.....
#LLLLLLLL#
#.LLLLLL.L
#.#LLLL.##"
    );

    assert_eq!(
        step(
            "#.LL.L#.##
#LLLLLL.L#
L.L.L..L..
#LLL.LL.L#
#.LL.LL.LL
#.LLLL#.##
..L.L.....
#LLLLLLLL#
#.LLLLLL.L
#.#LLLL.##",
            false
        ),
        "#.##.L#.##
#L###LL.L#
L.#.#..#..
#L##.##.L#
#.##.LL.LL
#.###L#.##
..#.#.....
#L######L#
#.LL###L.L
#.#L###.##"
    );

    assert_eq!(
        step(
            "#.##.L#.##
#L###LL.L#
L.#.#..#..
#L##.##.L#
#.##.LL.LL
#.###L#.##
..#.#.....
#L######L#
#.LL###L.L
#.#L###.##",
            false
        ),
        "#.#L.L#.##
#LLL#LL.L#
L.L.L..#..
#LLL.##.L#
#.LL.LL.LL
#.LL#L#.##
..L.L.....
#L#LLLL#L#
#.LLLLLL.L
#.#L#L#.##"
    );

    assert_eq!(
        step(
            "#.#L.L#.##
#LLL#LL.L#
L.L.L..#..
#LLL.##.L#
#.LL.LL.LL
#.LL#L#.##
..L.L.....
#L#LLLL#L#
#.LLLLLL.L
#.#L#L#.##",
            false
        ),
        "#.#L.L#.##
#LLL#LL.L#
L.#.L..#..
#L##.##.L#
#.#L.LL.LL
#.#L#L#.##
..L.L.....
#L#L##L#L#
#.LLLLLL.L
#.#L#L#.##"
    );
}

#[test]
pub fn test_solve() {
    assert_eq!(
        solve(
            "L.LL.LL.LL
LLLLLLL.LL
L.L.L..L..
LLLL.LL.LL
L.LL.LL.LL
L.LLLLL.LL
..L.L.....
LLLLLLLLLL
L.LLLLLL.L
L.LLLLL.LL",
            false
        ),
        "#.#L.L#.##
#LLL#LL.L#
L.#.L..#..
#L##.##.L#
#.#L.LL.LL
#.#L#L#.##
..L.L.....
#L#L##L#L#
#.LLLLLL.L
#.#L#L#.##"
    );
}

#[test]
pub fn test_step_no_proximity() {
    assert_eq!(
        step(
            "L.LL.LL.LL
LLLLLLL.LL
L.L.L..L..
LLLL.LL.LL
L.LL.LL.LL
L.LLLLL.LL
..L.L.....
LLLLLLLLLL
L.LLLLLL.L
L.LLLLL.LL",
            true
        ),
        "#.##.##.##
#######.##
#.#.#..#..
####.##.##
#.##.##.##
#.#####.##
..#.#.....
##########
#.######.#
#.#####.##"
    );
    assert_eq!(
        step(
            "#.##.##.##
#######.##
#.#.#..#..
####.##.##
#.##.##.##
#.#####.##
..#.#.....
##########
#.######.#
#.#####.##",
            true
        ),
        "#.LL.LL.L#
#LLLLLL.LL
L.L.L..L..
LLLL.LL.LL
L.LL.LL.LL
L.LLLLL.LL
..L.L.....
LLLLLLLLL#
#.LLLLLL.L
#.LLLLL.L#"
    );
    assert_eq!(
        step(
            "#.LL.LL.L#
#LLLLLL.LL
L.L.L..L..
LLLL.LL.LL
L.LL.LL.LL
L.LLLLL.LL
..L.L.....
LLLLLLLLL#
#.LLLLLL.L
#.LLLLL.L#",
            true
        ),
        "#.L#.##.L#
#L#####.LL
L.#.#..#..
##L#.##.##
#.##.#L.##
#.#####.#L
..#.#.....
LLL####LL#
#.L#####.L
#.L####.L#"
    );
    assert_eq!(
        step(
            "#.L#.##.L#
#L#####.LL
L.#.#..#..
##L#.##.##
#.##.#L.##
#.#####.#L
..#.#.....
LLL####LL#
#.L#####.L
#.L####.L#",
            true
        ),
        "#.L#.L#.L#
#LLLLLL.LL
L.L.L..#..
##LL.LL.L#
L.LL.LL.L#
#.LLLLL.LL
..L.L.....
LLLLLLLLL#
#.LLLLL#.L
#.L#LL#.L#"
    );
    assert_eq!(
        step(
            "#.L#.L#.L#
#LLLLLL.LL
L.L.L..#..
##LL.LL.L#
L.LL.LL.L#
#.LLLLL.LL
..L.L.....
LLLLLLLLL#
#.LLLLL#.L
#.L#LL#.L#",
            true
        ),
        "#.L#.L#.L#
#LLLLLL.LL
L.L.L..#..
##L#.#L.L#
L.L#.#L.L#
#.L####.LL
..#.#.....
LLL###LLL#
#.LLLLL#.L
#.L#LL#.L#"
    );
    assert_eq!(
        step(
            "#.L#.L#.L#
#LLLLLL.LL
L.L.L..#..
##L#.#L.L#
L.L#.#L.L#
#.L####.LL
..#.#.....
LLL###LLL#
#.LLLLL#.L
#.L#LL#.L#",
            true
        ),
        "#.L#.L#.L#
#LLLLLL.LL
L.L.L..#..
##L#.#L.L#
L.L#.LL.L#
#.LLLL#.LL
..#.L.....
LLL###LLL#
#.LLLLL#.L
#.L#LL#.L#"
    );
}

#[test]
pub fn test_solve_no_proximity() {
    assert_eq!(
        solve(
            "L.LL.LL.LL
LLLLLLL.LL
L.L.L..L..
LLLL.LL.LL
L.LL.LL.LL
L.LLLLL.LL
..L.L.....
LLLLLLLLLL
L.LLLLLL.L
L.LLLLL.LL",
            true
        ),
        "#.L#.L#.L#
#LLLLLL.LL
L.L.L..#..
##L#.#L.L#
L.L#.LL.L#
#.LLLL#.LL
..#.L.....
LLL###LLL#
#.LLLLL#.L
#.L#LL#.L#"
    );
}
