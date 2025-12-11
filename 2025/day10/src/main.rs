use std::collections::VecDeque;
use std::env;
use std::fs;
use std::io;

use microlp::ComparisonOp;
use microlp::LinearExpr;
use microlp::OptimizationDirection;
use microlp::Problem;

fn parse_line(line: &str) -> (u32, Vec<u32>, Vec<u8>) {
    let mut buttons = vec![];
    let mut target = 0;

    let mut it = line.chars();

    for (i, c) in it.by_ref().skip(1).enumerate() {
        match c {
            '#' => target |= 1 << i,
            '.' => (),
            _ => break,
        }
    }

    loop {
        match it.nth(1) {
            Some('{') => break,
            Some('(') => (),
            other => panic!("Unexpected character \"{other:?}\" in: {line}"),
        }

        let mut button = 0;

        while let Some(c) = it.next() {
            let d = c.to_digit(10).unwrap();
            button |= 1 << d;

            if let Some(')') = it.next() {
                break;
            }
        }

        buttons.push(button);
    }

    let rem = it.as_str().trim().trim_end_matches('}');

    let joltage = rem.split(',').map(|j| j.parse().unwrap()).collect();

    (target, buttons, joltage)
}

fn min_joltage(buttons: &[u32], joltage: &[u8]) -> i32 {
    let mut problem = Problem::new(OptimizationDirection::Minimize);
    let max = i32::from(*joltage.iter().max().unwrap_or(&0));

    let variables: Vec<_> = buttons
        .iter()
        .map(|_| problem.add_integer_var(1.0, (0, max)))
        .collect();

    for (bit, &value) in joltage.iter().enumerate() {
        let mut equation = LinearExpr::empty();

        for (&button, &variable) in buttons.iter().zip(&variables) {
            if button & (1 << bit) != 0 {
                equation.add(variable, 1.0);
            }
        }

        problem.add_constraint(equation, ComparisonOp::Eq, value.into());
    }

    problem.solve().unwrap().objective().round() as i32
}

fn minimum_clicks(target: u32, buttons: &[u32]) -> i32 {
    let max = buttons
        .iter()
        .map(|s| 32 - s.leading_zeros())
        .max()
        .unwrap_or(0);
    let possible = 2 << max;
    let mut seen = vec![false; possible];
    let mut todo = VecDeque::new();
    todo.push_back((0, 0));

    while let Some((steps, state)) = todo.pop_front() {
        for &button in buttons {
            let next = state ^ button;

            if next == target {
                return steps + 1;
            } else if !seen[next as usize] {
                seen[next as usize] = true;
                todo.push_back((steps + 1, next));
            }
        }
    }

    unreachable!("Did not find target");
}

fn solve(input: &str) -> (i32, i32) {
    let mut total_clicks = 0;
    let mut total_presses = 0;
    for line in input.trim().lines() {
        let (target, buttons, joltage) = parse_line(line);
        total_clicks += minimum_clicks(target, &buttons);
        total_presses += min_joltage(&buttons, &joltage)
    }

    (total_clicks, total_presses)
}

fn main() -> io::Result<()> {
    if let Some(path) = env::args_os().nth(1) {
        let input = fs::read_to_string(path)?;

        let (part1, part2) = solve(&input);
        println!("Part 1: {part1}\nPart 2: {part2}");
        Ok(())
    } else {
        eprintln!("Usage: {} INPUT_FILE", env::args().next().unwrap());
        std::process::exit(1);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &str = include_str!("../sample.txt");

    #[test]
    fn test_sample() {
        let (part1, part2) = solve(SAMPLE);
        assert_eq!(7, part1);
        assert_eq!(33, part2);
    }
}
