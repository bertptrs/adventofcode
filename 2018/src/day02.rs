use std::collections::HashMap;
use std::io;
use std::io::prelude::*;

use common;
use common::GroupingCount;

/// Count the occurrence characters in a string.
fn count_chars(word: &str) -> HashMap<char, usize> {
    word.chars().grouping_count()
}

/// Compute the number of different positions between two strings.
fn distance(a: &str, b: &str) -> usize {
    let mut dist = 0;

    for (a, b) in a.chars().zip(b.chars()) {
        if a != b {
            dist += 1;
        }
    }

    dist
}

#[derive(Default)]
pub struct Day02 {}

impl Day02 {
    pub fn new() -> Day02 {
        Default::default()
    }
}

impl common::Solution for Day02 {
    fn part1(&mut self, input: &mut io::Read) -> String {
        let reader = io::BufReader::new(input);
        let mut twos = 0;
        let mut threes = 0;

        for line in reader.lines() {
            let counts = count_chars(&line.unwrap());

            if counts.values().any(|&x| x == 2) {
                twos += 1;
            }

            if counts.values().any(|&x| x == 3) {
                threes += 1;
            }
        }

        (twos * threes).to_string()
    }

    fn part2(&mut self, input: &mut io::Read) -> String {
        let mut ids: Vec<String> = io::BufReader::new(input)
            .lines()
            .map(|x| x.unwrap()).collect();
        ids.sort_unstable();

        for id1 in &ids {
            for id2 in &ids {
                if id2 > id1 {
                    break;
                }

                if distance(id1, id2) == 1 {
                    let mut answer = String::new();
                    for (a, b) in id1.chars().zip(id2.chars()) {
                        if a == b {
                            answer.push(a);
                        }
                    }
                    return answer;
                }
            }
        }
        unreachable!("Input does not contain a valid solution.");
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_count_letters() {
        let result = count_chars("abcaba");

        assert_eq!(3, result[&'a']);
        assert_eq!(2, result[&'b']);
        assert_eq!(1, result[&'c']);
    }

    #[test]
    fn test_distance() {
        assert_eq!(2, distance("abcde", "axcye"));
        assert_eq!(1, distance("fghij", "fguij"));
    }
}
