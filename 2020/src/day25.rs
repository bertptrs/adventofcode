use std::io::Read;

use crate::common::from_lines;
use crate::Solution;

const MOD_BASE: u32 = 20201227;
const SUBJECT_NUMBER: u32 = 7;

fn loop_count(public_key: u32) -> u64 {
    let mut value = 1;
    let mut loops = 0;

    while value != public_key {
        value *= SUBJECT_NUMBER;
        value %= MOD_BASE;

        loops += 1;
    }

    loops as u64
}

#[inline]
fn mod_exp(base: u64, mut power: u64, mod_base: u64) -> u64 {
    let mut result = 1;
    let mut cur = base;

    while power > 0 {
        if power % 2 == 1 {
            result *= cur;
            result %= mod_base;
        }

        cur *= cur;
        cur %= mod_base;

        power /= 2;
    }

    result
}

#[derive(Default)]
pub struct Day25;

impl Solution for Day25 {
    fn part1(&mut self, input: &mut dyn Read) -> String {
        let nums: Vec<_> = from_lines(input);

        let exponent: u64 = nums.into_iter().map(loop_count).product();
        let result = mod_exp(SUBJECT_NUMBER as u64, exponent, MOD_BASE as u64);

        result.to_string()
    }
}

#[cfg(test)]
mod tests {
    use crate::test_implementation;

    use super::*;

    const SAMPLE: &[u8] = include_bytes!("../samples/25.txt");

    #[test]
    fn test_loop_count() {
        assert_eq!(8, loop_count(5764801));
        assert_eq!(11, loop_count(17807724));
    }

    #[test]
    fn sample_part1() {
        test_implementation!(Day25, 1, SAMPLE, 14897079);
    }
}
