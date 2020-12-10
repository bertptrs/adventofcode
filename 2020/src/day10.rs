use std::io::Read;

use crate::common::from_lines;
use crate::Solution;

#[derive(Default)]
pub struct Day10;

impl Solution for Day10 {
    fn part1(&mut self, input: &mut dyn Read) -> String {
        let mut adapters: Vec<u32> = from_lines(input);
        // Outlet
        adapters.push(0);
        adapters.sort();
        let device = *adapters.last().unwrap() + 3;
        adapters.push(device);

        let mut differences = [0u32; 4];

        for window in adapters.windows(2) {
            differences[(window[1] - window[0]) as usize] += 1;
        }

        (differences[1] * differences[3]).to_string()
    }

    fn part2(&mut self, input: &mut dyn Read) -> String {
        let mut adapters: Vec<u32> = from_lines(input);
        adapters.push(0);
        adapters.sort();

        let mut methods = vec![0u64; adapters.len()];
        methods[0] = 1;

        for (i, a) in adapters.iter().copied().enumerate() {
            let c = methods[i];

            for (j, b) in adapters[i..].iter().enumerate().skip(1) {
                if b - a <= 3 {
                    methods[i + j] += c;
                } else {
                    break;
                }
            }
        }

        methods.last().unwrap().to_string()
    }
}

#[cfg(test)]
mod tests {
    use crate::test_implementation;

    use super::*;

    const SAMPLE: &[u8] = include_bytes!("../samples/10.txt");
    const SAMPLE2: &[u8] = include_bytes!("../samples/10.2.txt");

    #[test]
    fn sample_part1() {
        test_implementation!(Day10, 1, SAMPLE, 35);
        test_implementation!(Day10, 1, SAMPLE2, 220);
    }

    #[test]
    fn sample_part2() {
        test_implementation!(Day10, 2, SAMPLE, 8);
        test_implementation!(Day10, 2, SAMPLE2, 19208);
    }
}
