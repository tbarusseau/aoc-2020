use crate::utils::crt::chinese_remainder;

fn generate_input(input: &str) -> (i64, Vec<(i64, i64)>) {
    let mut lines = input.lines();
    let starting_time = lines
        .next()
        .expect("Missing timestamp line")
        .parse::<i64>()
        .expect("Invalid timestamp");
    let buses = lines
        .next()
        .expect("Missing buses line")
        .split(',')
        .enumerate()
        .flat_map(|(i, w)| w.parse::<i64>().map(|v| (i as i64, v)))
        .collect::<Vec<(i64, i64)>>();

    (starting_time, buses)
}

pub fn solve_part1(input: &str) -> i64 {
    let (starting_time, buses) = generate_input(input);

    let mut departure_time = starting_time;
    let bus_id;

    'outer: loop {
        for bus in &buses {
            if departure_time % bus.1 == 0 {
                bus_id = bus.1;

                break 'outer;
            }
        }

        departure_time += 1;
    }

    (departure_time - starting_time) * bus_id
}

pub fn solve_part2(input: &str) -> Option<i64> {
    let (_, buses) = generate_input(input);

    let mods = buses.iter().map(|&(_, id)| id).collect::<Vec<i64>>();
    let remainders = buses
        .iter()
        .map(|&(index, id)| id - index)
        .collect::<Vec<i64>>();

    chinese_remainder(&remainders, &mods)
}

#[test]
pub fn test2() {
    assert_eq!(solve_part2("0\n17,x,13,19"), Some(3417));
    assert_eq!(solve_part2("0\n67,7,59,61"), Some(754018));
    assert_eq!(solve_part2("0\n67,x,7,59,61"), Some(779210));
    assert_eq!(solve_part2("0\n67,7,x,59,61"), Some(1261476));
    assert_eq!(solve_part2("0\n1789,37,47,1889"), Some(1202161486));
}

#[test]
pub fn test1() {
    assert_eq!(
        solve_part1(
            "939
7,13,x,x,59,x,31,19"
        ),
        295
    );
}
