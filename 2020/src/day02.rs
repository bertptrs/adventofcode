use std::io::BufRead;
use std::io::BufReader;
use std::io::Read;

use regex::Regex;

use crate::Solution;

fn matches1(min: usize, max: usize, c: char, sample: &str) -> bool {
    let occurrences = sample.matches(c).count();

    occurrences >= min && occurrences <= max
}

fn matches2(first: usize, second: usize, c: char, sample: &str) -> bool {
    let c = c as u8;
    let s = sample.as_bytes();

    (s[first - 1] == c) ^ (s[second - 1] == c)
}

fn read_rules<M>(input: &mut dyn Read, matcher: M) -> usize
where
    M: for<'r> Fn(usize, usize, char, &'r str) -> bool,
{
    let parser = Regex::new(r"^(\d+)-(\d+) ([a-z]): ([a-z]+)$").unwrap();

    let mut reader = BufReader::new(input);
    let mut buffer = String::new();

    let mut matching = 0;

    while let Ok(read) = reader.read_line(&mut buffer) {
        if read == 0 {
            break;
        }

        let cap = parser.captures(buffer.trim()).unwrap();

        let first = cap[1].parse().unwrap();
        let second = cap[2].parse().unwrap();
        let c = cap[3].chars().next().unwrap();
        let sample = &cap[4];

        if matcher(first, second, c, sample) {
            matching += 1
        }

        buffer.clear()
    }

    matching
}

#[derive(Default)]
pub struct Day02;

impl Solution for Day02 {
    fn part1(&mut self, input: &mut dyn Read) -> String {
        read_rules(input, matches1).to_string()
    }

    fn part2(&mut self, input: &mut dyn Read) -> String {
        read_rules(input, matches2).to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &[u8] = include_bytes!("../samples/02.txt");

    #[test]
    fn sample_part1() {
        let mut implementation = Day02;

        let result = implementation.part1(&mut SAMPLE.as_ref());
        assert_eq!("2", &result);
    }

    #[test]
    fn sample_part2() {
        let mut implementation = Day02;

        let result = implementation.part2(&mut SAMPLE.as_ref());
        assert_eq!("1", &result);
    }
}
