use std::io::Read;

use crate::common::read_single_input;
use crate::Solution;

type Answers = u32;

fn count(answers: Answers) -> u32 {
    answers.count_ones()
}

fn compute_answers(group: &str) -> Answers {
    group
        .chars()
        .filter(char::is_ascii_lowercase)
        .map(|q| 1 << (q as usize - 'a' as usize))
        .fold(0, |a, b| a | b)
}

fn count_answers_all(group: &str) -> u32 {
    let combined = group
        .split('\n')
        .map(compute_answers)
        .fold(0xffff_ffff, |a, b| a & b);

    count(combined)
}

#[derive(Default)]
pub struct Day06;

impl Solution for Day06 {
    fn part1(&mut self, input: &mut dyn Read) -> String {
        let input: String = read_single_input(input);

        let total: u32 = input.split("\n\n").map(|g| count(compute_answers(g))).sum();

        total.to_string()
    }

    fn part2(&mut self, input: &mut dyn Read) -> String {
        let input: String = read_single_input(input);

        let total: u32 = input.split("\n\n").map(count_answers_all).sum();

        total.to_string()
    }
}

#[cfg(test)]
mod tests {
    use crate::test_implementation;

    use super::*;

    const SAMPLE: &[u8] = include_bytes!("../samples/06.txt");

    #[test]
    fn sample_part1() {
        test_implementation!(Day06, 1, SAMPLE, 11);
    }

    #[test]
    fn sample_part2() {
        test_implementation!(Day06, 2, SAMPLE, 6);
    }
}
