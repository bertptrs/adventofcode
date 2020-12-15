use std::io::Read;

use crate::common::numbers_and_stuff;
use crate::Solution;

fn nth_number(start: &[u32], n: u32) -> u32 {
    // Don't want to special case this
    assert!(n as usize >= start.len());

    let mut history = vec![0; n as usize];

    let mut prev = 0;

    for (i, &n) in start.iter().enumerate() {
        history[prev as usize] = i as u32;
        prev = n;
    }

    for i in (start.len() as u32)..n {
        let last_seen = history[prev as usize];
        history[prev as usize] = i;

        prev = if last_seen == 0 { 0 } else { i - last_seen };
    }

    prev
}

#[derive(Default)]
pub struct Day15;

impl Solution for Day15 {
    fn part1(&mut self, input: &mut dyn Read) -> String {
        let numbers: Vec<u32> = numbers_and_stuff(input);

        nth_number(&numbers, 2020).to_string()
    }

    fn part2(&mut self, input: &mut dyn Read) -> String {
        let numbers: Vec<u32> = numbers_and_stuff(input);

        nth_number(&numbers, 30000000).to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_nth_number() {
        assert_eq!(0, nth_number(&[0, 3, 6], 10));
        assert_eq!(436, nth_number(&[0, 3, 6], 2020));
        assert_eq!(1, nth_number(&[1, 3, 2], 2020));
        assert_eq!(10, nth_number(&[2, 1, 3], 2020));
        assert_eq!(27, nth_number(&[1, 2, 3], 2020));
        assert_eq!(78, nth_number(&[2, 3, 1], 2020));
        assert_eq!(438, nth_number(&[3, 2, 1], 2020));
        assert_eq!(1836, nth_number(&[3, 1, 2], 2020));
    }
}
