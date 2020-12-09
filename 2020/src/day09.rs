use std::cmp::Ordering;
use std::io::Read;

use itertools::Itertools;

use crate::common::from_lines;
use crate::Solution;

#[derive(Default)]
pub struct Day09;

fn is_possible(numbers: &[u64], number: u64) -> bool {
    for (i, &n) in numbers.iter().enumerate() {
        for m in &numbers[(i + 1)..] {
            if number == n + m {
                return true;
            }
        }
    }

    false
}

fn find_missing(numbers: &[u64], size: usize) -> Option<u64> {
    for window in numbers.windows(size + 1) {
        if !is_possible(&window[..size], window[size]) {
            return Some(window[size]);
        }
    }

    None
}

fn find_range(numbers: &[u64], target: u64) -> Option<&[u64]> {
    let mut i = 0;
    let mut j = 0;
    let mut current = 0;

    while i < numbers.len() && j < numbers.len() {
        match current.cmp(&target) {
            Ordering::Less => {
                current += numbers[j];
                j += 1
            }
            Ordering::Equal => return Some(&numbers[i..j]),
            Ordering::Greater => {
                current -= numbers[i];
                i += 1
            }
        }
    }

    None
}

impl Solution for Day09 {
    fn part1(&mut self, input: &mut dyn Read) -> String {
        let numbers: Vec<u64> = from_lines(input);

        find_missing(&numbers, 25).unwrap().to_string()
    }

    fn part2(&mut self, input: &mut dyn Read) -> String {
        let numbers: Vec<u64> = from_lines(input);

        let target = find_missing(&numbers, 25).unwrap();

        let range = find_range(&numbers, target).unwrap();

        let (min, max) = range.iter().minmax().into_option().unwrap();

        (min + max).to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const NUMBERS: &[u64] = &[
        35, 20, 15, 25, 47, 40, 62, 55, 65, 95, 102, 117, 150, 182, 127, 219, 299, 277, 309, 576,
    ];

    #[test]
    fn sample_part1() {
        assert_eq!(Some(127), find_missing(NUMBERS, 5));
    }

    #[test]
    fn sample_part2() {
        let range = find_range(NUMBERS, 127).unwrap();

        assert_eq!(*range.first().unwrap(), 15);
        assert_eq!(*range.last().unwrap(), 40);
    }
}
