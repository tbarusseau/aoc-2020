use lazy_static::lazy_static;
use regex::Regex;

#[derive(Debug)]
pub struct PasswordEntry {
    min: usize,
    max: usize,
    letter: char,
    password: String,
}

lazy_static! {
    static ref RE: Regex = Regex::new(r"^(\d+)-(\d+) (.): (.*)$").unwrap();
}

pub fn to_password_entry(input: &str) -> PasswordEntry {
    let capture = RE.captures(input).unwrap();

    let entry = PasswordEntry {
        min: capture[1].parse().expect("Couldn't parse min"),
        max: capture[2].parse().expect("Couldn't parse max"),
        letter: capture[3].chars().next().expect("Couldn't get letter"),
        password: String::from(&capture[4]),
    };

    return entry;
}

#[aoc_generator(day2)]
pub fn input_generator(input: &str) -> Vec<PasswordEntry> {
    input.lines().map(|l| to_password_entry(l)).collect()
}

#[aoc(day2, part1)]
pub fn solve_part1(input: &[PasswordEntry]) -> Option<i32> {
    let mut valid_passwords = 0;

    input.iter().for_each(|p| {
        let count = p.password.matches(p.letter).count();
        if count >= p.min && count <= p.max {
            valid_passwords += 1;
        }
    });

    Some(valid_passwords)
}

pub fn is_valid_pt2(p: &PasswordEntry) -> bool {
    let char_min = p
        .password
        .chars()
        .nth(p.min - 1)
        .expect("Couldn't get min-index");
    let char_max = p
        .password
        .chars()
        .nth(p.max - 1)
        .expect("Couldn't get max-index");

    return (char_min == p.letter && char_max != p.letter)
        || (char_min != p.letter && char_max == p.letter);
}

#[aoc(day2, part2)]
pub fn solve_part2(input: &[PasswordEntry]) -> Option<i32> {
    let mut valid_passwords = 0;

    input.iter().for_each(|p| {
        if is_valid_pt2(p) {
            valid_passwords += 1;
        }
    });

    Some(valid_passwords)
}

#[test]
pub fn test_part_2() {
    assert_eq!(is_valid_pt2(&to_password_entry("1-3 a: abcde")), true);
    assert_eq!(is_valid_pt2(&to_password_entry("1-3 b: cdefg")), false);
    assert_eq!(is_valid_pt2(&to_password_entry("2-9 c: ccccccccc")), false);
    assert_eq!(is_valid_pt2(&to_password_entry("5-7 s: bwkbdlwns")), false);
    assert_eq!(is_valid_pt2(&to_password_entry("5-6 v: vvvvvc")), true);
    assert_eq!(
        is_valid_pt2(&to_password_entry("4-10 f: fffffqsfsffffff")),
        false
    );
    assert_eq!(
        is_valid_pt2(&to_password_entry("15-16 w: wwwwwwpwwwwwwwww")),
        false
    );
    assert_eq!(is_valid_pt2(&to_password_entry("4-5 x: bsnxd")), true);
    assert_eq!(is_valid_pt2(&to_password_entry("2-3 v: svvnsnq")), false);
    assert_eq!(
        is_valid_pt2(&to_password_entry("13-16 z: zzzzxzzzzzzzczzpz")),
        false
    );
    assert_eq!(
        is_valid_pt2(&to_password_entry("6-12 t: tctvtvsbkfkzmlf")),
        false
    );
}
