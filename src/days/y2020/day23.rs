fn generate_input(input: &str) -> Vec<usize> {
    input
        .chars()
        .flat_map(|c| c.to_digit(10))
        .map(|d| d as usize)
        .collect()
}

pub fn solve_part1(input: &str) -> String {
    let cups = generate_input(input);
    let mut current_cup = cups[0];
    let mut neighbours = get_neighbours(&cups);

    for _ in 0..100 {
        current_cup = step(&mut neighbours, current_cup);
    }

    // Concatenate labels into a string
    let mut out = String::new();
    let mut i = 1;

    loop {
        i = neighbours[i].1;

        if i == 1 {
            break;
        }

        out.push_str(&i.to_string());
    }

    out
}

fn get_neighbours(cups: &[usize]) -> Vec<(usize, usize)> {
    let mut neighbours = vec![(0, 0); cups.len() + 1];

    // Create a list containing (previous, next) for each cup (ghetto linked list :D)
    for e in cups.windows(3) {
        neighbours[e[1]] = (e[0], e[2]);
    }

    // Take care of first and last values
    neighbours[cups[0]] = (cups[cups.len() - 1], cups[1]);
    neighbours[cups[cups.len() - 1]] = (cups[cups.len() - 2], cups[0]);

    neighbours
}

fn step(neighbours: &mut Vec<(usize, usize)>, current_cup: usize) -> usize {
    // Pick up the three next cups
    let next1 = neighbours[current_cup].1;
    let next2 = neighbours[next1].1;
    let next3 = neighbours[next2].1;

    // Select initial value for destination cup
    let mut destination_cup = match current_cup {
        1 => neighbours.len() - 1,
        _ => current_cup - 1,
    };

    // Skip next three cups
    while vec![next1, next2, next3].contains(&destination_cup) {
        destination_cup = match destination_cup {
            1 => neighbours.len() - 1,
            _ => destination_cup - 1,
        };
    }

    // Perform the clockwise rotation
    let temp = neighbours[destination_cup].1;
    neighbours[current_cup].1 = neighbours[next3].1;
    neighbours[destination_cup].1 = next1;
    neighbours[next1] = (destination_cup, next2);
    neighbours[next2] = (next1, next3);
    neighbours[next3] = (next2, temp);
    neighbours[temp].0 = next3;

    // Return next cup
    neighbours[current_cup].1
}

pub fn solve_part2(input: &str) -> usize {
    let mut cups = generate_input(input);
    let max = cups.iter().copied().max().unwrap();
    for i in 1..=1_000_000 {
        cups.push(max + i);
    }

    let mut current_cup = cups[0];
    let mut neighbours = get_neighbours(&cups);

    for _ in 0..10_000_000 {
        current_cup = step(&mut neighbours, current_cup);
    }

    let r1 = neighbours[1].1;
    let r2 = neighbours[r1].1;

    r1 * r2
}
