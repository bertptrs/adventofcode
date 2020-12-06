use std::io::Read;

use crate::common::read_single_input;
use crate::Solution;

type Answers = u32;

fn count(answers: Answers) -> u32 {
    answers.count_ones()
}

fn compute_answers(group: &str) -> Answers {
    let mut questions = 0;

    for question in group.chars().filter(char::is_ascii_lowercase) {
        questions |= 1 << (question as usize - 'a' as usize);
    }

    questions
}

fn count_answers_all(group: &str) -> u32 {
    let mut combined = 0xffff_ffff;

    for line in group.split('\n') {
        let single = compute_answers(line);

        combined &= single;
    }

    count(combined)
}

#[derive(Default)]
pub struct Day06;

impl Solution for Day06 {
    fn part1(&mut self, input: &mut dyn Read) -> String {
        let input: String = read_single_input(input);

        let total: u32 = input.split("\n\n").map(compute_answers).map(count).sum();

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
