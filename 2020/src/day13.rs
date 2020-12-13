use std::io::Read;

use num_integer::Integer;

use crate::common::numbers_and_stuff;
use crate::common::Lines;
use crate::Solution;

#[derive(Default)]
pub struct Day13;

impl Solution for Day13 {
    fn part1(&mut self, input: &mut dyn Read) -> String {
        let numbers: Vec<u32> = numbers_and_stuff(input);

        let departure = numbers[0];

        let (wait, id) = numbers[1..]
            .iter()
            .map(|&interval| (interval - (departure % interval), interval))
            .min()
            .unwrap();

        (wait * id).to_string()
    }

    fn part2(&mut self, input: &mut dyn Read) -> String {
        let line = Lines::new(input).nth(1).unwrap();

        let mut period = 1;
        let mut wait = 0;

        for (offset, freq) in line
            .split(',')
            .enumerate()
            .filter_map(|(i, s)| s.parse().ok().map(|n| (i as u64, n)))
        {
            let offset = (freq - (offset % freq)) % freq;

            // Modular integer division does this faster but this works.
            while wait % freq != offset {
                wait += period;
            }

            period = period.lcm(&freq);
        }

        wait.to_string()
    }
}

#[cfg(test)]
mod tests {
    use crate::test_implementation;

    use super::*;

    const SAMPLE: &[u8] = include_bytes!("../samples/13.txt");

    #[test]
    fn sample_part1() {
        test_implementation!(Day13, 1, SAMPLE, 295);
    }

    #[test]
    fn sample_part2() {
        test_implementation!(Day13, 2, SAMPLE, 1068781);
    }
}
