use std::io::Read;

use crate::common::Lines;
use crate::Solution;
#[derive(Clone, Copy)]
enum Op {
    Addition,
    Multiplication,
}

fn extract_part(expression: &str) -> (&str, &str) {
    if expression.starts_with('(') {
        let mut brackets = 1;

        for (end, c) in expression.chars().enumerate().skip(1) {
            match c {
                '(' => brackets += 1,
                ')' => brackets -= 1,
                _ => {}
            }

            if brackets == 0 {
                return (
                    // Include the opening bracket for easier detection
                    &expression[..end],
                    expression.get((end + 2)..).unwrap_or(""),
                );
            }
        }

        panic!("Unmatched brackets: {}", expression);
    } else if let Some(pos) = expression.find(' ') {
        (
            &expression[..pos],
            expression.get((pos + 1)..).unwrap_or(""),
        )
    } else {
        (expression, "")
    }
}

fn compute_value(expression: &str) -> u64 {
    let mut remainder = expression;
    let mut value = 0;
    let mut op = None;

    while !remainder.is_empty() {
        let (part, rest) = extract_part(remainder);
        remainder = rest;

        let n = match part.chars().next().unwrap() {
            '+' => {
                op = Some(Op::Addition);
                continue;
            }
            '*' => {
                op = Some(Op::Multiplication);
                continue;
            }
            '(' => compute_value(&part[1..]),
            _ => part.parse().unwrap(),
        };

        match op {
            Some(Op::Addition) => value += n,
            Some(Op::Multiplication) => value *= n,
            None => value = n,
        }
    }

    value
}

fn compute_value2(expression: &str) -> u64 {
    let mut remainder = expression;
    let mut value = 0;
    let mut result = 1;
    let mut op = None;

    while !remainder.is_empty() {
        let (part, rest) = extract_part(remainder);
        remainder = rest;

        let n = match part.chars().next().unwrap() {
            '+' => {
                op = Some(Op::Addition);
                continue;
            }
            '*' => {
                op = Some(Op::Multiplication);
                continue;
            }
            '(' => compute_value2(&part[1..]),
            _ => part.parse().unwrap(),
        };

        match op {
            Some(Op::Addition) => value += n,
            Some(Op::Multiplication) => {
                result *= value;
                value = n;
            }
            None => value = n,
        }
    }

    result * value
}

#[derive(Default)]
pub struct Day18;

impl Solution for Day18 {
    fn part1(&mut self, input: &mut dyn Read) -> String {
        Lines::new(input)
            .map(|l| compute_value(&l))
            .sum::<u64>()
            .to_string()
    }

    fn part2(&mut self, input: &mut dyn Read) -> String {
        Lines::new(input)
            .map(|l| compute_value2(&l))
            .sum::<u64>()
            .to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample_part1() {
        assert_eq!(71, compute_value("1 + 2 * 3 + 4 * 5 + 6"));
        assert_eq!(51, compute_value("1 + (2 * 3) + (4 * (5 + 6))"));
        assert_eq!(26, compute_value("2 * 3 + (4 * 5)"));
        assert_eq!(437, compute_value("5 + (8 * 3 + 9 + 3 * 4 * 3)"));
        assert_eq!(
            12240,
            compute_value("5 * 9 * (7 * 3 * 3 + 9 * 3 + (8 + 6 * 4))")
        );
        assert_eq!(
            13632,
            compute_value("((2 + 4 * 9) * (6 + 9 * 8 + 6) + 6) + 2 + 4 * 2")
        );
    }

    #[test]
    fn sample_part2() {
        assert_eq!(231, compute_value2("1 + 2 * 3 + 4 * 5 + 6"));
        assert_eq!(51, compute_value2("1 + (2 * 3) + (4 * (5 + 6))"));
        assert_eq!(46, compute_value2("2 * 3 + (4 * 5)"));
        assert_eq!(1445, compute_value2("5 + (8 * 3 + 9 + 3 * 4 * 3)"));
        assert_eq!(
            669060,
            compute_value2("5 * 9 * (7 * 3 * 3 + 9 * 3 + (8 + 6 * 4))")
        );
        assert_eq!(
            23340,
            compute_value2("((2 + 4 * 9) * (6 + 9 * 8 + 6) + 6) + 2 + 4 * 2")
        );
    }
}
