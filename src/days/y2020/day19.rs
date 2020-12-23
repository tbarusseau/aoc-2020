// #[derive(Debug, PartialEq)]
// enum Entry {
//     Char(char),
//     Rule(Vec<Vec<usize>>),
// }
// use Entry::*;

// fn entry_from_line(l: &str) -> (usize, Entry) {
//     let mut s = l.split(": ");
//     let index = s.next().unwrap().parse::<usize>().unwrap();
//     let rest = s.next().unwrap();

//     if rest.starts_with('"') {
//         (index, Char(rest.chars().nth(1).unwrap()))
//     } else {
//         let mut ret = vec![];

//         rest.split(" | ").for_each(|rule| {
//             let mut v = vec![];

//             rule.split(' ').for_each(|element| {
//                 v.push(element.parse::<usize>().unwrap());
//             });

//             ret.push(v);
//         });

//         (index, Rule(ret))
//     }
// }

pub fn solve_part1(_input: &str) -> usize {
    // let mut s = input.split("\n\n");
    // let entries = s.next().unwrap();
    // let words = s.next().unwrap();

    // let v: Vec<Entry> = entries
    //     .lines()
    //     .map(entry_from_line)
    //     .collect::<Vec<(usize, Entry)>>()
    //     .sort_by(|(a, _), (b, _)| a.partial_cmp(b).unwrap());

    // let mut regex = "";

    0
}

pub fn solve_part2(_input: &str) -> usize {
    0
}

// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     pub fn test_entry_from_line() {
//         assert_eq!(entry_from_line(r#"72: "b""#), (72, Char('b')));
//         assert_eq!(
//             entry_from_line("50: 113 113"),
//             (50, Rule(vec![vec![113, 113]]))
//         );
//         assert_eq!(
//             entry_from_line("71: 72 106 | 52 128"),
//             (71, Rule(vec![vec![72, 106], vec![52, 128]]))
//         );
//     }
// }
