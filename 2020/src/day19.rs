use std::collections::HashMap;
use std::io::Read;

use regex::Regex;

use crate::common::Lines;
use crate::Solution;

fn read_input(input: &mut dyn Read) -> (HashMap<usize, String>, Vec<String>) {
    let mut lines = Lines::new(input);

    let mut rules = HashMap::new();

    while let Some(line) = lines.next() {
        if line.is_empty() {
            break;
        }

        let pos = line.find(':').expect("Invalid line");

        let n = line[..pos].parse().unwrap();
        let remainder = line[(pos + 2)..].to_owned();

        rules.insert(n, remainder);
    }

    (rules, lines.map(|s| s.to_string()).collect())
}

fn compute_regex(rules: &HashMap<usize, String>, memo: &mut [Option<String>], pos: usize) {
    if memo[pos].is_some() {
        return;
    }

    let rule = &rules[&pos];

    let mut expr = String::from("(?:");

    for part in rule.split(' ') {
        match part.chars().next().unwrap() {
            '|' => expr.push('|'),
            '"' => expr.push_str(&part[1..(part.len() - 1)]),
            c if c.is_ascii_digit() => {
                let i = part.parse().unwrap();
                compute_regex(rules, memo, i);
                expr.push_str(memo[i].as_ref().unwrap());
            }
            _ => panic!("Unknown regex bit: '{}'", part),
        }
    }

    expr.push(')');

    memo[pos] = Some(expr);
}

#[derive(Default)]
pub struct Day19;

impl Solution for Day19 {
    fn part1(&mut self, input: &mut dyn Read) -> String {
        let (raw_rules, samples) = read_input(input);
        let num_rules = raw_rules.keys().copied().max().unwrap() + 1;

        let mut memo = vec![None; num_rules];
        compute_regex(&raw_rules, &mut memo, 0);

        let expr = format!("^{}$", memo[0].as_ref().unwrap());

        let expr = Regex::new(&expr).unwrap();

        samples
            .into_iter()
            .filter(|p| expr.is_match(p))
            .count()
            .to_string()
    }

    fn part2(&mut self, input: &mut dyn Read) -> String {
        let (raw_rules, samples) = read_input(input);
        let num_rules = raw_rules.keys().copied().max().unwrap() + 1;

        let mut memo = vec![None; num_rules];
        compute_regex(&raw_rules, &mut memo, 0);

        // Rule 0 is 8 11
        // Rule 8 is 42+
        // Rule 11 is 42{n} 31{n} for arbtrary n
        // So we just need to know whether the sequence matches 42{n} 31{m} where n < m

        let r42 = memo[42].as_ref().unwrap();
        let r31 = memo[31].as_ref().unwrap();

        let r0 = format!("^({}+)({}+)$", r42, r31);

        let r0 = Regex::new(&r0).unwrap();
        let r42 = Regex::new(r42).unwrap();
        let r31 = Regex::new(r31).unwrap();

        samples
            .into_iter()
            .filter(|p| {
                if let Some(cap) = r0.captures(&p) {
                    let n = r42.find_iter(&cap[1]).count();
                    let m = r31.find_iter(&cap[2]).count();

                    n > m
                } else {
                    false
                }
            })
            .count()
            .to_string()
    }
}

#[cfg(test)]
mod tests {
    use crate::test_implementation;

    use super::*;

    const SAMPLE: &[u8] = include_bytes!("../samples/19.txt");
    const SAMPLE2: &[u8] = include_bytes!("../samples/19.2.txt");

    #[test]
    fn sample_part1() {
        test_implementation!(Day19, 1, SAMPLE, 2);
    }

    #[test]
    fn sample_part2() {
        test_implementation!(Day19, 1, SAMPLE2, 3);
        test_implementation!(Day19, 2, SAMPLE2, 12);
    }
}
