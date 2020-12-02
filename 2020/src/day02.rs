use std::io::BufRead;
use std::io::BufReader;
use std::io::Read;

use regex::Regex;

use crate::Solution;

struct Rule {
    c: char,
    min: usize,
    max: usize,
    sample: String,
}

impl Rule {
    fn matches1(&self) -> bool {
        let occurrences = self.sample.matches(self.c).count();

        occurrences >= self.min && occurrences <= self.max
    }

    fn matches2(&self) -> bool {
        let c = self.c as u8;
        let s = self.sample.as_bytes();

        (s[self.min - 1] == c) ^ (s[self.max - 1] == c)
    }
}

fn read_rules(input: &mut dyn Read) -> Vec<Rule> {
    let parser = Regex::new(r"^(\d+)-(\d+) ([a-z]): ([a-z]+)$").unwrap();

    let mut reader = BufReader::new(input);
    let mut buffer = String::new();

    let mut rules = Vec::new();

    while let Ok(read) = reader.read_line(&mut buffer) {
        if read == 0 {
            break;
        }

        let cap = parser.captures(buffer.trim()).unwrap();

        rules.push(Rule {
            c: cap[3].chars().next().unwrap(),
            min: cap[1].parse().unwrap(),
            max: cap[2].parse().unwrap(),
            sample: cap[4].to_owned(),
        });

        buffer.clear();
    }

    rules
}

#[derive(Default)]
pub struct Day02;

impl Solution for Day02 {
    fn part1(&mut self, input: &mut dyn Read) -> String {
        read_rules(input)
            .into_iter()
            .filter(Rule::matches1)
            .count()
            .to_string()
    }

    fn part2(&mut self, input: &mut dyn Read) -> String {
        read_rules(input)
            .into_iter()
            .filter(Rule::matches2)
            .count()
            .to_string()
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
