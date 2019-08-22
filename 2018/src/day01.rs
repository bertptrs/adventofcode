use std::collections::HashSet;
use std::io;
use std::io::prelude::*;

use common;

#[derive(Default)]
pub struct Day01 {}

impl Day01 {
    pub fn new() -> Day01 {
        Default::default()
    }
}

impl common::Solution for Day01 {
    fn part1(&mut self, input: &mut dyn io::Read) -> String {
        let reader = io::BufReader::new(input);

        let sum: i32 = reader
            .lines()
            .map(|x| x.unwrap().parse::<i32>().unwrap())
            .sum();

        sum.to_string()
    }

    fn part2(&mut self, input: &mut dyn io::Read) -> String {
        let reader = io::BufReader::new(input);
        let mut freqs = HashSet::new();
        freqs.insert(0);

        let mut sum = 0;
        let nums: Vec<i32> = reader
            .lines()
            .map(|x| x.unwrap().parse().unwrap())
            .collect();
        loop {
            for amount in &nums {
                sum += amount;
                if freqs.contains(&sum) {
                    return sum.to_string();
                } else {
                    freqs.insert(sum);
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use common::Solution;

    use super::*;

    #[test]
    fn samples_part1() {
        let mut instance = Day01::new();
        let sample1 = "+1\n+1\n+1";
        let sample2 = "+1\n+1\n-2";
        let sample3 = "-1\n-2\n-3";

        assert_eq!("3", instance.part1(&mut sample1.as_bytes()));
        assert_eq!("0", instance.part1(&mut sample2.as_bytes()));
        assert_eq!("-6", instance.part1(&mut sample3.as_bytes()));
    }

    #[test]
    fn samples_part2() {
        let mut instance = Day01::new();
        let sample1 = "+1\n-1";
        let sample2 = "+3\n+3\n+4\n-2\n-4";
        let sample3 = "-6\n+3\n+8\n+5\n-6";
        let sample4 = "+7\n+7\n-2\n-7\n-4";

        assert_eq!("0", instance.part2(&mut sample1.as_bytes()));
        assert_eq!("10", instance.part2(&mut sample2.as_bytes()));
        assert_eq!("5", instance.part2(&mut sample3.as_bytes()));
        assert_eq!("14", instance.part2(&mut sample4.as_bytes()));
    }
}
