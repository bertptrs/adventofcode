use std::io;
use std::io::prelude::*;
use itertools::Itertools;
use common;


#[derive(Default)]
pub struct Day16 {
}

impl Day16 {
    pub fn new() -> Day16 {
        Default::default()
    }

    fn solve(&self, input: &mut io::Read, target_size: usize) -> String {
        let mut reader = io::BufReader::new(input);
        let mut line = String::new();
        reader.read_line(&mut line).expect("No input!");

        let data: Vec<bool> = line.trim().chars().map(|x| x == '1').collect();
        let filled = fill(&data, target_size);
        let sum = checksum(&filled);

        sum.iter().map(|b| if *b { '1' } else { '0' }).collect()
    }
}


impl common::Solution for Day16 {

    fn part1(&mut self, input: &mut io::Read) -> String {
        self.solve(input, 272)
    }

    fn part2(&mut self, input: &mut io::Read) -> String {
        self.solve(input, 35651584)
    }
}


fn fill(initial: &[bool], size: usize) -> Vec<bool> {
    let mut a = initial.to_vec();
    while a.len() < size {
        let mut b = a.clone();
        a.push(false);
        b.reverse();
        a.extend(b.iter().map(|x| !x));
    }

    a.truncate(size);
    a
}

fn checksum(initial: &[bool]) -> Vec<bool> {
    let mut checksum = initial.to_vec();
    while checksum.len() % 2 == 0 {
        let mut new_checksum = Vec::new();
        for (a, b) in checksum.into_iter().tuples() {
            new_checksum.push(a == b);
        }
        checksum = new_checksum;
    }
    checksum
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fill() {
        assert_eq!(vec![true, false, false, false, false,
                        false, true, true, true, true,
                        false, false, true, false, false,
                        false, false, true, true, true], fill(&[true, false, false, false, false], 20));
    }

    #[test]
    fn test_checksum() {
        let result = checksum(&[true, false, false, false, false,
                              false, true, true, true, true,
                              false, false, true, false, false,
                              false, false, true, true, true]);
        assert_eq!(vec![false, true, true, false, false], result);
    }
}
