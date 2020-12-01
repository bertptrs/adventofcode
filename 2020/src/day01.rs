use std::collections::HashSet;
use std::io::Read;

use crate::common::from_lines;
use crate::Solution;

#[derive(Default)]
pub struct Day01;

impl Solution for Day01 {
    fn part1(&mut self, input: &mut dyn Read) -> String {
        let expenses: HashSet<u32> = from_lines(input);
        let target = 2020;

        for &expense in expenses.iter() {
            let partner = target - expense;
            if expenses.contains(&partner) {
                return (expense * partner).to_string();
            }
        }

        panic!("No solution found!")
    }

    fn part2(&mut self, input: &mut dyn Read) -> String {
        let expenses: HashSet<u32> = from_lines(input);
        let target = 2020;

        for &e1 in expenses.iter() {
            for &e2 in expenses.iter() {
                let c = e1 + e2;
                if c > target {
                    continue;
                }

                let e3 = target - e1 - e2;

                if expenses.contains(&e3) {
                    return (e3 * e1 * e2).to_string();
                }
            }
        }

        panic!("No solution found!")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &[u8] = include_bytes!("../samples/01.txt");

    #[test]
    fn sample_part1() {
        let mut day = Day01::default();
        let result = day.part1(&mut SAMPLE.as_ref());
        assert_eq!("514579", &result);
    }

    #[test]
    fn sample_part2() {
        let mut day = Day01::default();
        let result = day.part2(&mut SAMPLE.as_ref());
        assert_eq!("241861950", &result);
    }
}
