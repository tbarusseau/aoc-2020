use define_main::define_main;
#[define_main(5)]
fn a() {}

fn compute_row(input: &str) -> usize {
    let mut row: (usize, usize) = (0, 127);

    for i in 0..7 {
        match input
            .chars()
            .nth(i)
            .unwrap_or_else(|| panic!("Couldn't get index: {}", i))
        {
            'B' => row.0 += 2_usize.pow(6 - i as u32),
            'F' => row.1 -= 2_usize.pow(6 - i as u32),
            c @ _ => panic!("Invalid character: {}", c),
        }
    }

    row.0
}

#[test]
pub fn test_compute_row() {
    assert_eq!(compute_row("BFFFBBFRRR"), 70);
    assert_eq!(compute_row("FFFBBBFRRR"), 14);
    assert_eq!(compute_row("BBFFBBFRLL"), 102);
}

fn compute_column(input: &str) -> usize {
    let mut column: (usize, usize) = (0, 7);

    for i in 0..3 {
        match input
            .chars()
            .skip(7)
            .nth(i)
            .unwrap_or_else(|| panic!("Couldn't get index: {}", i))
        {
            'R' => column.0 += 2_usize.pow(2 - i as u32),
            'L' => column.1 -= 2_usize.pow(2 - i as u32),
            c @ _ => panic!("Invalid character: {}", c),
        }
    }

    column.0
}

#[test]
pub fn test_compute_column() {
    assert_eq!(compute_column("BFFFBBFRRR"), 7);
    assert_eq!(compute_column("FFFBBBFRRR"), 7);
    assert_eq!(compute_column("BBFFBBFRLL"), 4);
}

fn compute_seat_id(input: &str) -> usize {
    compute_row(input) * 8 + compute_column(input)
}

#[test]
pub fn test_compute_seat_id() {
    assert_eq!(compute_seat_id("BFFFBBFRRR"), 567);
    assert_eq!(compute_seat_id("FFFBBBFRRR"), 119);
    assert_eq!(compute_seat_id("BBFFBBFRLL"), 820);
}

pub fn solve_part1(input: &str) -> Option<usize> {
    input.lines().map(|s| compute_seat_id(s)).max()
}

pub fn solve_part2(input: &str) -> Option<usize> {
    let mut v: Vec<usize> = input.lines().map(|s| compute_seat_id(s)).collect();
    v.sort();

    let min = *v.get(0).expect("Couldn't get first vector element");
    for (index, v) in v.iter().enumerate() {
        if *v != index + min {
            return Some(*v - 1);
        }
    }

    None
}
