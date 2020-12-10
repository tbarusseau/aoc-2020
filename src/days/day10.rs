fn clean_input(input: &str) -> Vec<usize> {
    let mut v = input
        .lines()
        .map(|l| l.parse::<usize>().expect("Invalid number"))
        .collect::<Vec<usize>>();

    // Insert built-in jolt adapter
    let max = *v.iter().max().expect("Couldn't find max value");
    v.push(max + 3);
    v.sort();

    v
}

pub fn solve_part1(input: &str) -> usize {
    let mut diff: (usize, usize, usize) = (0, 0, 0);
    let mut last: usize = 0;
    let v = clean_input(input);

    v.iter().for_each(|a| {
        match a - last {
            1 => diff.0 += 1,
            2 => diff.1 += 1,
            3 => diff.2 += 1,
            i @ _ => panic!(
                "Invalid joltage difference: {}, between {} and {}",
                i, a, last
            ),
        };

        last = *a;
    });

    diff.0 * diff.2
}

#[test]
pub fn test_1() {
    assert_eq!(
        solve_part1(
            "28
33
18
42
31
14
46
20
48
47
24
23
49
45
19
38
39
11
1
32
25
35
8
17
7
9
4
2
34
10
3"
        ),
        22 * 10
    );
}

pub fn solve_part2(input: &str) -> usize {
    use std::collections::HashMap;

    let mut diff: (usize, usize, usize) = (0, 0, 0);
    let mut last: usize = 0;
    let v = clean_input(input);
    let mut r: HashMap<usize, usize> = vec![(0, 1)].into_iter().collect();

    v.iter().for_each(|a| {
        r.insert(*a, 0);

        for i in 1..=3 {
            if a >= &i && r.contains_key(&(a - i)) {
                *r.get_mut(a).expect("Couldn't get hashmap value") += r[&(a - i)];
            }
        }

        last = *a;
    });

    r[&last]
}

#[test]
pub fn test1() {
    assert_eq!(
        solve_part2(
            "28
33
18
42
31
14
46
20
48
47
24
23
49
45
19
38
39
11
1
32
25
35
8
17
7
9
4
2
34
10
3"
        ),
        19208
    );
}
