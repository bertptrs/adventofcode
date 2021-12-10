use std::io::Read;

use crate::common::LineIter;

macro_rules! check_matching {
    ($stack:ident, $total:ident, $match:literal, $score:literal) => {{
        if let Some($match) = $stack.pop() {
            continue;
        } else {
            $total += $score;
            break;
        }
    }};
    ($stack:ident, $match:literal) => {{
        if let Some($match) = $stack.pop() {
            continue;
        } else {
            $stack.clear();
            break;
        }
    }};
}

pub fn part1(input: &mut dyn Read) -> String {
    let mut reader = LineIter::new(input);

    let mut stack = Vec::new();
    let mut total = 0;

    while let Some(line) = reader.next() {
        stack.clear();

        for &c in line.as_bytes() {
            match c {
                b'(' | b'[' | b'{' | b'<' => stack.push(c),
                b')' => check_matching!(stack, total, b'(', 3),
                b']' => check_matching!(stack, total, b'[', 57),
                b'}' => check_matching!(stack, total, b'{', 1197),
                b'>' => check_matching!(stack, total, b'<', 25137),
                _ => panic!("Invalid bracket '{}'", char::from(c)),
            }
        }
    }

    total.to_string()
}

pub fn part2(input: &mut dyn Read) -> String {
    let mut reader = LineIter::new(input);

    let mut stack = Vec::new();
    let mut scores = Vec::new();

    while let Some(line) = reader.next() {
        for &c in line.as_bytes() {
            match c {
                b'(' | b'[' | b'{' | b'<' => stack.push(c),
                b')' => check_matching!(stack, b'('),
                b']' => check_matching!(stack, b'['),
                b'}' => check_matching!(stack, b'{'),
                b'>' => check_matching!(stack, b'<'),
                _ => panic!("Invalid bracket '{}'", char::from(c)),
            }
        }

        if !stack.is_empty() {
            let score = stack
                .drain(..)
                .rev()
                .map(|c| match c {
                    b'(' => 1,
                    b'[' => 2,
                    b'{' => 3,
                    b'<' => 4,
                    _ => 0,
                })
                .fold(0u64, |acc, s| 5 * acc + s);
            scores.push(score);
        }
    }

    scores.sort_unstable();
    scores[scores.len() / 2].to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::test_implementation;

    const SAMPLE: &[u8] = include_bytes!("samples/10.txt");

    #[test]
    fn sample_part1() {
        test_implementation(part1, SAMPLE, 26397);
    }

    #[test]
    fn sample_part2() {
        test_implementation(part2, SAMPLE, 288957);
    }
}
