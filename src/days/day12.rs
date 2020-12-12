fn manhattan(pos: (isize, isize)) -> usize {
    (pos.0.abs() + pos.1.abs()) as usize
}

#[test]
pub fn test_manhattan() {
    assert_eq!(manhattan((17, -8)), 25);
    assert_eq!(manhattan((214, 72)), 286);
}

fn update_direction(current_direction: (isize, isize), direction: char, degrees: isize) -> (isize, isize) {
    let modifier = match direction {
        'R' => (-1, 1),
        'L' => (1, -1),
        d @ _ => panic!("Impossible direction: {}", d),
    };

    if degrees % 90 != 0 {
        panic!("Degrees must be a multiple of 90");
    }

    let mut current_direction = current_direction;

    for _ in 0..(degrees / 90) {
        let tmp = current_direction;

        current_direction.0 = tmp.1 * modifier.0;
        current_direction.1 = tmp.0 * modifier.1;
    }

    current_direction
}

fn step(l: &str, position: (isize, isize), direction: (isize, isize), waypoint: bool) -> ((isize, isize), (isize, isize)) {
    let mut position = position;
    let mut direction = direction;

    let action = l.chars().nth(0).expect("No action");
    let value = l.chars().skip(1).collect::<String>().parse::<isize>().expect("Invalid value");
    
    match action {
        'N' => {
            match waypoint {
                false => position.1 -= value,
                true => direction.1 -= value,
            }
        },
        'S' => {
            match waypoint {
                false => position.1 += value,
                true => direction.1 += value,
            }
        },
        'E' => {
            match waypoint {
                false => position.0 += value,
                true => direction.0 += value,
            }
        },
        'W' => {
            match waypoint {
                false => position.0 -= value,
                true => direction.0 -= value,
            }
        },
        d @ 'L' | d @ 'R' => direction = update_direction(direction, d, value),
        'F' => {
            position.0 += direction.0 * value;
            position.1 += direction.1 * value;
        },
        i @ _ => panic!("Invalid instruction: {}", i),
    };

    (position, direction)
}

pub fn solve_part1(input: &str) -> usize {
    let mut position = (0, 0);
    let mut direction = (1, 0);

    input.lines().for_each(|l| {
        (position, direction) = step(l, position, direction, false);
    });

    manhattan(position)
}

pub fn solve_part2(input: &str) -> usize {
    let mut position = (0, 0);
    let mut waypoint = (10, -1);

    input.lines().for_each(|l| {
        (position, waypoint) = step(l, position, waypoint, true);
    });

    manhattan(position)
}

#[test]
pub fn test1() {
    assert_eq!(solve_part1("F10
N3
F7
R90
F11"), 25);
}

#[test]
pub fn test2() {
    assert_eq!(solve_part2("S2
W5
F20"), 120);

    assert_eq!(solve_part2("F10
N3
F7
R90
F11"), 286);

    assert_eq!(solve_part2("S2
W5
F20
E3
S5
R90
W5
F48
R180
E3
S3"), 832);

    assert_eq!(solve_part2("S2
W5
F20
E3
S5
R90
W5
F48
R180
E3
S3
E5
S3
F83
S1
W5
F81
W3
R90
F88"), 3496);
}
