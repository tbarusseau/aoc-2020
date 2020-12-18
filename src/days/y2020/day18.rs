use std::collections::VecDeque;

fn get_precedence(c: char, part2: bool) -> usize {
    if part2 {
        match c {
            '+' => 2,
            '*' => 1,
            _ => 0,
        }
    } else {
        match c {
            '+' | '*' => 1,
            _ => 0,
        }
    }
}

fn perform_op(n1: usize, op: char, n2: usize) -> usize {
    match op {
        '+' => n1 + n2,
        '*' => n1 * n2,
        c => panic!("Invalid operator: {}", c),
    }
}

fn compute(l: &str, part2: bool) -> usize {
    let l = l.replace(' ', "");
    let mut values: VecDeque<usize> = VecDeque::new();
    let mut operators: VecDeque<char> = VecDeque::new();

    macro_rules! do_op {
        ($values: expr, $operators: expr) => {
            let n2 = $values.pop_back().unwrap();
            let n1 = $values.pop_back().unwrap();
            let op = $operators.pop_back().unwrap();
            $values.push_back(perform_op(n1, op, n2));
        };
    }

    l.chars().for_each(|c| match c {
        '(' => {
            operators.push_back('(');
        }
        ')' => {
            while operators.len() != 0 && operators[operators.len() - 1] != '(' {
                do_op!(values, operators);
            }

            operators.pop_back();
        }
        c @ '0'..='9' => {
            values.push_back(c.to_digit(10).unwrap() as usize);
        }
        c @ '+' | c @ '*' => {
            while operators.len() != 0
                && (get_precedence(operators[operators.len() - 1], part2)
                    >= get_precedence(c, part2))
            {
                do_op!(values, operators);
            }

            operators.push_back(c);
        }
        c => panic!("Invalid char: {}", c),
    });

    while operators.len() != 0 {
        do_op!(values, operators);
    }

    values.pop_back().unwrap()
}

pub fn solve_part1(input: &str) -> usize {
    input.lines().fold(0, |acc, l| acc + compute(l, false))
}

pub fn solve_part2(input: &str) -> usize {
    input.lines().fold(0, |acc, l| acc + compute(l, true))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn test1() {
        assert_eq!(solve_part1("1 + 2 * 3 + 4 * 5 + 6"), 71);
        assert_eq!(solve_part1("(1 + 2)"), 3);
        assert_eq!(solve_part1("(1 + 2) + (1 + 2)"), 6);
        assert_eq!(solve_part1("(1 + (2 + 3))"), 6);
        assert_eq!(solve_part1("(2 * (2 + 3))"), 10);
        assert_eq!(solve_part1("1 + (2 * 3) + (4 * (5 + 6))"), 51);
        assert_eq!(solve_part1("2 * 3 + (4 * 5)"), 26);
        assert_eq!(solve_part1("5 + (8 * 3 + 9 + 3 * 4 * 3)"), 437);
        assert_eq!(
            solve_part1("5 * 9 * (7 * 3 * 3 + 9 * 3 + (8 + 6 * 4))"),
            12240
        );
        assert_eq!(
            solve_part1("((2 + 4 * 9) * (6 + 9 * 8 + 6) + 6) + 2 + 4 * 2"),
            13632
        );
    }

    #[test]
    pub fn test2() {
        assert_eq!(solve_part2("1 + (2 * 3) + (4 * (5 + 6))"), 51);
        assert_eq!(solve_part2("2 * 3 + (4 * 5)"), 46);
        assert_eq!(solve_part2("5 + (8 * 3 + 9 + 3 * 4 * 3)"), 1445);
        assert_eq!(
            solve_part2("5 * 9 * (7 * 3 * 3 + 9 * 3 + (8 + 6 * 4))"),
            669060
        );
        assert_eq!(
            solve_part2("((2 + 4 * 9) * (6 + 9 * 8 + 6) + 6) + 2 + 4 * 2"),
            23340
        );
    }
}
