use std::cmp::Ordering;
use std::io::Read;

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

fn find_range(numbers: &[u64], target: u64) -> Option<(usize, usize)> {
    let mut sums = Vec::with_capacity(numbers.len() + 1);
    sums.push(0);
    sums.extend_from_slice(numbers);

    // Compute cumulative sums
    for i in 1..numbers.len() {
        sums[i] += sums[i - 1];
    }

    let mut i = 0;
    let mut j = 1;

    while i < sums.len() && j < sums.len() && numbers[j] < target {
        let current = sums[j] - sums[i];

        match current.cmp(&target) {
            Ordering::Less => j += 1,
            Ordering::Equal => return Some((i, j - 1)),
            Ordering::Greater => i += 1,
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

        let (first, last) = find_range(&numbers, target).unwrap();

        let range = &numbers[first..=last];

        let min = range.iter().min().unwrap();
        let max = range.iter().max().unwrap();

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
        let (first, last) = find_range(NUMBERS, 127).unwrap();

        assert_eq!(NUMBERS[first], 15);
        assert_eq!(NUMBERS[last], 40);
    }
}
